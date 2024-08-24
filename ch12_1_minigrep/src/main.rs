use ch12_1_minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing config, error: {err}");
            process::exit(1)
        });

    println!("Searching for {} in file {}", config.query, config.file_path);
    println!("Case insensitive?: {}", config.ignore_case);

    if let Err(e) = ch12_1_minigrep::run(config) {
        eprintln!("Run failed: {e}");
        process::exit(1)
    }
}
