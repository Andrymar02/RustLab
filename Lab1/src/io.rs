use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn process_csv(filename: &str, head_n: usize) -> Result<(), io::Error> {
    let file = File::open(filename)?; // Gestisce file inesistente o permessi
    let reader = BufReader::new(file);

    let mut lines_count = 0;
    let mut head_lines = Vec::new(); 

    for line in reader.lines() {
        let line_content = line?; // Gestione errore di lettura riga
        if lines_count < head_n {
            head_lines.push(line_content);
        }
        lines_count += 1;
    }

    println!("rows: {}", lines_count);
    println!("head ({}):", head_n);
    for line in head_lines {
        println!("{}", line);
    }

    Ok(())
}