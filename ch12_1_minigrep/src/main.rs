use ch12_1_minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing config, error: {err}");
            process::exit(1)
        });

    println!("Searching for {} in file {}", config.query, config.file_path);

    if let Err(e) = ch12_1_minigrep::run(config) {
        println!("Run failed: {e}");
        process::exit(1)
    }
}
