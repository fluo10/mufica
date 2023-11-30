pub mod args;
pub mod config;
pub mod errors;
pub mod backend;
pub mod history;
pub mod worker;


use args::Args;
use config::{Config, FrontendConfig, BackendConfig, };
use errors::Result;
use backend::Backend;
use history::{PlainHistory, PlainHistories, MutexHistory, MutexHistories};
use worker::Worker;


use std::fs;
use std::time::Duration;
use clap::Parser;
use std::thread;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args: Args = Args::parse();

    let path = &args.config_file;
    let data = fs::read_to_string(path).unwrap();

    // Load config file
    let config: Config = serde_yaml::from_str(&data).unwrap();
    if args.check {
        println!("Config Ok");
        return Ok(());
    }

    let workers: Vec<Worker> = Vec::new();

    // Try Initialize for each frontends and bachend
    todo!();

    if args.show_history {
        for worker in workers.into_iter() {
            thread::spawn( move || {
                worker.sync_once();
            });
        }

        // Get all histories
        todo!();
        return Ok(());
    }

    for worker in workers.into_iter() {
        thread::spawn( move || {
            worker.sync();
        });
    }
    Ok(())


}
