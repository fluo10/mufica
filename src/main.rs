pub mod args;
pub mod config;
pub mod errors;
pub mod backend;
pub mod history;
pub mod subscriber;


use args::Args;
use config::{Config, FrontendConfig, BackendConfig, };
use errors::Result;
use backend::Backend;
use history::{PlainHistory, PlainHistories, MutexHistory, MutexHistories};
use subscriber::Subscriber;


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

    let mut subscribers: Vec<Subscriber> = Vec::new();

    // Try Initialize for each frontends and bachend
    for frontend_config in config.frontends.iter() {
        for subscriber in frontend_config.to_subscribers()?.into_iter() {
            subscribers.push(subscriber.into());
        }

    }

    if args.show_history {
        for subscriber in subscribers.into_iter() {
            thread::spawn( move || {
                subscriber.sync_once();
            });
        }

        // Get all histories
        todo!();
        return Ok(());
    }

    for subscriber in subscribers.into_iter() {
        thread::spawn( move || {
            subscriber.sync();
        });
    }
    Ok(())


}
