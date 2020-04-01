use crate::coord::Coord;
use crate::direction::Direction;
use crate::piece::Piece;
use crate::ThudError;

/// A struct representing the positioning of the Thud [`Pieces`](enum.Piece.html) on the board.
///
/// **Note**: `Board` is not aware of the whole state of the game, only the position of the pieces.
/// As a result, the movement methods provided only perform checks according to the pieces on the
/// board, but they will *not* check whether the move is valid in terms of turn progress - you
/// should use the methods on [`ThudState`](struct.ThudState.html) for that.
#[derive(Debug, Default)]
pub struct Board {
    // 1-based indexing
    squares: [[Piece; 15]; 15],
}

type MoveResult = Result<(), ThudError>;

impl Board {
    /// Get a fresh `Board`, with [`Piece`](enum.Piece.html)s placed in the default positions for thud.
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
                Box::new(|num| (num + 5, num)),
                Box::new(|num| (num, num + 5)),
                Box::new(|num| (num + 5, 5 - num)),
            ];
            for calc in calcs {
                for dwarf in (0..5).map(|seed| calc(seed).into()) {
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

    /// Put a `Piece` on the board.
    pub fn place(&mut self, square: Coord, piece: Piece) {
        let (x, y) = square.value();
        self.squares[x][y] = piece;
    }

    /// Find what `Piece` is at the coordinate specified.
    pub fn get(&self, square: Coord) -> Piece {
        let (x, y) = square.value();
        self.squares[x][y]
    }

    /// Return a vector of all the coordinates occupied by the given piece type.
    ///
    /// ```
    /// use thud::{Board, Piece, Coord};
    ///
    /// let board = Board::fresh();
    /// let stone = board.get_army(Piece::Thudstone);
    ///
    /// assert_eq!(stone[0].value(), (7, 7));
    /// ```
    pub fn get_army(&self, piece_type: Piece) -> Vec<Coord> {
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

    /// Get a vector of valid coordinates in the 8 possible adjacent squares to the one given.
    ///
    /// Coordinates out of board bounds will not be included.
    pub fn get_adjacent(&self, square: Coord) -> Vec<Coord> {
        let mut adjacent: Vec<Coord> = Vec::with_capacity(8);
        let (x, y) = square.value();

        // Horrific nested if's to avoid addressing outside arrays
        for x_off in (if x != 0 { x - 1 } else { x })..(if x != 14 { x } else { x + 1 }) {
            for y_off in (if y != 0 { y - 1 } else { y })..(if y != 14 { y } else { y + 1 }) {
                if let Ok(coord) = Coord::zero_based(x + x_off, y + y_off) {
                    adjacent.push(coord);
                }
            }
        }
        adjacent
    }

    fn cast(&self, loc: Coord, dir: Direction) -> Vec<(Coord, Piece)> {
        let mut current = dir.modify(loc);
        let mut result: Vec<(Coord, Piece)> = Vec::new();

        while let Ok(coord) = Coord::zero_based(current.0, current.1) {
            result.push((coord, self.get(coord)));
            current = dir.modify(coord);
        }
        result
    }

    fn verify_clear(&self, src: Coord, dest: Coord) -> MoveResult {
        // Raycast to check for obstacles
        if let Ok(dir) = Direction::obtain(src, dest) {
            let cast = self.cast(src, dir);
            // Skip the first member of the list, which is the square with the src on
            for (current, piece) in &cast[1..] {
                if *piece != Piece::Empty {
                    // There is something in the way
                    return Err(ThudError::IllegalMove);
                }
                if *current == dest {
                    // Stop at the target square
                    break;
                }
            }
        } else {
            // The source and target are not on a straight line
            return Err(ThudError::IllegalMove);
        }

        Ok(())
    }

    fn count_line(&self, start: Coord, dir: Direction, piece: Piece) -> usize {
        let mut length = 0;
        let cast = self.cast(start, dir);

        for (_, cur_piece) in cast {
            if cur_piece != piece {
                break;
            }
            length += 1;
        }

        length
    }

    /// Move a troll.
    ///
    /// Returns a `ThudError::IllegalMove` if:
    ///
    /// - The troll square is not `Piece::Troll`
    /// - The target square is not `Piece::Empty`
    /// - The target square is more than 1 squares away from the troll square
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
    /// Returns a `ThudError::IllegalMove` if:
    ///
    /// - The troll square is not `Piece::Troll`
    /// - The target square is not `Piece::Empty`
    /// - The target square is obstructed
    /// - The distance to the target square is larger than the length of the lines of trolls going
    /// in the other direction
    pub fn troll_shove(&mut self, troll: Coord, target: Coord) -> MoveResult {
        if (self.get(troll), self.get(target)) != (Piece::Troll, Piece::Empty) {
            return Err(ThudError::IllegalMove);
        }
        self.verify_clear(troll, target)?;

        let troll_len = self.count_line(
            troll,
            // unwrap because `self.verify_clear` would return an error if we weren't in a straight line
            Direction::obtain(troll, target).unwrap().opposite(),
            Piece::Troll,
        );
        if troll.diff(target).max() > troll_len {
            // Move is too far
            return Err(ThudError::IllegalMove);
        }

        // Move the troll
        self.place(troll, Piece::Empty);
        self.place(target, Piece::Troll);

        Ok(())
    }

    pub fn troll_capture(&mut self, troll: Coord, targets: Vec<Direction>) -> MoveResult {
        if self.get(troll) != Piece::Troll {
            return Err(ThudError::IllegalMove);
        }

        // Grab all the true coordinates from `targets`, returning an error if any are invalid
        for target in targets.into_iter() {
            let target_raw = target.modify(troll);
            if let Ok(coord) = Coord::zero_based(target_raw.0, target_raw.1) {
                if self.get(coord) == Piece::Dwarf {
                    self.place(coord, Piece::Empty);
                }
            }
        }

        Ok(())
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn place_piece() {
        let mut board = Board::default();
        let loc: Coord = (8, 8).into();
        board.place(loc, Piece::Dwarf);
        assert_eq!(board.get(loc), Piece::Dwarf);
    }

    #[test]
    fn place_pieces() {
        let mut board = Board::default();
        let locs = (
            Coord::zero_based(9, 9).unwrap(),
            Coord::zero_based(10, 10).unwrap(),
        );
        board.place(locs.0, Piece::Dwarf);
        board.place(locs.1, Piece::Troll);
        assert_eq!(board.get(locs.0), Piece::Dwarf);
        assert_eq!(board.get(locs.1), Piece::Troll);
    }

    #[test]
    fn fresh_correct() {
        let board = Board::fresh();

        // thudstone
        assert_eq!(board.get((7, 7).into()), Piece::Thudstone);

        // trolls
        assert_eq!(board.get((8, 7).into()), Piece::Troll);
        assert_eq!(board.get((8, 7).into()), Piece::Troll);

        // dwarves
        assert_eq!(board.get((0, 5).into()), Piece::Dwarf);
    }

    #[test_case(0, 7 => 5)]
    #[test_case(7, 0 => 5)]
    #[test_case(7, 14 => 5)]
    #[test_case(14, 7 => 5)]
    fn adjacent(x: usize, y: usize) -> usize {
        Board::default().get_adjacent((x, y).into()).len()
    }

    #[test_case((8, 7), (9, 7))]
    #[test_case((8, 8), (9, 9))]
    #[test_case((8, 8), (10, 10) => panics "no")]
    #[test_case((8, 8), (7, 7) => panics "no")]
    fn troll_move(src: (usize, usize), dest: (usize, usize)) {
        Board::fresh()
            .troll_move(src.into(), dest.into())
            .expect("no");
    }
}
