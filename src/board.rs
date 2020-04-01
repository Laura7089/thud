use crate::coord::Coord;
use crate::direction::Direction;
use crate::piece::Piece;
use crate::ThudError;

/// A struct representing the positioning of the Thud [`Pieces`](enum.Piece.html) on the board.
#[derive(Debug, Default)]
pub struct Board {
    // 1-based indexing
    squares: [[Piece; 15]; 15],
}

impl Board {
    fn cast(&self, loc: Coord, dir: Direction) -> Vec<(Coord, Piece)> {
        let modifier = dir.modifier();
        let (x, y) = loc.value();
        let mut current = 0;

        let mut result: Vec<(Coord, Piece)> = Vec::new();

        while let Ok(coord) = Coord::zero_based(
            match modifier.0 {
                1 => x + current,
                -1 => x - current,
                _ => x,
            },
            match modifier.1 {
                1 => y + current,
                -1 => y - current,
                _ => y,
            },
        ) {
            result.push((coord, self.get(coord)));
            current += 1;
        }
        result
    }

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

    pub fn move_troll(&mut self, troll: Coord, target: Coord) -> Result<(), ThudError> {
        // Check the target is clear and the place we're moving from actually has a troll
        if self.get(troll) != Piece::Troll || self.get(target) != Piece::Empty {
            return Err(ThudError::IllegalMove);
        };

        // Validate the move, ie. one space between them
        if troll.diff(target).max() != 1 {
            return Err(ThudError::IllegalMove);
        }

        self.place(troll, Piece::Empty);
        self.place(target, Piece::Troll);

        Ok(())
    }

    pub fn move_dwarf(&mut self, dwarf: Coord, target: Coord) -> Result<(), ThudError> {
        if self.get(dwarf) != Piece::Dwarf || self.get(target) != Piece::Empty || dwarf == target {
            return Err(ThudError::IllegalMove);
        }

        // Raycast to check for obstacles
        let cast;
        if let Ok(dir) = Direction::obtain(dwarf, target) {
            cast = self.cast(dwarf, dir);
        } else {
            return Err(ThudError::IllegalMove);
        }

        for (_, piece) in cast {
            if piece != Piece::Empty {
                return Err(ThudError::IllegalMove);
            }
        }

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

    #[test_case(0, 7)]
    #[test_case(7, 0)]
    #[test_case(7, 14)]
    #[test_case(14, 7)]
    fn adjacent_safe(x: usize, y: usize) {
        Board::default().get_adjacent((x, y).into());
    }

    #[test_case((8, 7), (9, 7))]
    #[test_case((8, 8), (9, 9))]
    #[test_case((8, 8), (10, 10) => panics "no")]
    fn move_troll(src: (usize, usize), dest: (usize, usize)) {
        Board::fresh()
            .move_troll(src.into(), dest.into())
            .expect("no");
    }
}
