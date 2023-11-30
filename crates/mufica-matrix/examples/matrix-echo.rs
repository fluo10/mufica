use mufica_matrix::{MatrixConfig, Result, MatrixClient, MatrixTimeline};
use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,

    #[arg(short='n', long)]
    dry_run: bool,

    #[arg(short, long)]
    daemon: bool,
}
#[tokio::main]
pub async fn main() -> Result<()>{
    let args = Args::parse();
    let config = MatrixConfig::read_file(&args.config).unwrap(); 
    for client in config.to_clients().unwrap().into_iter() {
        if args.daemon {
            client.sync_once().await;
        } else {
            client.sync(|x| {
                println!("{:?}", x);
                Ok(())
            }).await;

        }
    }
    Ok(())
}


