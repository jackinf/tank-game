pub struct EnumHelpers;

impl EnumHelpers {
    pub fn assert_valid_enum<T: TryFrom<usize>>(tile_type: usize) -> Option<T> {
        match T::try_from(tile_type) {
            Ok(tile_type) => Some(tile_type),
            Err(_) => None,
        }
    }
}
