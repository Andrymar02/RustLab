use std::env;

// Simulo il file table.rs fornito dal professore. 
// Se hai il file table.rs originale nella cartella src, rimuovi questo modulo 
// e scrivi semplicemente: mod table;
pub mod table {
    pub const SUBS_I: &str = "àáâäæãåāèéêëēėęîïíīįìôöòóœøōõûüùúūñńçćčłśšżźž";
    pub const SUBS_O: &str = "aaaaaaaaeeeeeeeiiiiiiioooooooouuuuunnccclsszzz";
}

use table::{SUBS_I, SUBS_O};

/// Passo 2: Funzione di conversione del singolo carattere [cite: 71]
fn conv(c: char) -> char {
    // Se è un carattere alfanumerico semplice, lo restituiamo [cite: 77]
    if c.is_ascii_lowercase() || c.is_ascii_digit() {
        return c;
    }

    // Cerchiamo il carattere nella tabella degli accenti SUBS_I 
    // Usiamo .chars().position() perché non possiamo indicizzare le stringhe UTF-8 con [i] [cite: 87]
    if let Some(idx) = SUBS_I.chars().position(|x| x == c) {
        // Se lo troviamo, restituiamo il carattere corrispondente in SUBS_O [cite: 78, 88]
        if let Some(out_char) = SUBS_O.chars().nth(idx) {
            return out_char;
        }
    }

    // In tutti gli altri casi, restituiamo '-' [cite: 79]
    '-'
}

/// Passo 1: Funzione principale di slugify [cite: 66]
fn slugify(s: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for c in s.chars() {
        // .to_lowercase() restituisce un iteratore [cite: 81]
        for lower_c in c.to_lowercase() {
            let converted = conv(lower_c);
            
            if converted == '-' {
                // Non ammettiamo due '-' consecutivi [cite: 63]
                if !last_was_dash {
                    slug.push('-');
                    last_was_dash = true;
                }
            } else {
                slug.push(converted);
                last_was_dash = false;
            }
        }
    }

    // Un '-' finale non è ammesso, a meno che non sia l'unico carattere [cite: 64]
    if slug.len() > 1 && slug.ends_with('-') {
        slug.pop();
    }

    slug
}

/// Passo 4: Interfaccia da command line [cite: 114, 115]
fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Controlliamo che sia stata passata la stringa come argomento [cite: 116]
    if args.len() > 1 {
        let input = &args[1];
        let result = slugify(input);
        println!("slug: {}", result);
    } else {
        eprintln!("Uso: cargo run -- \"stringa da convertire\"");
    }
}

/// Passo 3: Unit test [cite: 91, 92]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv_accented() {
        assert_eq!(conv('à'), 'a');
    }

    #[test]
    fn test_conv_plain() {
        assert_eq!(conv('b'), 'b');
    }

    #[test]
    fn test_conv_invalid() {
        assert_eq!(conv('!'), '-');
    }

    #[test]
    fn test_conv_unknown_accent() {
        assert_eq!(conv('ฒ'), '-'); 
    }

    #[test]
    fn test_slug_multiword() {
        assert_eq!(slugify("ciao mondo"), "ciao-mondo");
    }

    #[test]
    fn test_slug_accented() {
        assert_eq!(slugify("città"), "citta");
    }

    #[test]
    fn test_slug_empty() {
        assert_eq!(slugify(""), ""); 
    }

    #[test]
    fn test_slug_consecutive_spaces() {
        assert_eq!(slugify("ciao   mondo"), "ciao-mondo");
    }

    #[test]
    fn test_slug_consecutive_invalid() {
        assert_eq!(slugify("ciao!@#mondo"), "ciao-mondo");
    }

    #[test]
    fn test_slug_only_invalid() {
        assert_eq!(slugify("!@#"), "-");
    }

    #[test]
    fn test_slug_trailing_space() {
        assert_eq!(slugify("ciao "), "ciao");
    }

    #[test]
    fn test_slug_trailing_invalid() {
        assert_eq!(slugify("ciao!@#"), "ciao");
    }
}