"""Remove a flat background from a sprite render and write an RGBA PNG.

Flood-fills from the image edges through pixels close to the border colour, so
only the background that is actually connected to the edge is erased. Look-alike
pixels inside the artwork survive. A short feather softens the cut edge.

Usage:
    python tools/cutbg.py <src.png> <dst.png>

Run it through the Makefile to get the venv set up automatically:
    make cutbg IN=path/to/src.png OUT=apps/tank-game/assets/buildings/foo.png
"""

import sys
from collections import deque

import numpy as np
from PIL import Image

TOL = 60.0      # colour distance from background still counted as "background"
FEATHER = 22.0  # soft edge width just outside TOL


def main():
    if len(sys.argv) != 3:
        print("Usage: python tools/cutbg.py <src.png> <dst.png>", file=sys.stderr)
        sys.exit(1)
    src, dst = sys.argv[1], sys.argv[2]

    im = Image.open(src).convert("RGB")
    arr = np.asarray(im).astype(np.int16)
    h, w, _ = arr.shape

    # Background colour = median of the four border lines (robust to specks).
    border = np.concatenate(
        [arr[0, :, :], arr[-1, :, :], arr[:, 0, :], arr[:, -1, :]], axis=0
    )
    bg = np.median(border, axis=0)

    dist = np.sqrt(((arr - bg) ** 2).sum(axis=2))
    bglike = dist < TOL

    # Flood fill from every border pixel through the background-like mask.
    visited = np.zeros((h, w), dtype=bool)
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

    # Alpha: 0 over the erased background, soft ramp on the pixels bordering it.
    alpha = np.full((h, w), 255, dtype=np.float32)
    alpha[visited] = 0.0
    adj = np.zeros((h, w), dtype=bool)
    adj[1:, :] |= visited[:-1, :]
    adj[:-1, :] |= visited[1:, :]
    adj[:, 1:] |= visited[:, :-1]
    adj[:, :-1] |= visited[:, 1:]
    fz = (~visited) & (dist < TOL + FEATHER) & adj
    alpha[fz] = np.clip((dist[fz] - TOL) / FEATHER, 0, 1) * 255.0

    out = np.dstack([arr.astype(np.uint8), alpha.astype(np.uint8)])
    Image.fromarray(out, "RGBA").save(dst)

    cleared = int(visited.sum())
    print(f"{w}x{h}, cleared {cleared} px ({100 * cleared / (w * h):.1f}%) -> {dst}")


if __name__ == "__main__":
    main()
