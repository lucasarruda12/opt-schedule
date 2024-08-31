/// - The first digits in the Sigaa Time Format represent days of the week.
/// - Weekdays range from 2 to 6, as in portuguese they are called 'second' to 'sixth'.
/// - Even though 1 is a valid day, i believe there aren't any classes on sunday. It is represented
/// here anyway.
/// - 7 is also valid. I believe there are classes on saturday, but i've never seen/been to one.
///
/// - I opted for an array of 8 boolean values to represent the sigaa day format.
/// Even though i won't use the 0th position of the array, doing it this way it this way makes it
/// makes sense to write certain_days.values[2] and i believe that makes this part of the code much more readable.
///
/// - I also opted to hide the array implementation here, and since i plan on using this part of
/// the code for formating strings, all the interface of the methods were made to handle char or
/// string arguments.

use std::cmp::PartialEq;
use std::cmp::Eq;
use std::fmt;

#[derive(Debug)]
pub struct Days {
    values: [bool; 8],
}

impl fmt::Display for Days {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for i in 0..8 {
            if self.values[i] {
                result = result + &i.to_string();
            }
        }

        write!(f, "{}", result)
   }
}

impl Days {

    pub fn new() -> Days {
        Days{values: [false; 8]}
    }

    pub fn from_string(str: &String) -> Result<Days, String> {
        let mut new = Days::new();

        for c in str.chars() {
            match new.add_day(&c) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(new)
    }

    pub fn concat(&mut self, other: &Days) {
        for iter in 1..8 {
            if other.values[iter] {
                self.values[iter] = true;
            }
        }
    }

    pub fn add_day(&mut self, c: &char) -> Result<(), String> {
        self.add_day_from_usize(*c as usize - '0' as usize)
    }

    pub fn remove_day(&mut self, c: &char) -> Result<(), String> {
        self.remove_day_from_usize(*c as usize - '0' as usize)
    }

    pub fn toggle_day(&mut self, c: &char) -> Result<(), String> {
        let s = *c as usize - '0' as usize;

        match self.values[s] {
            true => self.remove_day_from_usize(s),
            false => self.add_day_from_usize(s),
        }
    }

    fn from_slice(str: &str) -> Result<Days, String> {
        Self::from_string(&str.to_string())
    }

    fn add_day_from_usize(&mut self, u: usize) -> Result<(), String> {
        match u {
            1..8 => {
                self.values[u] = true;
                Ok(())
            }
            _ => Err(String::from("Invalid input")),
        }
    }

    fn remove_day_from_usize(&mut self, u: usize) -> Result<(), String> {
        match u {
            1..8 => {
                self.values[u] = false;
                Ok(())
            }
            _ => Err(String::from("Invalid input")),
        }
    }
}

impl PartialEq for Days {
    fn eq(&self, other: &Days) -> bool {
        for i in 1..8 {
            if self.values[i] != other.values[i] {
                return false;
            }
        }

        true
    }
}

impl Eq for Days {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_days_should_be_empty() {
        assert_eq!(Days::new().to_string(), String::new());
    }

    #[test]
    fn one_to_seven_are_valid_days() {
        for str in [
            "123",
            "456",
            "246",
            "35",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
        ]{
            let days = Days::from_slice(str).unwrap();
            assert_eq!(days.to_string(), str.to_string());
        }
    }

    #[test]
    fn arbitrary_chars_are_invalid_days() {
        for str in [
            "abc",
            "8",
            "lucas",
            "pipoquinha",
            "ĺúćáś"
        ]{
            let days = Days::from_slice(str);
            assert!(days.is_err());
        }
    }

    #[test]
    fn should_be_able_to_add_valid_chars() {
        let mut days = Days::new();
        let mut str = String::new();

        for i in "1234567".to_string().chars() {
            let _ = days.add_day(&i);
            str = str + &i.to_string();

            assert_eq!(days.to_string(), str);
        }

    }

    #[test]
    fn should_be_able_to_remove_valid_chars() {
        let mut str = String::from("1234567");
        let mut days = Days::from_string(&str).unwrap();

        for i in "7654321".to_string().chars() {
            let _ = days.remove_day(&i);
            str.pop();

            assert_eq!(days.to_string(), str);
        }
    }

    #[test]
    fn should_be_able_to_toggle_valid_chars() {
        let str = String::from("1357");
        let mut days = Days::from_string(&str).unwrap();

        for i in "1234567".to_string().chars() {
            let _ = days.toggle_day(&i);
        }

        assert_eq!(days.to_string(), "246".to_string());
    }

    #[test]
    fn should_be_able_to_concat_two_days() {
        let mut d1 = Days::from_slice("123").unwrap();
        let d2 = Days::from_slice("456").unwrap();
        let _ = d1.concat(&d2);

        assert_eq!(d1, Days::from_slice("123456").unwrap());
    }
}
