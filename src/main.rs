mod auth;
mod api;
mod cli;
mod utils;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(e) = cli::run(cli).await {
        eprintln!("{}", colored::Colorize::red(format!("Error: {e}").as_str()));
        std::process::exit(1);
    }
}
