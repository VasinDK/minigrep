use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // Входные данные
    let config = Config::new(env::args()).unwrap_or_else(|_err|{
        process::exit(1);
    });

    if let Err(_e) = minigrep::run(config) {
        process::exit(1);
    };

}
