pub enum UnitType {
    Tank = 1,
    Soldier = 2,
    Harvester = 3,
}

impl TryFrom<usize> for UnitType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UnitType::Tank),
            2 => Ok(UnitType::Soldier),
            3 => Ok(UnitType::Harvester),
            _ => Err(()),
        }
    }
}
