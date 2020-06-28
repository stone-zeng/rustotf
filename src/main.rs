use rustotf;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match rustotf::read_font(&args[1]) {
            Err(e) => {
                eprintln!("Application error: {}", e);
                process::exit(1);
            },
            _ => (),
        }
    }
}
