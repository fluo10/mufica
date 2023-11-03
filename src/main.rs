pub mod args;
pub mod config;
pub mod errors;
pub mod backend;
pub mod history;
pub mod worker;


pub use args::Args;
pub use config::{Config, FrontendConfig, BackendConfig, MatrixConfig, TextGenerationWebuiConfig, };
pub use errors::{Result, Error};
pub use backend::{Backend, TextGenerationWebuiBackend};
pub use history::{PlainHistory, PlainHistories, MutexHistory, MutexHistories, MatrixHistory, TextGenerationWebuiHistory};
pub use worker::{Worker, MatrixWorker};


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
