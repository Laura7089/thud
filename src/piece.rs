#[cfg(serialize)]
use serde::{Deserialize, Serialize};

/// A piece on the Thud [`Board`](struct.Board.html)
///
/// **Note**: Empty squares are modelled as `Piece`s too, to avoid the horror of `Option<Piece>`
/// everywhere.
#[cfg_attr(feature = "serialise", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Piece {
    Dwarf,
    Troll,
    Thudstone,
    Empty,
}

impl Default for Piece {
    fn default() -> Self {
        Self::Empty
    }
}
