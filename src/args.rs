use clap::{Parser};


#[derive(Debug, PartialEq, Parser)]
pub struct Args{
    #[arg(short, long)]
    check: bool,
    #[arg(short='f', long)]
    config_file: String,
}





