use clap::Parser;
use std::path::PathBuf;
use super::verify_path;

#[derive(Debug, Parser)] 
pub enum HttpSubCommand {
    #[ command( about = "Serve a directory over HTTP")]
    Serve(HettpServeOpts),
}


#[derive(Debug, Parser)]
pub struct HettpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,

    #[arg(long, default_value = "8080")]
    pub port: u16,
}

