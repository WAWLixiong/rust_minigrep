use std::env;
use std::process;

mod lib;

// use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    // if let Err(e) = minigrep::run(config) {
    if let Err(e) = lib::run(config) {
        eprintln!("Application err:{}", e);
        process::exit(1);
    }
}

