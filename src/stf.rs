use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

const ALLOWED_CHARACTERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', 'M', 'N', 'T'];

pub struct TimeSlot {
    days: [bool; 8],
    shift: char,
    periods: [bool; 7],
}

impl fmt::Display for TimeSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();

        for i in 1..8 {
            if self.days[i] {
                out.push_str(&i.to_string()[..]);
            }
        }

        out.push_str(&self.shift.to_string()[..]);

        for i in 1..7 {
            if self.periods[i] {
                out.push_str(&i.to_string()[..]);
            }
        }

        write!(f, "{}", out)
    }
}

impl TimeSlot {
    fn empty() -> TimeSlot {
        TimeSlot { days: [false; 8], shift: ' ', periods: [false;7] }
    }

    fn add_days(&mut self, str: &str) -> Result<(), Error> {
        for c in str.chars() {
            if !['1', '2', '3', '4', '5', '6', '7'].contains(&c) {
                return Err(Error::InvalidInput);
            }
        }

        for c in str.chars() {
            self.days[c as usize - '0' as usize] = true;
        }

        Ok(())
    }

    fn set_shift(&mut self, c: &char) -> Result<(), Error> {
        if !['M', 'N', 'T'].contains(c) {
            return Err(Error::InvalidInput);
        }

        if *c == 'N' && (self.periods[5] || self.periods[6]) {
            return Err(Error::InvalidInput);
        }

        self.shift = *c;

        Ok(())
    }

    fn add_periods(&mut self, str: &str) -> Result<(), Error> {
        for c in str.chars() {
            println!("{}", c);

            if !['1', '2', '3', '4', '5', '6'].contains(&c) {
                return Err(Error::InvalidInput);
            }

            if (c == '5' || c == '6') && self.shift == 'N' {
                return Err(Error::InvalidInput);
            }
        }

        for c in str.chars() {
            self.periods[c as usize - '0' as usize] = true;
        }

        Ok(())
    }

    pub fn from_string(str: &str) -> Result<TimeSlot, Error> {
        if str.is_empty() {
            return Err(Error::InvalidInput);
        }

        for c in str.chars() {
            if !ALLOWED_CHARACTERS.contains(&c) {
                return Err(Error::InvalidInput);
            }
        }

        let mut out = TimeSlot::empty();

        for (i, c) in str.chars().enumerate() {
            if c == 'M' || c == 'T' || c == 'N' {
                out.add_days(&str[0..i])?;
                out.set_shift(&c)?;
                out.add_periods(&str[i+1..])?;
                break;
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use std::vec::Vec;
    use super::*;

    #[test]
    fn empty_strings_are_not_valid_input() {
        let ts = TimeSlot::from_string("");
        assert!(ts.is_err());
    }

    #[test]
    fn valid_input_follows_sigaa_specification() {
        for str in [
            "246M12",
            "35T34",
            "56N12",
            "2M12",
        ] {
            let ts = TimeSlot::from_string(str).unwrap();
        }
    }

    #[test]
    fn to_string_should_match_input() {
        let string = "23M12";
        let ts = TimeSlot::from_string(string);
        assert_eq!(ts.unwrap().to_string(), string);
    }

    #[test]
    fn disallowed_input_should_return_err() {
        for str in [
            "Lucas",
            "Pipoca",
            "ÑÓN @1ªº̣]}[2 ÁSCĆ©cÇÍ¬ÍNPUT"
        ] {
            let ts = TimeSlot::from_string(str);
            assert!(ts.is_err());
        }
    }

    #[test]
    fn multiple_shifts_should_return_err() {
        let string = "23MNT56";
        let ts = TimeSlot::from_string(string);
        assert!(ts.is_err());
    }

    #[test]
    fn there_are_only_four_periods_at_night() {
        let string = "1234567N56";
        let ts = TimeSlot::from_string(string);
        assert!(ts.is_err());
    }

    #[test]
    fn check_against_real_valid_input() {
        let test_vec = vec!["2T12", "246M34", "246T56", "3T12", "6T3456", "246T12", "246M56", "7M3456", "6M56", "4T56", "35T56", "24T34", "35T12", "3M56", "35N34", "3N1234", "4M1234", "5T34", "35T34", "6N1234", "4N12", "6T56", "3N123", "3N34", "246T34", "3T1", "246N12", "7M34", "35M12", "24N34", "24T56", "24M56", "46M34", "35M34", "24T12", "246M12", "6N12", "3M34", "6T1234", "5T56", "6M3456", "35N12", "5N12", "24M34", "246N34", "3T56", "5N1234", "3T12", "24N12", "4M56", "3N12", "4T12", "46M12", "5N34", "35M56"];

        for str in test_vec {
            let ts = TimeSlot::from_string(str).unwrap();
            assert_eq!(ts.to_string(), str);
        }
    }
}
