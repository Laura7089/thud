use crate::{Board, Coord, Direction, Piece, ThudError};

pub struct RayCast<'a> {
    board: &'a Board,
    next_place: Result<Coord, ThudError>,
    dir: Direction,
}

impl<'a> RayCast<'a> {
    pub fn new(board: &'a Board, start: Coord, dir: Direction) -> Self {
        RayCast {
            board,
            next_place: dir.modify(start),
            dir,
        }
    }
}

impl<'a> Iterator for RayCast<'a> {
    type Item = (Coord, Piece);
    fn next(&mut self) -> Option<(Coord, Piece)> {
        if let Ok(coord) = self.next_place {
            self.next_place = self.dir.modify(coord);
            Some((coord, self.board.get(coord)))
        } else {
            None
        }
    }
}
