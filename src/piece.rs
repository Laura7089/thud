use crate::Player;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Piece {
    Dwarf,
    Troll,
    Thudstone,
    Empty,
}

impl Piece {
    pub fn allegiance(&self) -> Option<Player> {
        match self {
            Self::Dwarf => Some(Player::Dwarf),
            Self::Troll => Some(Player::Troll),
            Self::Thudstone => None,
            Self::Empty => None,
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self::Empty
    }
}
