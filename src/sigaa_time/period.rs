//use std::fmt;
//
//pub enum Period {
//    First,
//    Second,
//    Third,
//    Fourth,
//    Fifth,
//    Sixth,
//}
//
//impl fmt::Display for Period {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       match self {
//            Period::First => write!(f, "1"),
//            Period::Second => write!(f, "2"),
//            Period::Third => write!(f, "3"),
//            Period::Fourth => write!(f, "4"),
//            Period::Fifth => write!(f, "5"),
//            Period::Sixth => write!(f, "6"),
//       }
//   }
//}
//
//impl Period {
//    pub fn from_int(period: u8) -> Result<Period, String> {
//        match period {
//            1 => Ok(Period::First),
//            2 => Ok(Period::Second),
//            3 => Ok(Period::Third),
//            4 => Ok(Period::Fourth),
//            5 => Ok(Period::Fifth),
//            6 => Ok(Period::Sixth),
//            _ => Err(String::from("Únicos periodos possíveis são de 1-6"))
//        }
//    }
//}
