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
    if args.daemon {
        for (timeline, mut subscriber) in config.to_client().unwrap().subscribe().await.unwrap().into_iter() {
            subscriber.sync(|x| {
                println!("{:?}", x);
                Ok(x.to_string())
            }).await;
        }
    }
    Ok(())
}


