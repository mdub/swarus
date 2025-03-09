use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, Error};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// S3 operations
    S3 {
        #[command(subcommand)]
        command: S3Commands,
    },
}

#[derive(Subcommand)]
enum S3Commands {
    /// List buckets
    Buckets,
    /// Show bucket details
    Bucket {
        /// Name of the bucket
        name: String,
    },
}

async fn list_buckets(client: &Client) -> Result<(), Error> {
    let req = client.list_buckets();
    let resp = req.send().await?;
    let buckets = resp.buckets.unwrap_or_default();
    for b in buckets.iter() {
        if let Some(name) = &b.name {
            println!("{}", name);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let shared_config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = Client::new(&shared_config);
    match cli.command {
        Commands::S3 { command } => match command {
            S3Commands::Buckets => {
                list_buckets(&client).await?;
            }
            S3Commands::Bucket { name } => {
                println!("Bucket details for: {}", name);
                // TODO: Implement bucket details
            }
        },
    }

    Ok(())
}
