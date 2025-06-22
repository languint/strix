use clap::{Parser, ValueEnum, arg};
use serde::Serialize;

#[derive(Parser)]
pub struct Cli {
    pub cmd: Cmd,
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Cmd {
    Eval,
}
