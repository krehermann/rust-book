use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args[1..]).unwrap_or_else(|err| {
        eprintln!("Error parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
