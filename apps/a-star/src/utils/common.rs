#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct Vec2(pub usize, pub usize);

pub fn find_coord(grid: &Vec<Vec<usize>>, cell_type: usize) -> Result<Vec2, String> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == cell_type {
                return Ok(Vec2(i, j));
            }
        }
    }

    Err("Not Found".into())
}