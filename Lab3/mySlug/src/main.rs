pub mod slugify {
    // Inserisci qui le costanti SUBS_I e SUBS_O e la funzione conv() del tuo codice precedente.
    pub const SUBS_I: &str = "àáâäæãåāèéêëēėęîïíīįìôöòóœøōõûüùúūñńçćčłśšżźž";
    pub const SUBS_O: &str = "aaaaaaaaeeeeeeeiiiiiiioooooooouuuuunnccclsszzz";

    fn conv(c: char) -> char {
        /* ... il tuo codice ... */
        if c.is_ascii_lowercase() || c.is_ascii_digit() { return c; }
        if let Some(idx) = SUBS_I.chars().position(|x| x == c) {
            if let Some(out_char) = SUBS_O.chars().nth(idx) { return out_char; }
        }
        '-'
    }

    pub fn to_slug(s: &str) -> String {
        /* ... il tuo codice originale della funzione slugify() ... */
        let mut slug = String::new();
        let mut last_was_dash = false;
        for c in s.chars() {
            for lower_c in c.to_lowercase() {
                let converted = conv(lower_c);
                if converted == '-' {
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
        if slug.len() > 1 && slug.ends_with('-') {
            slug.pop();
        }
        slug
    }

    // NUOVA FUNZIONE [cite: 24]
    pub fn is_slug(s: &str) -> bool {
        // Uno slug non è vuoto, contiene solo a-z, 0-9 e '-' non consecutivi, 
        // e non termina con un trattino.
        if s.is_empty() { return false; }
        
        let mut last_was_dash = false;
        for c in s.chars() {
            if c == '-' {
                if last_was_dash { return false; } // Niente "--" consecutivi
                last_was_dash = true;
            } else if c.is_ascii_lowercase() || c.is_ascii_digit() {
                last_was_dash = false;
            } else {
                return false; // Trovato un carattere non valido (es. maiuscole, spazi, simboli)
            }
        }
        !s.ends_with('-') // L'ultimo carattere non può essere un trattino
    }
}

pub trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

// "Implementa MySlug per QUALSIASI TIPO T..." [cite: 34]
impl<T> MySlug for T
where
    // "...purché T sia in grado di fornirmi un riferimento a stringa (&str)" [cite: 42]
    T: AsRef<str>, 
{
    fn is_slug(&self) -> bool {
        // self.as_ref() converte magicamente sia String che &str in un semplice &str
        // così possiamo passarlo alla nostra funzione di supporto
        slugify::is_slug(self.as_ref())
    }

    fn to_slug(&self) -> String {
        slugify::to_slug(self.as_ref())
    }
}

fn main() {
    let s1 = String::from("Hello String"); 
    let s2 = "hello-slice";

    println!("{}", s1.is_slug()); // Dovrebbe stampare: false
    println!("{}", s2.is_slug()); // Dovrebbe stampare: true

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug(); 

    println!("s3:{} s4:{}", s3, s4); // Dovrebbe stampare: s3:hello-string s4:hello-slice [cite: 21]
}