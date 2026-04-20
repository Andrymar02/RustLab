use std::env;

pub struct Config {
    pub filename: String,
    pub head_count: usize,
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, String> {
        args.next(); // Salta il nome del programma [cite: 14]

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Errore: Argomenti mancanti. Specificare un file CSV.".to_string()),
        };

        let mut head_count = 10; // Default [cite: 27]

        while let Some(arg) = args.next() {
            if arg == "--head" {
                match args.next() {
                    Some(val) => {
                        head_count = val.parse::<usize>().map_err(|_| {
                            "Errore: --head deve essere un intero positivo.".to_string()
                        })?;
                    }
                    None => return Err("Errore: --head richiede un valore numerico.".to_string()),
                }
            }
        }

        Ok(Config { filename, head_count })
    }
}