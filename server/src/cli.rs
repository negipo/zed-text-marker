use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "text-marker", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// LSPサーバーをstdioで起動する
    Serve,
    /// 選択文字列のマークをトグルする
    Toggle { text: String },
    /// 全マークを消す
    Clear,
}
