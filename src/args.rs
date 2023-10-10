use clap::{Parser};


#[derive(Debug, PartialEq, Parser)]

pub struct Args{
    #[arg(short, long)]
    pub check: bool,
    #[arg(short='f', long, default_value_t)]
    pub config_file: String,
    #[arg(short, long)]
    pub auth: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            config_file: "/etc/tgwbot/tgwbot.yaml".to_string(),
            check: false,
            auth: false,
        }
    }
}





