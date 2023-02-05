mod client;
mod report;
mod sensors;
mod server;

#[macro_use]
extern crate rocket;

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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let args = Args::parse();
    match args.mode {
        Some(Mode::Server) => server::run().await,
        Some(Mode::Client) => client::run().await,
        None => {
            println!("No mode specified. Exiting.");
            Ok(())
        }
    }
}
