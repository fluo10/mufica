mod args;
mod config;
mod errors;
mod frontend;
mod backend;
mod history;
mod service;

pub use args::Args;
pub use config::Config;
pub use errors::{Result, Error};
pub use history::{Message, PairedMessages, History, TimestampedMessage, PairedTimestampedMessages, TimestampedHistory};
pub use service::{Service, ServiceExt, FrontendServiceExt, BackendServiceExt};
pub use frontend::{FrontendConfig, FrontendService, MatrixConfig, MatrixService};
pub use backend::{BackendConfig, BackendService, TextGenerationWebuiConfig, TextGenerationWebuiService};


use std::fs;
use std::thread;
use std::time::Duration;
use clap::Parser;

fn main() {
    let args: Args = Args::parse();

    let path = &args.config_file;
    let data = fs::read_to_string(path).unwrap();

    // Load config file
    let config: Config = serde_yaml::from_str(&data).unwrap();
    if args.check {
        println!("Config Ok");
        return;
    }

    let services: Vec<Service> = Vec::new();
    let frontend_services: Vec<FrontendService> = Vec::new();

    // Try authentication for each frontends and bachend
    if args.auth {
        for service in services.iter(){
            if service.needs_auth() {
                println!("Token: {}", service.try_auth().unwrap()) 
            }
        }
        return;
    }

    
    loop {
        for frontend_service in frontend_services.iter() {
            //if frontend. {
            //    
            //    let history = for frontends.get_history().join().collect();
            //    let response = backend.request(history);
               // frontend.reply(respoponse);
            //}
        }

        thread::sleep(Duration::new(config.global.interval,0))
    }

}
