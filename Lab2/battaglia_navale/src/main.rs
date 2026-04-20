use std::env;
use std::fs;

const BSIZE: usize = 20;

#[derive(Clone)]
pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

#[derive(Debug)]
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat {
    Vertical(usize),
    Horizontal(usize),
}

impl Board {
    /// Crea una board vuota con la disponibilità di navi specificata.
    pub fn new(boats: &[u8]) -> Board {
        let mut b = [0; 4];
        for i in 0..4 {
            if i < boats.len() {
                b[i] = boats[i];
            }
        }
        // Inizializza la matrice 20x20 con il carattere spazio ' '
        Board {
            boats: b,
            data: [[b' '; BSIZE]; BSIZE],
        }
    }

    /// Crea una Board a partire dal contenuto del file (come stringa).
    pub fn from(s: String) -> Board {
        let mut lines = s.lines();
        let mut boats = [0; 4];
        
        // Legge la prima riga (navi disponibili, separate da spazio)
        if let Some(first_line) = lines.next() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            for i in 0..4 {
                if i < parts.len() {
                    boats[i] = parts[i].parse().unwrap_or(0);
                }
            }
        }

        let mut data = [[b' '; BSIZE]; BSIZE];
        // Legge le successive 20 righe
        for (r, line) in lines.enumerate() {
            if r < BSIZE {
                for (c, ch) in line.chars().enumerate() {
                    if c < BSIZE {
                        data[r][c] = ch as u8;
                    }
                }
            }
        }

        Board { boats, data }
    }

    /// Aggiunge la nave; restituisce la nuova Board o un errore.
    pub fn add_boat(mut self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {
        let (len, is_vertical) = match boat {
            Boat::Vertical(l) => (l, true),
            Boat::Horizontal(l) => (l, false),
        };

        // Verifica che la lunghezza sia valida (da 1 a 4)
        if len < 1 || len > 4 {
            return Err(Error::OutOfBounds);
        }

        // Verifica disponibilità navi
        if self.boats[len - 1] == 0 {
            return Err(Error::BoatCount);
        }

        // Gli indici da command line partono da 1, li convertiamo a 0-based
        if pos.0 == 0 || pos.1 == 0 {
            return Err(Error::OutOfBounds);
        }
        let r = pos.0 - 1;
        let c = pos.1 - 1;

        let (dr, dc) = if is_vertical { (1, 0) } else { (0, 1) };

        // Verifica bordi
        if r + dr * (len - 1) >= BSIZE || c + dc * (len - 1) >= BSIZE {
            return Err(Error::OutOfBounds);
        }

        // Verifica sovrapposizione e tocco (anche diagonale)
        for i in 0..len {
            let cr = r + dr * i;
            let cc = c + dc * i;

            // Controlliamo il "vicinato" 3x3 per ogni blocco della nave
            let r_start = cr.saturating_sub(1);
            let r_end = (cr + 1).min(BSIZE - 1);
            let c_start = cc.saturating_sub(1);
            let c_end = (cc + 1).min(BSIZE - 1);

            for rr in r_start..=r_end {
                for c_col in c_start..=c_end {
                    if self.data[rr][c_col] == b'B' {
                        return Err(Error::Overlap);
                    }
                }
            }
        }

        // Applica la nave e riduce il contatore
        for i in 0..len {
            self.data[r + dr * i][c + dc * i] = b'B';
        }
        self.boats[len - 1] -= 1;

        Ok(self)
    }

    /// Converte la board in una stringa salvabile su file.
    pub fn to_string(&self) -> String {
        let mut s = format!(
            "{} {} {} {}\n",
            self.boats[0], self.boats[1], self.boats[2], self.boats[3]
        );
        for r in 0..BSIZE {
            for c in 0..BSIZE {
                s.push(self.data[r][c] as char);
            }
            s.push('\n');
        }
        s
    }

    /// Stampa la board sul terminale in modo leggibile per il debug
    pub fn print(&self) {
        println!("\n  --- STATO DELLA BOARD ---");
        // Intestazione delle colonne (da 1 a 20)
        print!("   ");
        for c in 1..=BSIZE {
            print!("{:02} ", c);
        }
        println!();

        for r in 0..BSIZE {
            // Numero di riga (da 1 a 20)
            print!("{:02} ", r + 1); 
            for c in 0..BSIZE {
                // Sostituiamo visivamente lo spazio con un puntino
                let ch = if self.data[r][c] == b' ' { '.' } else { 'B' };
                print!(" {} ", ch);
            }
            println!();
        }
        println!("Navi disponibili: [1 casella: {}] [2 caselle: {}] [3 caselle: {}] [4 caselle: {}]\n",
            self.boats[0], self.boats[1], self.boats[2], self.boats[3]);
    }
}

// ==========================================
// Gestione della Command Line
// ==========================================
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Uso:");
        eprintln!("  cargo run -- <file.txt> new <4,3,2,1>");
        eprintln!("  cargo run -- <file.txt> add_boat <V|H>,<len>,<riga>,<colonna>");
        return;
    }

    let filename = &args[1];
    let command = &args[2];
    let params = &args[3];

    if command == "new" {
        // Esempio: 4,3,2,1
        let counts: Vec<u8> = params
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        let board = Board::new(&counts);
        
        if let Err(e) = fs::write(filename, board.to_string()) {
            eprintln!("Errore nel salvataggio del file: {}", e);
        } else {
            println!("Nuova board creata in {}", filename);
            board.print();
        }
    } else if command == "add_boat" {
        // Leggiamo la board esistente
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Errore di lettura del file: {}", e);
                return;
            }
        };
        
        let board = Board::from(content);

        // Parsing dei parametri: V,3,10,10
        let parts: Vec<&str> = params.split(',').collect();
        if parts.len() != 4 {
            eprintln!("Formato add_boat errato. Usa: V,3,10,10");
            return;
        }

        let dir = parts[0];
        let len: usize = parts[1].parse().unwrap_or(0);
        let row: usize = parts[2].parse().unwrap_or(0);
        let col: usize = parts[3].parse().unwrap_or(0);

        let boat = match dir {
            "V" => Boat::Vertical(len),
            "H" => Boat::Horizontal(len),
            _ => {
                eprintln!("Direzione non valida. Usa V o H.");
                return;
            }
        };

        // Aggiungiamo la nave
        match board.add_boat(boat, (row, col)) {
            Ok(new_board) => {
                if let Err(e) = fs::write(filename, new_board.to_string()) {
                    eprintln!("Errore nel salvataggio del file: {}", e);
                } else {
                    println!("Nave aggiunta con successo!");
                    new_board.print();
                }
            }
            Err(e) => {
                eprintln!("Errore durante l'aggiunta della nave: {:?}", e);
            }
        }
    } else {
        eprintln!("Comando sconosciuto: {}", command);
    }
}