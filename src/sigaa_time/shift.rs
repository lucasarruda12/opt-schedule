use std::fmt;

#[derive(Debug)]
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
    pub fn from_char(c: &char) -> Result<Shift, String> {
        match c {
            'M' => Ok(Shift::M),
            'T' => Ok(Shift::T),
            'N' => Ok(Shift::N),
             _  => Err("Invalid input".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_valid_input_is_mnt() {
        for c in [
            'M',
            'N',
            'T',
        ] {
            let shift = Shift::from_char(&c).unwrap();
            assert_eq!(shift.to_string(), c.to_string());
        }
    }

    #[test]
    fn err_on_invalid_input() {
        for c in [
            'a',
            'b',
            'c',
            'é',
            'ë'
        ] {
            let shift = Shift::from_char(&c);
            assert!(shift.is_err());
        }
    }
}
