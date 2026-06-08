mod cli;

use clap::Parser;
use cli::{Cli, Command};
use text_marker::commands;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Serve => commands::serve::run(),
        Command::Toggle { text } => commands::toggle::run(&text),
        Command::Clear => commands::clear::run(),
        Command::Install => commands::install::run(),
    }
}
