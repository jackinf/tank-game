use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let tilemap = generate_tilemap(100, 100);
    write_tilemap_to_file("apps/tank-game/assets/map3.txt", &tilemap);
    write_tilemap_to_file("apps/tank-game/assets/map3_p1_units.txt", &tilemap);
    write_tilemap_to_file("apps/tank-game/assets/map3_p2_units.txt", &tilemap);
}

fn generate_tilemap(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut map = vec![vec![0; width]; height];
    for y in 0..height {
        for x in 0..width {
            let tile = match rand::random::<f64>() {
                r if r < 0.7 => 0,
                r if r < 0.75 => 1,
                r if r < 0.95 => 2,
                _ => 3,
            };
            map[y][x] = tile;
        }
    }
    map
}

fn write_tilemap_to_file(filename: &str, tilemap: &[Vec<usize>]) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for row in tilemap {
        let line = row
            .iter()
            .map(|tile| tile.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}
