use clap::{Parser, Subcommand};

/// Program to run rust tutorials
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    #[clap(default_value_t = String::from("3D"), long, short, action)]
    pub render: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {}
