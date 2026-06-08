use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "text-marker", version, about = "Toggle text marks and highlight matches in Zed via LSP diagnostics")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run the LSP server over stdio
    Serve,
    /// Toggle a mark for the given text
    Toggle { text: String },
    /// Remove all marks
    Clear,
    /// Set up Zed tasks and the marks directory
    Install,
}
