mod client;
mod report;
mod sensors;
mod server;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Mode to run in.
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand, Debug)]
enum Mode {
    Server,
    Client,
}

fn main() {
    let args = Args::parse();
    match args.mode {
        Some(Mode::Server) => server::run(),
        Some(Mode::Client) => client::run(),
        None => println!("No mode specified."),
    }
}
