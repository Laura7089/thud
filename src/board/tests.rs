use super::*;
use test_case::test_case;

// TODO write tests for:
// - troll_capture
// - score
// - winner
// - available_moves

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
    Board::default().adjacent((x, y).into()).len()
}

#[test_case((8, 7), (9, 7))]
#[test_case((8, 8), (9, 9))]
#[test_case((8, 8), (10, 10) => panics "")]
#[test_case((8, 8), (7, 7) => panics "")]
fn troll_move(src: (usize, usize), dest: (usize, usize)) {
    let mut board = Board::fresh();
    board.troll_move(src.into(), dest.into()).expect("");
    assert_eq!(board.get(src.into()), Piece::Empty);
    assert_eq!(board.get(dest.into()), Piece::Troll);
}

#[test_case(vec![(3, 6), (4, 6), (5, 6)], (8, 6), (13, 6))]
#[test_case(vec![(6, 5)], (8, 7), (11, 10))]
#[test_case(vec![], (8, 6), (13, 6) => panics "")]
#[test_case(vec![(3, 6), (4, 6), (5, 6)], (8, 6), (12, 6) => panics "")]
fn troll_shove(pre_places: Vec<(usize, usize)>, src: (usize, usize), dest: (usize, usize)) {
    let mut board = Board::fresh();
    for place in pre_places {
        board.place(place.into(), Piece::Troll);
    }
    board.troll_shove(src.into(), dest.into()).expect("");
    assert_eq!(board.get(src.into()), Piece::Empty);
    assert_eq!(board.get(dest.into()), Piece::Troll);
}

#[test_case((6, 0), (6, 5))]
#[test_case((6, 0), (10, 4))]
#[test_case((4, 13), (12, 5))]
#[test_case((8, 0), (8, 12) => panics "")]
#[test_case((1, 4), (0, 5) => panics "")]
fn dwarf_move(src: (usize, usize), dest: (usize, usize)) {
    let mut board = Board::fresh();
    board.dwarf_move(src.into(), dest.into()).expect("");
    assert_eq!(board.get(src.into()), Piece::Empty);
    assert_eq!(board.get(dest.into()), Piece::Dwarf);
}

#[test_case(vec![(6, 1), (6, 2), (6, 3)], (6, 3), (6, 6))]
#[test_case(vec![(3, 10), (4, 9)], (4, 9), (6, 7))]
fn dwarf_hurl(pre_places: Vec<(usize, usize)>, src: (usize, usize), dest: (usize, usize)) {
    let mut board = Board::fresh();
    for place in pre_places {
        board.place(place.into(), Piece::Dwarf);
    }
    board.dwarf_hurl(src.into(), dest.into()).expect("");
    assert_eq!(board.get(src.into()), Piece::Empty);
    assert_eq!(board.get(dest.into()), Piece::Dwarf);
}

#[test_case((7, 6), Direction::Up => 8)]
#[test_case((5, 0), Direction::Down => 0)]
#[test_case((3, 6), Direction::UpLeft => 3)]
#[test_case((14, 9), Direction::DownLeft => 9)]
fn cast_len(loc: (usize, usize), dir: Direction) -> usize {
    dbg!(Board::default()
        .cast(loc.into(), dir)
        .collect::<Vec<(Coord, Piece)>>())
    .len()
}

#[test]
fn cast_alone() {
    let mut board = Board::default();
    board.place((7, 7).into(), Piece::Thudstone);

    let mut cast = board.cast((7, 6).into(), Direction::Up);

    assert_eq!(cast.next(), Some(((7, 7).into(), Piece::Thudstone)));

    let mut current = 2;
    for (next, piece) in cast {
        assert_eq!(piece, Piece::Empty);
        assert_eq!(next, (7, 6 + current).into());
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
