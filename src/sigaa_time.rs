use std::fmt;

pub struct Weekday(u8);

impl fmt::Display for Weekday {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Weekday(2) => write!(f, "2"),
           Weekday(3) => write!(f, "3"),
           Weekday(4) => write!(f, "4"),
           Weekday(5) => write!(f, "5"),
           Weekday(6) => write!(f, "6"),
           Weekday(_) => write!(f, "?"),
       }
   }
}

impl Weekday {
    pub fn new(reference_number: u8) -> Result<Weekday, String> {
        if reference_number >= 2 && reference_number <= 7 {
            Ok(Weekday(reference_number))
        }

        else {
            Err(String::from("Dias da semana devem ser entre 2 e 6"))
        }
    }
}

 //use std::vec;
 //
 //struct SigaaTime {
 //    day_of_the_week: vec<Weekday>,
 //    shift: SigaaShift,
 //    period: vec<SigaaPeriod>,
 //}
 //
 //impl fmt::Display for SigaaTime {
 //    fn fmt(&self, &mut fmt::Formater<'_>) -> fmt::Result {
 //        write!(f, "")
 //    }
 //}
