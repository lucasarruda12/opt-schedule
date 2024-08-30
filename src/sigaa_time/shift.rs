use std::fmt;

pub enum Shift {
    M,
    T,
    N,
}

impl fmt::Display for Shift {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
            Shift::M => write!(f, "M"),
            Shift::T => write!(f, "T"),
            Shift::N => write!(f, "N"),
       }
   }
}

impl Shift {
    pub fn from_string(shift: &str) -> Result<Shift, String> {
        match shift {
            "M" => Ok(Shift::M),
            "m" => Ok(Shift::M),

            "T" => Ok(Shift::T),
            "t" => Ok(Shift::T),

            "N" => Ok(Shift::N),
            "n" => Ok(Shift::N),

            _ => Err(String::from("Únicos turnos permitidos são M, T, N")),
        }
    }
}
