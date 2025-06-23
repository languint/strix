use std::fmt;

use clap::{Parser, Subcommand, arg};
use owo_colors::OwoColorize;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Cmd {
    Eval { depth: u8 },
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn print_info<T: fmt::Display>(head: T, msg: &str, depth: u8) {
    println!("{}{} {}", " ".repeat(depth as usize), head.bold(), msg);
}
