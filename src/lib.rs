//! `thud` is a crate for hosting a game of
//! [Thud](https://www.boardgamegeek.com/boardgame/4532/thud).
//!
//! ## Getting Started
//!
//! To get started, take a look at the [`Thud`](struct.Thud.html) `struct`.
//!
//! ## Extra Features
//!
//! ### serialize
//!
//! The library supports serialising and deserialising all types using
//! [`serde`](https://serde.rs/).

mod board;
mod coord;
mod direction;
#[cfg(ffi)]
mod ffi;
mod piece;
mod state;

pub use board::Board;
pub use coord::Coord;
pub use direction::Direction;
pub use piece::Piece;
#[cfg(serialize)]
use serde::{Deserialize, Serialize};
pub use state::Thud;

/// One of the two Thud players
#[cfg_attr(feature = "serialise", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    Dwarf,
    Troll,
}

/// What victory condition a [`Thud`](struct.Thud.html) game is in once it has ended
#[cfg_attr(feature = "serialise", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EndState {
    Won(Player),
    Draw,
}

/// Reports invalid action
#[cfg_attr(feature = "serialise", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ThudError {
    /// A coordinate was intialised with a position out of bounds
    InvalidPosition,
    /// The requested move is not allowed according to the rules of Thud
    IllegalMove,
    /// There is a piece blocking that move
    Obstacle,
    /// A shove or hurl has been attempted with too few supporting dwarves/trolls
    LineTooShort,
    /// An arithmetic error
    MathError,
    /// That action is not allowed at this time
    BadAction,
}
