mod cli;

use clap::Parser;
use cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Serve => unimplemented!("task 5"),
        Command::Toggle { .. } => unimplemented!("task 4"),
        Command::Clear => unimplemented!("task 4"),
    }
}
