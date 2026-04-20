use std::env;

// Simulo il file table.rs fornito dal professore. 
// Se hai il file table.rs originale nella cartella src, rimuovi questo modulo 
// e scrivi semplicemente: mod table;
pub mod table {
    pub const SUBS_I: &str = "àáâäæãåāèéêëēėęîïíīįìôöòóœøōõûüùúūñńçćčłśšżźž";
    pub const SUBS_O: &str = "aaaaaaaaeeeeeeeiiiiiiioooooooouuuuunnccclsszzz";
}

pub mod slugify {
    pub fn is_slug(s: &str) -> bool {
        // Uno slug tipico non è vuoto e contiene solo a-z, 0-9, o '-'
        !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }
}

pub trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

impl<T> MySlug for T 
where 
    T: AsRef<str> 
{
    fn is_slug(&self) -> bool {
        slugify::is_slug(self.as_ref())
    }

    fn to_slug(&self) -> String {
        // Qui useresti la logica dell'esercitazione precedente
        // Per ora facciamo un esempio semplice:
        self.as_ref().to_lowercase().replace(" ", "-")
    }
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

    // In tutti gli altri casi, restituiamo '-' 
    '-'
}

/// Passo 1: Funzione principale di slugify 
fn slugify(s: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for c in s.chars() {
        // .to_lowercase() restituisce un iteratore 
        for lower_c in c.to_lowercase() {
            let converted = conv(lower_c);
            
            if converted == '-' {
                // Non ammettiamo due '-' consecutivi 
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

    // Un '-' finale non è ammesso, a meno che non sia l'unico carattere 
    if slug.len() > 1 && slug.ends_with('-') {
        slug.pop();
    }

    slug
}


/// Passo 4: Interfaccia da command line
fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Controlliamo che sia stata passata la stringa come argomento 
    if args.len() > 1 {
        let input = &args[1];
        let result = slugify(input);
        println!("slug: {}", result);
    } else {
        eprintln!("Uso: cargo run -- \"stringa da convertire\"");
    }
}

/// Passo 3: Unit test 
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