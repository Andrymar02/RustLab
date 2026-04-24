pub mod solution {
    // Definiamo anche l'errore che ci servirà più avanti nei test
    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError {
        ImaginaryNotZero,
    }

    // La nostra struttura per i numeri complessi
    pub struct ComplexNumber {
        real: f64,
        imag: f64,
    }

    impl ComplexNumber {
        /// Crea un nuovo numero complesso
        pub fn new(real: f64, imag: f64) -> Self {
            Self { real, imag }
        }

        /// Crea un numero complesso con sola parte reale (immaginaria a 0)
        pub fn from_real(real: f64) -> Self {
            Self { real, imag: 0.0 }
        }

        /// Restituisce la parte reale
        pub fn real(&self) -> f64 {
            self.real
        }

        /// Restituisce la parte immaginaria
        pub fn imag(&self) -> f64 {
            self.imag
        }

        /// Restituisce il numero complesso come tupla (reale, immaginaria)
        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real, self.imag)
        }
    }
}