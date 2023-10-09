mod args;
mod config;

pub use args::Args;
pub use config::Config;

use std::fs;
use clap::Parser;

fn main() {
    let args: Args = Args::parse();

    if args.check {
        let path = &args.config_file;
        let data = fs::read_to_string(path).unwrap();
        let config: Config = serde_yaml::from_str(&data).unwrap();
    }
}
