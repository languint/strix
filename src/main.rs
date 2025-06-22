mod cli;

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::{Cli, Cmd};

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.cmd {
        Cmd::Eval => {}
    }

    Ok(())
}
