use clap::{Parser, Subcommand};

mod auth;
mod config;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { instance }) => {
            let oauth = auth::run_oauth(instance).await?;

            let config = config::Config {
                url: instance.clone(),
                access_token: oauth.access_token,
            };

            config.save()?;
            println!("Config ok, run loxodocli");
        }
        None => {
            println!("Run: loxodocli init --instance <INSTANCE>");
        }
    }

    Ok(())
}
