mod args;
mod config;
mod errors;
mod frontend;
mod backend;
mod history;
mod worker;


pub use args::Args;
pub use config::Config;
pub use errors::{Result, Error};
pub use frontend::{FrontendConfig, MatrixConfig, MatrixWorker, MatrixHistory};
pub use backend::{BackendConfig, Backend, TextGenerationWebuiConfig, TextGenerationWebui};
pub use history::{History, Histories, LocalHistory, LocalHistories};
pub use worker::Worker;


use std::fs;
use std::time::Duration;
use clap::Parser;
use std::thread;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::parse();

    let path = &args.config_file;
    let data = fs::read_to_string(path).unwrap();

    // Load config file
    let config: Config = serde_yaml::from_str(&data).unwrap();
    if args.check {
        println!("Config Ok");
        return Ok(());
    }

    let services: Vec<Worker> = Vec::new();

    // Try Initialize for each frontends and bachend
    todo!();

    for service in services {
        thread::spawn( move || {
            todo!();
        });
    }
    Ok(())
    

}
