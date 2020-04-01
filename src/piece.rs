// use crate::Player;

/// Represents a piece on the Thud board
///
/// **Note**: Empty squares are modelled as `Piece`s too, to avoid the horror of `Option<Piece>`
/// everywhere.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Piece {
    Dwarf,
    Troll,
    Thudstone,
    Empty,
}

// impl Piece {
//     pub fn allegiance(&self) -> Option<Player> {
//         match self {
//             Self::Dwarf => Some(Player::Dwarf),
//             Self::Troll => Some(Player::Troll),
//             Self::Thudstone => None,
//             Self::Empty => None,
//         }
//     }
// }

impl Default for Piece {
    fn default() -> Self {
        Self::Empty
    }
}
