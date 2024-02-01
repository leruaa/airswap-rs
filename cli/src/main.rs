use clap::Parser;
use dotenv::dotenv;

use crate::cli::{BoxedAction, Cli};

mod actions;
mod cli;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    BoxedAction::from(&cli).execute().await.unwrap();
}
