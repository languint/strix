use chess::{ChessMove, Color, Game, MoveGen, Piece};

use crate::state::get_materials;

pub type Eval = i32;
pub type AlphaBetaResult = (Eval, Option<ChessMove>);

pub fn eval_static(game: &Game, color: &Color) -> Eval {
    let (white_material, black_material) = get_materials(&game.current_position());

    let mut eval = white_material - black_material;

    if color == &Color::Black {
        eval = -eval;
    }

    eval
}

// Alpha-beta search.
// https://en.wikipedia.org/wiki/Alpha–beta_pruning
pub fn alpha_beta(
    game: &mut Game,
    depth: &u8,
    alpha: i32,
    beta: i32,
    color: Color,
) -> AlphaBetaResult {
    let mut our_alpha = alpha;
    let mut our_beta = beta;

    if *depth == 0 || node_is_terminal(game) {
        return (eval_static(game, &color), None);
    }

    let mut best_move: Option<ChessMove> = None;

    if color == Color::White {
        let mut value = -i32::MAX;
        for mv in MoveGen::new_legal(&game.current_position()) {
            let mut new_game = game.clone();
            new_game.make_move(mv);

            let (eval, _) = alpha_beta(&mut new_game, &(depth - 1), our_alpha, our_beta, !color);

            if eval > value {
                value = eval;
                best_move = Some(mv);
            }

            our_alpha = our_alpha.max(value);
            if value >= our_beta {
                break;
            }
        }

        (value, best_move)
    } else {
        let mut value = i32::MAX;
        for mv in MoveGen::new_legal(&game.current_position()) {
            let mut new_game = game.clone();
            new_game.make_move(mv);

            let (eval, _) = alpha_beta(&mut new_game, &(depth - 1), our_alpha, our_beta, !color);

            if eval < value {
                value = eval;
                best_move = Some(mv);
            }

            our_beta = our_beta.min(value);
            if value <= our_alpha {
                break;
            }
        }

        (value, best_move)
    }
}

#[inline]
pub fn node_is_terminal(game: &Game) -> bool {
    game.result().is_some()
}

// Returns the value of a piece in centipawns
#[inline]
pub fn get_piece_value(p: Piece) -> i32 {
    match p {
        Piece::Pawn => 100,
        Piece::Knight => 300,
        Piece::Bishop => 300,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 0,
    }
}
