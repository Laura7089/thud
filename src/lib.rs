//! `thud` is a crate for hosting a game of
//! [Thud](https://www.boardgamegeek.com/boardgame/4532/thud).
//!
//! ## Getting Started
//!
//! To get started, take a look at the [`Thud`](struct.Thud.html) `struct`.
//!
//! ### `serialize` feature
//!
//! The library supports serialising and deserialising all types using
//! [`serde`](https://serde.rs/) when this feature is enabled.

mod board;
mod coord;
mod direction;
mod piece;
mod state;

use thiserror::Error;

pub use board::Board;
pub use coord::Coord;
pub use direction::Direction;
pub use piece::Piece;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
pub use state::Thud;

/// One of the two Thud players
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    Dwarf,
    Troll,
}

/// What victory condition a [`Thud`](struct.Thud.html) game is in once it has ended
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EndState {
    Won(Player),
    Draw,
}

/// Reports invalid action
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone, Error)]
pub enum ThudError {
    #[error("({0},{1}) is out of bounds")]
    InvalidPosition(usize, usize),
    #[error("Requested move not allowed")]
    IllegalMove,
    #[error("A piece at ({0},{1}) is blocking that move")]
    Obstacle(usize, usize),
    #[error("You need {0} pieces behind you to make that move but you only have {1}")]
    LineTooShort(usize, usize),
    #[error("Arithmetic Error")]
    MathError,
    #[error("Action not allowed at this point in the game")]
    BadAction,
}
