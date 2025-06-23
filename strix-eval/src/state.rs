use chess::{BitBoard, Board, Color, Square};

use crate::eval;

pub fn get_material_for_color(board: &Board, color: Color) -> i32 {
    let mut material = 0;
    for sq in board.color_combined(color).collect::<Vec<Square>>() {
        if let Some(piece) = board.piece_on(sq) {
            material += eval::get_piece_value(piece);
        }
    }

    material
}

// Returns the material for each color.
#[inline]
pub fn get_materials(board: &Board) -> (i32, i32) {
    let white_material = get_material_for_color(board, Color::White);
    let black_material = get_material_for_color(board, Color::Black);

    (white_material, black_material)
}
