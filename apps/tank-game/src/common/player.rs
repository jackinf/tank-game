#[derive(Clone)]
pub enum Player {
    P1 = 1,
    P2 = 2,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Player::P1, Player::P1) => true,
            (Player::P2, Player::P2) => true,
            _ => false,
        }
    }
}
