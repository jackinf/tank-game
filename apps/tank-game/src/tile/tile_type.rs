use std::convert::TryFrom;

#[derive(Clone)]
pub enum TileType {
    Grass = 0,
    Gold = 1,
    Wall = 2,
    Water = 3,
}

impl TryFrom<usize> for TileType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TileType::Grass),
            1 => Ok(TileType::Gold),
            2 => Ok(TileType::Wall),
            3 => Ok(TileType::Water),
            _ => Err(()),
        }
    }
}

impl TileType {
    pub fn get_tile_type_sprite(&self) -> String {
        match self {
            TileType::Wall => "sprites/tiles/wall_b.png".into(),
            TileType::Grass => "sprites/tiles/grass_b.png".into(),
            TileType::Gold => "sprites/tiles/gold.png".into(),
            TileType::Water => "sprites/tiles/water.png".into(),
        }
    }

    pub fn get_tile_type_layer(&self) -> f32 {
        match self {
            TileType::Grass => 0.,
            TileType::Gold => 5.,
            TileType::Wall => 5.,
            TileType::Water => 5.,
        }
    }
}
