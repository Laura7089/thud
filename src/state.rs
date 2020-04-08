use crate::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Stores the current state of a game of Thud
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Thud {
    board: Board,
    state: GameState,
}

#[derive(PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
enum GameState {
    Nominal(Player),
    PostTrollMove(bool),
    GameEnded(EndState),
}

impl Thud {
    /// Get a `Thud` ready to be played!
    pub fn new() -> Self {
        Thud {
            board: Board::fresh(),
            state: GameState::Nominal(Player::Dwarf),
        }
    }

    /// Find which player's turn it is.
    ///
    /// Will return `Some(Player)` if the game is still in progress, or if the game is ended
    /// `None` will be returned.
    pub fn turn(&self) -> Option<Player> {
        match self.state {
            GameState::Nominal(p) => Some(p),
            GameState::PostTrollMove(_) => Some(Player::Troll),
            GameState::GameEnded(_) => None,
        }
    }

    /// Wrapper for [`Board::winner()`](struct.Board.html#method.winner)
    pub fn winner(&mut self) -> Option<EndState> {
        // Note: checks cached game state, otherwise runs Board::winner()
        match self.state {
            GameState::GameEnded(p) => Some(p),
            _ => match self.board.winner() {
                Some(p) => {
                    self.state = GameState::GameEnded(p);
                    Some(p)
                }
                None => None,
            },
        }
    }

    /// Wrapper for [`Board::score()`](struct.Board.html#method.score)
    pub fn score(&self) -> (usize, usize) {
        self.board.score()
    }

    /// Move a piece of the player whose turn it is
    ///
    /// On a Dwarf turn, the turn will automatically tick over, on a Troll turn, the player may
    /// make a capture afterward with [`.troll_cap()`](#method.troll_cap).
    /// Should the troll player not wish to make a capture, they may call
    /// [`.troll_cap()`](#method.troll_cap) with an empty `Vec`.
    ///
    /// Will pass errors from [`Board.dwarf_move()`](struct.Board.html#method.dwarf_move)
    /// and [`Board.troll_move()`](struct.Board.html#method.troll_move).
    pub fn move_piece(&mut self, src: Coord, target: Coord) -> Result<(), ThudError> {
        match self.state {
            // If it's the dwarf player, move the dwarf and end the turn
            GameState::Nominal(Player::Dwarf) => {
                self.board.dwarf_move(src, target)?;
                self.state = GameState::Nominal(Player::Troll);
                Ok(())
            }
            // If it's the troll player, move the troll and enter GameState::PostTrollMove
            GameState::Nominal(Player::Troll) => {
                self.board.troll_move(src, target)?;
                self.state = GameState::PostTrollMove(false);
                Ok(())
            }
            // Otherwise we can't move
            _ => Err(ThudError::BadAction),
        }
    }

    /// Attack with a piece of the player whose turn it is
    ///
    /// This can only be taken as the first action of the player's turn, otherwise
    /// [`Err(ThudError::BadAction)`](enum.ThudError.html) will be returned.
    ///
    /// On a Dwarf turn, it will automatically tick over to the next turn, on a Troll turn, *at
    /// least one capture* must be made afterward with [`.troll_cap()`](#method.troll_cap).
    ///
    /// Will pass errors from [`Board.dwarf_hurl()` and `Board.troll_shove()`](struct.Board.html).
    pub fn attack(&mut self, src: Coord, target: Coord) -> Result<(), ThudError> {
        match self.state {
            // If it's the dwarf player's turn, perform the hurl and end the turn
            GameState::Nominal(Player::Dwarf) => {
                self.board.dwarf_hurl(src, target)?;
                self.state = GameState::Nominal(Player::Troll);
                Ok(())
            }
            // If it's the troll player's turn, perform the shove and enter
            // GameState::PostTrollMove with the shove flag set
            GameState::Nominal(Player::Troll) => {
                self.board.troll_shove(src, target)?;
                self.state = GameState::PostTrollMove(true);
                Ok(())
            }
            _ => Err(ThudError::BadAction),
        }
    }

    /// Capture a number of dwarves with a troll
    ///
    /// This may only be called after a move or a shove/attack on a troll player's turn.
    ///
    /// If the previous action was a shove/attack then `targets` *must contain at least 1 valid
    /// dwarf to take*, otherwise [`Err(ThudError::IllegalMove)`](enum.ThudError.html) will be
    /// returned and the method must be called again before play can continue.
    ///
    /// Otherwise, the turn will be ticked over automatically.
    pub fn troll_cap(&mut self, troll: Coord, targets: Vec<Direction>) -> Result<(), ThudError> {
        match self.state {
            // If this is after a shove, perform the move then ensure at least 1 dwarf was taken
            // (error if not) then end the turn
            GameState::PostTrollMove(true) => {
                let captured = self.board.troll_capture(troll, targets)?;
                if captured == 0 {
                    Err(ThudError::IllegalMove)
                } else {
                    self.state = GameState::Nominal(Player::Dwarf);
                    Ok(())
                }
            }
            // If this is after a move, perform the move then end the turn
            GameState::PostTrollMove(false) => {
                self.board.troll_capture(troll, targets)?;
                self.state = GameState::Nominal(Player::Dwarf);
                Ok(())
            }
            _ => Err(ThudError::BadAction),
        }
    }
}
