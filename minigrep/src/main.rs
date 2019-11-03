use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for '{}' in file '{}'",
        config.pattern, config.filename
    );
    if let Err(e) = minigrep::run(config) {
        eprintln!("App error: {}", e);
        process::exit(1);
    }
}
