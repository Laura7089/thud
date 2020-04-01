mod board;
mod coord;
mod direction;
mod piece;

pub use board::Board;
pub use coord::Coord;
pub use piece::Piece;

/// An enum representing one of the two players.
#[derive(Debug, Copy, Clone)]
pub enum Player {
    Dwarf,
    Troll,
}

/// An enum to represent when something has gone wrong in the game.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ThudError {
    /// A coordinate was intialised with a position out of bounds.
    InvalidPosition,
    /// The requested move is not allowed according to the rules of Thud.
    IllegalMove,
    /// An arithmetic error.
    MathError,
}

/// A struct storing the current state of a game of Thud.
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
