"""Slice contact-sheet decal renders into individual game-ready sprites.

For each input sheet: flood-fill the flat background to transparent (reusing the
cutbg approach), split the remaining opaque pixels into connected components,
drop the ones that look like sparse grass tufts / tiny specks, then crop, recolour
(desaturate + slight darken so arid rocks sit on green grass) and save each as its
own RGBA PNG.

    python tools/slice_decals.py OUT_DIR SHEET1.png [SHEET2.png ...]
"""

import sys
from collections import deque

import numpy as np
from PIL import Image
from scipy import ndimage

TOL = 60.0          # background colour distance
MIN_AREA = 1400     # ignore specks (px, at source resolution)
MAX_FRAC = 0.45     # ignore blobs covering >45% of the sheet (merged background)
MIN_FILL = 0.20     # ignore sparse/spiky blobs (grass tufts) by bbox fill ratio
SAT_KEEP = 0.42     # desaturation: keep this fraction of original saturation
DARKEN = 0.86


def bg_alpha(arr):
    h, w, _ = arr.shape
    border = np.concatenate([arr[0], arr[-1], arr[:, 0], arr[:, -1]], axis=0)
    bg = np.median(border, axis=0)
    dist = np.sqrt(((arr - bg) ** 2).sum(2))
    bglike = dist < TOL
    visited = np.zeros((h, w), bool)
    dq = deque()
    for x in range(w):
        for y in (0, h - 1):
            if bglike[y, x] and not visited[y, x]:
                visited[y, x] = True
                dq.append((y, x))
    for y in range(h):
        for x in (0, w - 1):
            if bglike[y, x] and not visited[y, x]:
                visited[y, x] = True
                dq.append((y, x))
    while dq:
        y, x = dq.popleft()
        for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            ny, nx = y + dy, x + dx
            if 0 <= ny < h and 0 <= nx < w and not visited[ny, nx] and bglike[ny, nx]:
                visited[ny, nx] = True
                dq.append((ny, nx))
    alpha = np.where(visited, 0.0, 255.0).astype(np.float32)
    return alpha


def recolour(rgb):
    lum = (0.3 * rgb[..., 0] + 0.59 * rgb[..., 1] + 0.11 * rgb[..., 2])[..., None]
    out = (lum + (rgb - lum) * SAT_KEEP) * DARKEN
    return np.clip(out, 0, 255)


def main():
    out_dir = sys.argv[1]
    sheets = sys.argv[2:]
    idx = 0
    for sheet in sheets:
        im = Image.open(sheet).convert("RGB")
        arr = np.asarray(im).astype(np.float32)
        h, w, _ = arr.shape
        alpha = bg_alpha(arr)
        mask = alpha > 40
        labels, n = ndimage.label(mask)
        for lab in range(1, n + 1):
            ys, xs = np.where(labels == lab)
            area = len(ys)
            if area < MIN_AREA or area > MAX_FRAC * h * w:
                continue
            y0, y1, x0, x1 = ys.min(), ys.max() + 1, xs.min(), xs.max() + 1
            fill = area / ((y1 - y0) * (x1 - x0))
            if fill < MIN_FILL:
                continue  # sparse / spiky -> probably a grass tuft
            sub_rgb = recolour(arr[y0:y1, x0:x1].copy())
            sub_a = np.where(labels[y0:y1, x0:x1] == lab, alpha[y0:y1, x0:x1], 0.0)
            out = np.dstack([sub_rgb, sub_a]).astype(np.uint8)
            Image.fromarray(out, "RGBA").save(f"{out_dir}/decal_{idx:02d}.png")
            idx += 1
    print(f"wrote {idx} decals to {out_dir}")


if __name__ == "__main__":
    main()
