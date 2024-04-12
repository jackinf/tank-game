use crate::common::constants::RawGrid;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileHelpers;

impl FileHelpers {
    pub fn read_map_from_file(map_path: &str) -> RawGrid {
        let map_file = File::open(map_path).unwrap();
        let reader = BufReader::new(map_file);

        let mut tilemap: RawGrid = vec![];
        for line_result in reader.lines() {
            if let Err(_) = line_result {
                continue;
            }
            let line = line_result.unwrap();
            if line.is_empty() {
                continue;
            }

            let cells: Vec<usize> = line
                .split(' ')
                .map(|letter| letter.parse::<usize>().unwrap())
                .collect();
            tilemap.push(cells.clone());
        }
        tilemap
    }
}
