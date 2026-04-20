use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

// ==========================================
// Passo 1: Lettura e scrittura di file
// ==========================================
pub fn passo1() {
    // Usiamo match per gestire gli errori senza panic [cite: 136]
    match fs::read_to_string("test.txt") {
        Ok(content) => {
            let repeated = content.repeat(10); 
            if let Err(e) = fs::write("test.txt", repeated) { 
                eprintln!("Errore durante la scrittura: {}", e); 
            } else {
                println!("File 'test.txt' aggiornato con 10 ripetizioni!");
            }
        }
        Err(e) => eprintln!("Errore durante la lettura: {}", e), 
    }
}

// ==========================================
// Passo 2: Enum con valori associati
// ==========================================
pub enum MyError {
    Simple(SystemTime), 
    Complex(SystemTime, String),
}

pub fn print_error(e: MyError) { 
    match e { 
        MyError::Simple(time) => {
            // Per evitare di usare {:?} (Debug formatter), estraiamo i secondi trascorsi dall'epoca
            let secs = time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
            println!("Errore Semplice al timestamp: {}", secs);
        }
        MyError::Complex(time, msg) => {
            let secs = time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
            println!("Errore Complesso al timestamp {}: {}", secs, msg);
        }
    }
}

// ==========================================
// Passo 3: Funzioni che restituiscono Result
// ==========================================
#[derive(Debug, PartialEq)] // Comodo per fare dei test
pub enum MulErr {
    Overflow, 
    NegativeNumber, 
}

pub fn mul(a: i32, b: i32) -> Result<u32, MulErr> { 
    // Se uno SOLO dei due è negativo. 
    // Lo XOR (^) bit a bit sui booleani fa esattamente questo!
    if (a < 0) ^ (b < 0) {
        return Err(MulErr::NegativeNumber);
    }

    // Se arriviamo qui, i numeri sono entrambi positivi o entrambi negativi.
    // Il loro prodotto sarà positivo, quindi convertiamo i valori assoluti in u32.
    // unsigned_abs() è perfetto perché gestisce anche il caso limite di i32::MIN.
    let u_a = a.unsigned_abs();
    let u_b = b.unsigned_abs();

    // Usiamo checked_mul su u32 per rilevare l'overflow in sicurezza 
    match u_a.checked_mul(u_b) {
        Some(res) => Ok(res), 
        None => Err(MulErr::Overflow), 
    }
}

fn main() {
    println!("--- Test Esercizi Propedeutici ---");
    println!("--- Test Esercizi Propedeutici ---");

    // ==========================================
    // Test Passo 1
    // ==========================================
    println!("\n> Test Passo 1:");
    // Chiama la funzione. Se test.txt esiste, lo leggerà e lo moltiplicherà!
    passo1();

    // ==========================================
    // Test Passo 2
    // ==========================================
    println!("\n> Test Passo 2:");
    // Generiamo l'istante di tempo attuale
    let now = SystemTime::now();
    
    // Creiamo i due tipi di errore
    let err_semplice = MyError::Simple(now);
    let err_complesso = MyError::Complex(now, String::from("Impossibile connettersi al server"));
    
    // Li stampiamo
    print_error(err_semplice);
    print_error(err_complesso);

    // ==========================================
    // Test Passo 3
    // ==========================================
    println!("\n> Test Passo 3:");
    // Stampiamo i Result (usiamo {:?} perché Result implementa il tratto Debug)
    
    // Caso 1: Entrambi positivi (dovrebbe dare Ok)
    println!("mul(5, 4) = {:?}", mul(5, 4)); 
    
    // Caso 2: Entrambi negativi (dovrebbe dare Ok)
    println!("mul(-5, -4) = {:?}", mul(-5, -4)); 
    
    // Caso 3: Uno solo negativo (dovrebbe dare Err(NegativeNumber))
    println!("mul(5, -4) = {:?}", mul(5, -4)); 
    
    // Caso 4: Numeri troppo grandi per u32 (dovrebbe dare Err(Overflow))
    // 100.000 * 100.000 = 10.000.000.000, che supera il limite di u32 (circa 4.2 miliardi)
    println!("mul(100000, 100000) = {:?}", mul(100000, 100000));
}