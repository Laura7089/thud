use crate::coord::Coord;
use crate::direction::Direction;
use crate::piece::Piece;
use crate::ThudError;

/// Represents the positioning of the Thud [`Piece`s](enum.Piece.html) on the board
///
/// **Note**: `Board` is not aware of the whole state of the game, only the position of the pieces.
/// As a result, the movement methods provided only perform checks according to the pieces on the
/// board, but they will *not* check whether the move is valid in terms of turn progress - you
/// should use the methods on [`Thud`](struct.Thud.html) for that.
#[derive(Debug, Default)]
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

    /// Return a vector of all the [`Coord`s](struct.Coord.html) of squares occupied by the given piece type.
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

    /// Get a vector of valid [`Coord`s](struct.Coord.html) in the 8 possible adjacent squares to the one given.
    ///
    /// Coordinates out of board bounds will not be included.
    pub fn get_adjacent(&self, square: Coord) -> Vec<(Coord, Piece)> {
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
            .get_adjacent(target)
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
        if troll.diff(target).max() > troll_len {
            // Move is too far
            return Err(ThudError::LineTooShort);
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
    /// - square `target` is not either [`Piece::Empty`](enum.Piece.html) or [`Piece::Troll`](enum.Piece.html)
    ///
    /// Returns [`Err(ThudError::Obstacle)`](enum.ThudError.html) if there is a piece in the way.
    ///
    /// Returns [`Err(ThudError::LineTooShort)`](enum.ThudError.html) if the distance to the target
    /// square is larger than the length of the line of dwarves going in the other direction
    pub fn dwarf_hurl(&mut self, dwarf: Coord, target: Coord) -> MoveResult {
        // We now need to check if the target is *either* a troll or empty
        if self.get(dwarf) != Piece::Dwarf
            || self.get(target) != Piece::Troll
            || self.get(target) != Piece::Empty
        {
            return Err(ThudError::IllegalMove);
        }
        self.verify_clear(dwarf, target)?;

        // Make sure there are enough supporting dwarves
        if self.count_line(
            dwarf,
            Direction::from_route(target, dwarf).unwrap(),
            Piece::Dwarf,
        ) < dwarf.diff(target).max()
        {
            return Err(ThudError::LineTooShort);
        }

        self.place(dwarf, Piece::Empty);
        self.place(target, Piece::Dwarf);

        Ok(())
    }

    fn cast(&self, loc: Coord, dir: Direction) -> Vec<(Coord, Piece)> {
        let mut coord = loc;
        let mut result: Vec<(Coord, Piece)> = Vec::new();

        while let Ok(next) = dir.modify(coord) {
            result.push((next, self.get(next)));
            coord = next;
        }
        result
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
                return Err(ThudError::Obstacle);
            }
        }

        Ok(())
    }

    fn count_line(&self, start: Coord, dir: Direction, piece: Piece) -> usize {
        if self.get(start) != piece {
            return 0;
        }

        let mut length = 1;
        for (_, cur_piece) in dbg!(self.cast(start, dir)) {
            if cur_piece != piece {
                break;
            }
            length += 1;
        }
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case((8,3), Piece::Troll)]
    fn place_piece(loc: (usize, usize), piece: Piece) {
        let mut board = Board::default();
        let coord = loc.into();
        board.place(coord, piece);
        assert_eq!(board.get(coord), piece);
    }

    // thudstone
    #[test_case((7, 7) => Piece::Thudstone)]
    // trolls
    #[test_case((8, 7) => Piece::Troll)]
    #[test_case((8, 8) => Piece::Troll)]
    #[test_case((6, 6) => Piece::Troll)]
    // dwarves
    #[test_case((0, 5) => Piece::Dwarf)]
    fn fresh_correct(loc: (usize, usize)) -> Piece {
        Board::fresh().get(loc.into())
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
    #[test_case((8, 8), (10, 10) => panics "")]
    #[test_case((8, 8), (7, 7) => panics "")]
    fn troll_move(src: (usize, usize), dest: (usize, usize)) {
        Board::fresh()
            .troll_move(src.into(), dest.into())
            .expect("");
    }

    #[test_case((7, 6), Direction::Up => 8)]
    #[test_case((5, 0), Direction::Down => 0)]
    #[test_case((3, 6), Direction::UpLeft => 3)]
    #[test_case((14, 9), Direction::DownLeft => 9)]
    fn cast_len(loc: (usize, usize), dir: Direction) -> usize {
        dbg!(Board::default().cast(loc.into(), dir)).len()
    }

    #[test]
    fn cast_alone() {
        let mut board = Board::default();
        board.place((7, 7).into(), Piece::Thudstone);

        let cast = board.cast((7, 6).into(), Direction::Up);

        assert_eq!(cast[0], ((7, 7).into(), Piece::Thudstone));

        let mut current = 2;
        for (next, piece) in &cast[1..] {
            assert_eq!(*piece, Piece::Empty);
            assert_eq!(*next, (7, 6 + current).into());
            current += 1;
        }
    }

    #[test_case((6, 7), (0, 7))]
    #[test_case((6, 6), (3, 3))]
    #[test_case((8, 0), (13, 5))]
    #[test_case((7, 7), (0, 7) => panics "")]
    fn verify_clear(src: (usize, usize), dest: (usize, usize)) {
        Board::fresh()
            .verify_clear(src.into(), dest.into())
            .expect("")
    }

    #[test_case((5, 0), Direction::UpLeft, Piece::Dwarf => 6)]
    #[test_case((5, 0), Direction::Up, Piece::Dwarf => 1)]
    #[test_case((6, 6), Direction::Up, Piece::Troll => 3)]
    #[test_case((7, 6), Direction::Right, Piece::Troll => 2)]
    fn count_line(loc: (usize, usize), dir: Direction, piece: Piece) -> usize {
        Board::fresh().count_line(loc.into(), dir, piece)
    }
}
