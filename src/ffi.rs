use crate::state::Thud;
use crate::Coord;
use crate::Direction;
use crate::EndState;
use crate::Player;
use libc::c_uint;

#[no_mangle]
pub extern "C" fn thud_new() -> *mut Thud {
    Box::into_raw(Box::new(Thud::new()))
}

#[no_mangle]
pub unsafe extern "C" fn thud_move(
    thud_raw: *mut Thud,
    src_raw: *const Coord,
    dest_raw: *const Coord,
) -> c_uint {
    let thud = &mut *thud_raw;
    let (src, dest) = (*src_raw, *dest_raw);
    match thud.move_piece(src, dest) {
        Ok(_) => 0,
        _ => 1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn thud_attack(
    thud_raw: *mut Thud,
    src_raw: *const Coord,
    dest_raw: *const Coord,
) -> c_uint {
    let thud = &mut *thud_raw;
    let (src, dest) = (*src_raw, *dest_raw);
    match thud.attack(src, dest) {
        Ok(_) => 0,
        _ => 1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn thud_get_turn(thud_raw: *const Thud) -> c_uint {
    let thud = &*thud_raw;
    match thud.turn() {
        Some(Player::Dwarf) => 1,
        Some(Player::Troll) => 2,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn thud_get_winner(thud_raw: *mut Thud) -> c_uint {
    let thud = &mut *thud_raw;
    match thud.winner() {
        Some(EndState::Won(Player::Dwarf)) => 1,
        Some(EndState::Won(Player::Troll)) => 2,
        Some(EndState::Draw) => 3,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn thud_get_score(thud_raw: *const Thud) -> [c_uint; 2] {
    let thud = &*thud_raw;
    let (dwarf, troll) = thud.score();
    [dwarf as u32, troll as u32]
}

#[no_mangle]
pub unsafe extern "C" fn thud_troll_cap(
    thud_raw: *mut Thud,
    src_raw: *const Coord,
    targets_raw: *const [c_uint; 8],
) -> c_uint {
    let thud = &mut *thud_raw;
    let targets = &*targets_raw;
    let mut attack_dirs: Vec<Direction> = Vec::new();
    for i in 0..8 {
        if targets[i] == 1 {
            attack_dirs.push(match Direction::from_num(i) {
                Ok(dir) => dir,
                _ => return 1,
            });
        }
    }
    match thud.troll_cap(*src_raw, attack_dirs) {
        Ok(_) => 0,
        _ => 1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn thud_get_board(thud_raw: *const Thud) -> [[c_uint; 15]; 15] {
    let thud = &*thud_raw;
    let board = thud.board().full_raw();
    let mut result = [[0; 15]; 15];
    for x in 0..15 {
        for y in 0..15 {
            result[x][y] = board[x][y].into_int() as c_uint;
        }
    }
    result
}
