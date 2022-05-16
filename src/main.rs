use std::process;
use clap::Parser;

use deepsearch::*;

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

