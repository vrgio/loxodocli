use clap::{Parser, Subcommand};

mod config;

use crate::config::Config;

#[derive(Parser)]
#[command(name = "loxodocli")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        instance: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { instance }) => {
            println!("{}",&instance);
        }
        None => {
            println!("Run: loxodocli init --instance <INSTANCE>");
        }
    }
}
