use rustotf;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if let Err(e) = rustotf::read_font(&args[1]) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
