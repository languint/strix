mod cli;

use anyhow::Result;
use chess::{Color, Game};
use clap::Parser;
use owo_colors::OwoColorize;

use crate::cli::{Cli, Cmd, print_info};

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.cmd {
        Cmd::Eval { depth } => {
            print_info("Running".green(), args.cmd.to_string().as_str(), 3);

            let mut game = Game::new();

            let (score, best_move) =
                strix_eval::eval::alpha_beta(&mut game, &depth, i64::MAX, -i64::MAX, Color::White);

            print_info("Score".cyan(), score.to_string().as_str(), 4);
            if let Some(best_move) = best_move {
                print_info("Best Move".cyan(), best_move.to_string().as_str(), 4);
            }
        }
    }

    Ok(())
}
