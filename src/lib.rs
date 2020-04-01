mod board;
mod coord;
mod direction;
mod piece;

pub use board::Board;
pub use coord::Coord;
pub use direction::Direction;
pub use piece::Piece;

/// Represents one of the two Thud players
#[derive(Debug, Copy, Clone)]
pub enum Player {
    Dwarf,
    Troll,
}

/// Reports invalid action
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ThudError {
    /// A coordinate was intialised with a position out of bounds
    InvalidPosition,
    /// The requested move is not allowed according to the rules of Thud
    IllegalMove,
    /// An arithmetic error
    MathError,
}

/// Stores the current state of a game of Thud
pub struct ThudState {
    pub board: Board,
    turn: Player,
}

impl ThudState {
    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn winner(&self) -> Option<Player> {
        None
    }
}
