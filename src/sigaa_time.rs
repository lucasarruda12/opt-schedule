use std::vec::Vec;

pub use self::day::Days;
pub use self::shift::Shift;
pub use self::period::Period;

pub mod day;
pub mod shift;
pub mod period;

struct SigaaTime {
    weekdays: Days,
    shift: Shift,
    periods: Vec<Period>,
}

//impl fmt::Display for SigaaTime {
//    fn fmt(&self, &mut fmt::Formater<'_>) -> fmt::Result {
//        for weekday in weekdays {
//
//        }
//    }
//}
//

