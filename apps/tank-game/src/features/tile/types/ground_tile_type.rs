use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum GroundTileType {
    Grass = 0,
    Gold = 1,
    Wall = 2,
    Water = 3,
}

impl TryFrom<i32> for GroundTileType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GroundTileType::Grass),
            1 => Ok(GroundTileType::Gold),
            2 => Ok(GroundTileType::Wall),
            3 => Ok(GroundTileType::Water),
            _ => Err(()),
        }
    }
}

impl Display for GroundTileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
