use std::fmt;
use std::collections::HashSet

pub enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fry,
}

impl fmt::Display for Day {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Day::Mon => write!(f, "2"),
           Day::Tue => write!(f, "3"),
           Day::Wed => write!(f, "4"),
           Day::Thu => write!(f, "5"),
           Day::Fry => write!(f, "6"),
       }
   }
}

impl Day {
    pub fn from_int(day: u8) -> Result<Day, String>{
        match day {
            2 => Ok(Day::Mon),
            3 => Ok(Day::Tue),
            4 => Ok(Day::Wed),
            5 => Ok(Day::Thu),
            6 => Ok(Day::Fry),
            _ => Err(String::from("Dia da semana deve ser entre 2 e 6")),
        }
    }
}
