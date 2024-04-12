pub struct EnumHelpers;

impl EnumHelpers {
    pub fn assert_valid_enum<T: TryFrom<usize>>(tile_type: usize) -> T {
        match T::try_from(tile_type) {
            Ok(tile_type) => tile_type,
            Err(_) => panic!("Invalid TileType value: {}", tile_type),
        }
    }
}
