/// - The first few digits in the Sigaa Time Format represent days of the week.
/// - Weekdays range from 2 to 6, as in portuguese they are called 'second' to 'sixth'.
/// - Even though 1 is a valid day, i believe there aren't any classes on sunday. It is represented
/// here anyway.
/// - 7 is also valid. I believe there are classes on saturday, but i've never seen one.
///
/// - I opted for an array of 8 boolean values to represent these.
/// Even though i won't use the 0th position of the array, this way it makes
/// sense to write certain_days.values[2] and i believe it makes this part of the code
/// much more readable.
///
/// - I also opted to hide the array implementation here, and since i plan on using this part of
/// the code for formating strings, all the interface of the methods were made to handle char
/// arguments.

pub struct Days {
    values: [bool; 8],
}

impl Days {

    pub fn cat(&mut self, other: Days) {
        for iter in 0..8 {
            if other.values[iter] {
                self.values[iter] = true;
            }
        }
    }

    fn add_day_from_usize(&mut self, u: usize) -> Result<(), String> {
        match u {
            0..=8 => {
                self.values[u] = true;
                Ok(())
            }
            _ => Err(String::from("Invalid input")),
        }
    }

    fn remove_day_from_usize(&mut self, u: usize) -> Result<(), String> {
        match u {
            0..=8 => {
                self.values[u] = false;
                Ok(())
            }
            _ => Err(String::from("Invalid input")),
        }
    }

    pub fn add_day(&mut self, c: char) -> Result<(), String> {
        self.add_day_from_usize(c as usize - '0' as usize)
    }

    pub fn remove_day(&mut self, c: char) -> Result<(), String> {
        self.remove_day_from_usize(c as usize - '0' as usize)
    }

    pub fn toggle_day(&mut self, c: char) -> Result<(), String> {
        let c = c as usize - '0' as usize;

        match self.values[c] {
            true => self.remove_day_from_usize(c),
            false => self.add_day_from_usize(c),
        }
    }

    pub fn from_string(str: String) -> Result<Days, String> {
        let mut new = Days{values: [false; 8]};

        for c in str.chars() {
            match new.add_day(c) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(new)
    }
}
