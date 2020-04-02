mod board;
mod coord;
mod direction;
mod piece;
mod state;

pub use board::Board;
pub use coord::Coord;
pub use direction::Direction;
pub use piece::Piece;
pub use state::Thud;

/// Represents one of the two Thud players
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    Dwarf,
    Troll,
}

/// Represents what victory condition a [`Thud`](struct.Thud.html) is in once it has ended
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EndState {
    Won(Player),
    Draw,
}

/// Reports invalid action
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
