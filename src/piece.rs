#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// A piece on the Thud [`Board`](struct.Board.html)
///
/// **Note**: Empty squares are modelled as `Piece`s too, to avoid the horror of `Option<Piece>`
/// everywhere.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Piece {
    #[cfg_attr(feature = "serialize", serde(rename = "dwarf"))]
    Dwarf,
    #[cfg_attr(feature = "serialize", serde(rename = "troll"))]
    Troll,
    #[cfg_attr(feature = "serialize", serde(rename = "thudstone"))]
    Thudstone,
    #[cfg_attr(feature = "serialize", serde(rename = "empty"))]
    Empty,
}

impl Default for Piece {
    fn default() -> Self {
        Self::Empty
    }
}

#[cfg(feature = "ffi")]
impl Piece {
    pub fn into_int(&self) -> usize {
        match self {
            Piece::Empty => 0,
            Piece::Dwarf => 1,
            Piece::Troll => 2,
            Piece::Thudstone => 3,
        }
    }
}
