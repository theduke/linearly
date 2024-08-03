use clap::Parser;
use cmd::CliCommand;
use colored::Colorize;

mod cmd;
mod config;
mod render;
mod util;

#[tokio::main]
async fn main() {
    let args = cmd::Args::parse();
    if let Err(err) = args.run().await {
        eprintln!("{}: {}", "ERROR".red(), err);
        std::process::exit(1);
    }
}
