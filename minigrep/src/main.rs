use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
