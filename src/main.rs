use std::{env, process};
use minigrep::{run, Config};

fn main() {
    // get iterator of cl args and turn into a collection
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("error processing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // handle error case from enum
    // we do not want to unwrap the result in the same way because run returns ()
    // we only care about detecting an error
    if let Err(e) = run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}




