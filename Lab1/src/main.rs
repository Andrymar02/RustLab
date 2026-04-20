mod cli;
mod io; 

use std::env;
use std::process;

fn main() {
    // Parsing argomenti
    let config = cli::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // Lettura file 
    if let Err(e) = io::process_csv(&config.filename, config.head_count) {
        eprintln!("Errore durante la lettura del file '{}': {}", config.filename, e);
        process::exit(1); 
    }
}