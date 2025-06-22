use std::i64;

use chess::{ChessMove, Color, Game, MoveGen};

pub type Eval = i64;
pub type AlphaBetaResult = (Eval, Option<ChessMove>);

pub fn eval_static(game: &Game, color: &Color) -> Eval {
    0
}

// Alpha-beta search.
// https://en.wikipedia.org/wiki/Alpha–beta_pruning
pub fn alpha_beta(
    game: &mut Game,
    depth: &u8,
    alpha: &mut i64,
    beta: &mut i64,
    color: &Color,
) -> AlphaBetaResult {
    let mut our_alpha = *alpha;
    let mut our_beta = *beta;

    if *depth == 0 || node_is_terminal(game) {
        return (eval_static(game, color), None);
    }

    let mut best_move: Option<ChessMove> = None;

    if *color == Color::White {
        let mut value = -i64::MAX;
        for mv in MoveGen::new_legal(&game.current_position()) {
            let mut new_game = game.clone();
            new_game.make_move(mv);

            let (eval, _) = alpha_beta(
                &mut new_game,
                &(depth - 1),
                &mut our_alpha,
                &mut our_beta,
                &!color.to_owned(),
            );

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
        let mut value = i64::MAX;
        for mv in MoveGen::new_legal(&game.current_position()) {
            let mut new_game = game.clone();
            new_game.make_move(mv);

            let (eval, _) = alpha_beta(
                &mut new_game,
                &(depth - 1),
                &mut our_alpha,
                &mut our_beta,
                &!color.to_owned(),
            );

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

pub fn node_is_terminal(game: &Game) -> bool {
    game.result().is_some()
}
