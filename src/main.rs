use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error while parsing argumnets: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("Some Error Occured while Running the program: {e}");
        process::exit(2);
    };
}
