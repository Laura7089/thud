mod raycast;
use crate::coord::Coord;
use crate::direction::Direction;
use crate::piece::Piece;
use crate::{EndState, Player, ThudError};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(test)]
mod tests;

/// A configuration of Thud [`Piece`s](enum.Piece.html) on a Thud board
///
/// **Note**: `Board` is not aware of the whole state of the game, only the position of the pieces.
/// As a result, the movement methods provided only perform checks according to the pieces on the
/// board, but they will *not* check whether the move is valid in terms of turn progress - you
/// should use the methods on [`Thud`](struct.Thud.html) for that.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Default)]
pub struct Board {
    // 1-based indexing
    squares: [[Piece; 15]; 15],
}

type MoveResult = Result<(), ThudError>;

impl Board {
    /// Get a "fresh" `Board`, with [`Piece`s](enum.Piece.html) placed in the default positions for thud.
    pub fn fresh() -> Self {
        let mut filled_board = Self::default();
        // Place the trolls
        for i in 6..9 {
            for j in 6..9 {
                filled_board.place((i, j).into(), Piece::Troll);
            }
        }
        // Place the dwarves
        {
            // Diagonals
            let calcs: Vec<Box<dyn Fn(usize) -> (usize, usize)>> = vec![
                Box::new(|num| (num, 5 - num)),
                Box::new(|num| (num + 9, num)),
                Box::new(|num| (num, num + 9)),
                Box::new(|num| (num + 9, 14 - num)),
            ];
            for calc in calcs {
                for dwarf in (0..=5).map(|seed| calc(seed).into()) {
                    filled_board.place(dwarf, Piece::Dwarf);
                }
            }

            // Extras at corners
            for dwarf in vec![
                (0, 6),
                (0, 8),
                (14, 6),
                (14, 8),
                (6, 0),
                (8, 0),
                (6, 14),
                (8, 14),
            ] {
                filled_board.place(dwarf.into(), Piece::Dwarf);
            }
        }
        // Place the thudstone
        filled_board.place((7, 7).into(), Piece::Thudstone);
        filled_board
    }

    /// Put a [`Piece`](enum.Piece.html) on the board.
    pub fn place(&mut self, square: Coord, piece: Piece) {
        let (x, y) = square.value();
        self.squares[x][y] = piece;
    }

    /// Find what [`Piece`](enum.Piece.html) is at the [`Coord`](struct.Coord.html) specified.
    pub fn get(&self, square: Coord) -> Piece {
        let (x, y) = square.value();
        self.squares[x][y]
    }

    pub fn full_raw(&self) -> [[Piece; 15]; 15] {
        self.squares
    }

    /// Return a vector of all the [`Coord`s](struct.Coord.html) of squares occupied by the given piece type.
    ///
    /// ```
    /// use thud::{Board, Piece, Coord};
    ///
    /// let board = Board::fresh();
    /// let stone = board.army(Piece::Thudstone);
    ///
    /// assert_eq!(stone[0].value(), (7, 7));
    /// ```
    pub fn army(&self, piece_type: Piece) -> Vec<Coord> {
        let mut result: Vec<Coord> = Vec::new();
        for x in 0..15 {
            for y in 0..15 {
                if self.squares[x][y] == piece_type {
                    if let Ok(coord) = Coord::zero_based(x, y) {
                        result.push(coord);
                    }
                }
            }
        }
        result
    }

    /// Get a vector of valid [`Coord`s](struct.Coord.html) in the 8 possible adjacent squares to the one given.
    ///
    /// Coordinates out of board bounds will not be included.
    pub fn adjacent(&self, square: Coord) -> Vec<(Coord, Piece)> {
        let mut adjacent: Vec<(Coord, Piece)> = Vec::with_capacity(8);

        for dir in Direction::all() {
            if let Ok(coord) = dir.modify(square) {
                adjacent.push((coord, self.get(coord)));
            }
        }
        adjacent
    }

    /// Move a troll.
    ///
    /// Returns [`Err(ThudError::IllegalMove)`](enum.ThudError.html) if:
    ///
    /// - The `troll` square is not [`Piece::Troll`](enum.Piece.html)
    /// - The `target` square is not [`Piece::Empty`](enum.Piece.html)
    /// - The `target` square is more than 1 squares away from the `troll` square
    pub fn troll_move(&mut self, troll: Coord, target: Coord) -> MoveResult {
        // Check the target is clear and the place we're moving from actually has a troll
        if (self.get(troll), self.get(target)) != (Piece::Troll, Piece::Empty) {
            return Err(ThudError::IllegalMove);
        };

        // Validate the move, ie. one space between them
        if troll.diff(target).max() != 1 {
            return Err(ThudError::IllegalMove);
        }

        // Move the troll
        self.place(troll, Piece::Empty);
        self.place(target, Piece::Troll);

        Ok(())
    }

    /// "Shove" a troll.
    ///
    /// Returns [`Err(ThudError::IllegalMove)`](enum.ThudError.html) if:
    ///
    /// - The `troll` square is not [`Piece::Troll`](enum.Piece.html)
    /// - The `target` square is not [`Piece::Empty`](enum.Piece.html)
    /// - There are no [`Piece::Dwarf`s](enum.Piece.html) adjacent to the `target` square
    ///
    /// Returns [`Err(ThudError::Obstacle)`](enum.ThudError.html) if the target square is obstructed
    ///
    /// Returns [`Err(ThudError::LineTooShort)`](enum.ThudError.html) if the distance to the target
    /// square is larger than the length of the line of trolls going in the other direction
    pub fn troll_shove(&mut self, troll: Coord, target: Coord) -> MoveResult {
        if (self.get(troll), self.get(target)) != (Piece::Troll, Piece::Empty) {
            return Err(ThudError::IllegalMove);
        }
        self.verify_clear(troll, target)?;

        let dwarves: Vec<(Coord, Piece)> = self
            .adjacent(target)
            .into_iter()
            .filter(|(_, x)| *x == Piece::Dwarf)
            .collect();
        if dwarves.len() == 0 {
            return Err(ThudError::IllegalMove);
        }

        let troll_len = self.count_line(
            troll,
            // unwrap because `self.verify_clear` would return an error if we weren't in a straight line
            Direction::from_route(target, troll).unwrap(),
            Piece::Troll,
        );
        let dist = troll.diff(target).max();
        if dist > troll_len {
            // Move is too far
            return Err(ThudError::LineTooShort(dist, troll_len));
        }

        // Move the troll
        self.place(troll, Piece::Empty);
        self.place(target, Piece::Troll);

        Ok(())
    }

    /// Use a troll to selectively capture dwarves around it.
    ///
    /// `targets` should be a `Vec` of [`Direction`s](enum.Direction.html) in which to capture; if there is a dwarf above
    /// your troll and you wish to capture it then `targets` should contain
    /// [`Direction::Up`](enum.Direction.html).
    ///
    /// Note that any invalid (out of board limits) or duplicate
    /// [`Direction`s](enum.Direction.html) will be ignored.
    ///
    /// Returns [`Err(ThudError::IllegalMove)`](enum.ThudError.html) if the piece at `troll` is not [`Piece::Troll`](enum.Piece.html).
    pub fn troll_capture(
        &mut self,
        troll: Coord,
        targets: Vec<Direction>,
    ) -> Result<usize, ThudError> {
        if self.get(troll) != Piece::Troll {
            return Err(ThudError::IllegalMove);
        }

        let mut captured = 0;

        // Grab all the true coordinates from `targets`, returning an error if any are invalid
        for target in targets.into_iter() {
            if let Ok(coord) = target.modify(troll) {
                if self.get(coord) == Piece::Dwarf {
                    self.place(coord, Piece::Empty);
                    captured += 1;
                }
            }
        }

        Ok(captured)
    }

    /// Move a dwarf.
    ///
    /// Returns [`Err(ThudError::IllegalMove)`](enum.ThudError.html) if:
    ///
    /// - square `dwarf` is not [`Piece::Dwarf`](enum.Piece.html)
    /// - square `target` is not [`Piece::Empty`](enum.Piece.html)
    ///
    /// Returns [`Err(ThudError::Obstacle)`](enum.ThudError.html) if there is a piece in the way.
    pub fn dwarf_move(&mut self, dwarf: Coord, target: Coord) -> MoveResult {
        // Check the target is clear and the place we're moving from actually has a dwarf
        if (self.get(dwarf), self.get(target)) != (Piece::Dwarf, Piece::Empty) {
            return Err(ThudError::IllegalMove);
        }
        self.verify_clear(dwarf, target)?;

        // Move the dwarf
        self.place(dwarf, Piece::Empty);
        self.place(target, Piece::Dwarf);

        Ok(())
    }

    /// "Hurl" a dwarf.
    ///
    /// Returns [`Err(ThudError::IllegalMove)`](enum.ThudError.html) if:
    ///
    /// - square `dwarf` is not [`Piece::Dwarf`](enum.Piece.html)
    /// - square `target` is not [`Piece::Troll`](enum.Piece.html)
    ///
    /// Returns [`Err(ThudError::Obstacle)`](enum.ThudError.html) if there is a piece in the way.
    ///
    /// Returns [`Err(ThudError::LineTooShort)`](enum.ThudError.html) if the distance to the target
    /// square is larger than the length of the line of dwarves going in the other direction
    pub fn dwarf_hurl(&mut self, dwarf: Coord, target: Coord) -> MoveResult {
        if self.get(dwarf) != Piece::Dwarf || self.get(target) != Piece::Troll {
            return Err(ThudError::IllegalMove);
        }
        self.verify_clear(dwarf, target)?;

        // Make sure there are enough supporting dwarves
        let dwarf_len = self.count_line(
            dwarf,
            Direction::from_route(target, dwarf).unwrap(),
            Piece::Dwarf,
        );
        let dist = dwarf.diff(target).max();
        if dwarf_len < dist {
            return Err(ThudError::LineTooShort(dist, dwarf_len));
        }

        self.place(dwarf, Piece::Empty);
        self.place(target, Piece::Dwarf);

        Ok(())
    }

    /// Get a `Vec` of [`Coord`s](struct.Coord.html) that the piece at `loc` can make
    pub fn available_moves(&self, loc: Coord) -> Vec<Coord> {
        let mut avail: Vec<Coord> = Vec::new();
        match self.get(loc) {
            Piece::Dwarf => {
                for dir in Direction::all() {
                    // Count the dwarves behind us
                    let line_behind = self.count_line(loc, dir.opposite(), Piece::Dwarf);

                    for (count, (poss, piece)) in self.cast(loc, dir).into_iter().enumerate() {
                        match piece {
                            // If it's empty, we can move into it!
                            Piece::Empty => avail.push(poss),
                            // If there's a troll there, we can take it if we're not so far out
                            // that our line of dwarves can't support us (but cannot jump over it)
                            Piece::Troll => {
                                if count <= line_behind {
                                    avail.push(poss);
                                }
                                break;
                            }
                            _ => break,
                        }
                    }
                }
            }
            Piece::Troll => {
                // Look as far as we are allowed by our line of trolls in all directions, and get
                // any empty squares we find
                for dir in Direction::all() {
                    let behind_line = self.count_line(loc, dir.opposite(), Piece::Troll);
                    let mut cast = self.cast(loc, dir);
                    cast.next();
                    for (poss, piece) in cast.take(behind_line) {
                        match piece {
                            Piece::Empty => avail.push(poss),
                            _ => break,
                        }
                    }
                }
            }
            _ => (),
        }

        avail
    }

    /// Find if there is a winner or the game is over.
    ///
    /// Returns:
    ///
    /// - [`Some(EndState::Won(Player))`](enum.EndState.html) if a player has won the match
    /// - [`Some(EndState::Draw)`](enum.EndState.html) if the match is a draw
    /// - `None` if the board still has moves to play
    pub fn winner(&self) -> Option<EndState> {
        // Check dwarves
        let mut dwarf_moves = 0;
        for dwarf in self.army(Piece::Dwarf) {
            dwarf_moves += self.available_moves(dwarf).len();
        }

        // Check trolls
        let mut troll_moves = 0;
        for troll in self.army(Piece::Troll) {
            troll_moves += self.available_moves(troll).len();
        }

        if troll_moves == 0 || dwarf_moves == 0 {
            let (dwarf_score, troll_score) = self.score();
            if dwarf_score > troll_score {
                Some(EndState::Won(Player::Dwarf))
            } else if troll_score > dwarf_score {
                Some(EndState::Won(Player::Troll))
            } else {
                Some(EndState::Draw)
            }
        } else {
            None
        }
    }

    /// Get the scores of each player
    ///
    /// Given in format `(<dwarf score>, <troll score>)`
    pub fn score(&self) -> (usize, usize) {
        let dwarves = self.army(Piece::Dwarf).len();
        let trolls = self.army(Piece::Troll).len() * 4;
        (dwarves, trolls)
    }

    fn cast(&self, loc: Coord, dir: Direction) -> raycast::RayCast {
        raycast::RayCast::new(self, loc, dir)
    }

    fn verify_clear(&self, src: Coord, dest: Coord) -> MoveResult {
        let dir = Direction::from_route(src, dest)?;
        // Skip the first element
        for (current, piece) in self.cast(src, dir) {
            if current == dest {
                // Stop at the target square
                break;
            }
            if piece != Piece::Empty {
                // There is something in the way
                let (x, y) = current.value();
                return Err(ThudError::Obstacle(x, y));
            }
        }

        Ok(())
    }

    fn count_line(&self, start: Coord, dir: Direction, piece: Piece) -> usize {
        if self.get(start) != piece {
            return 0;
        }

        let mut length = 1;
        for (_, cur_piece) in self.cast(start, dir) {
            if cur_piece != piece {
                break;
            }
            length += 1;
        }
        length
    }
}
