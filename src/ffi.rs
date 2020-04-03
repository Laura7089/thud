use thud::state::Thud;
use thud::ThudError;

#[no_mangle]
pub extern "C" fn thud_new() -> *mut Thud {
    Box::into_raw(Box::new(Thud::new()))
}

#[no_mangle]
pub extern "C" fn thud_move(thud: *mut Thud, src: *const Coord, dest: *const Coord) -> *const u8 {
    match thud.move_piece(src, dest) {
        Err(ThudError::*) => *1,
        _ => *0,
    }
}
