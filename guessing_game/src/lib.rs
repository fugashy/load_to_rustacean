pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, String> {
        if value < 1 || 100 < value {
            return Err(String::from("Guess value must be between 1 and 100"));
        }
        Ok(Guess { value })
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}
impl std::fmt::Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "value: {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::Guess;

    #[test]
    fn retun_ok() {
        let r: std::result::Result<Guess, String> = Guess::new(100);
        let g = match r {
            Ok(guess) => guess,
            Err(e) => panic!("Error: {:?}", e),
        };
        assert_eq!(g.value(), 100);
    }

    #[test]
    fn return_err() {
        let r: std::result::Result<Guess, String> = Guess::new(101);
        let _g = match r {
            Ok(_guess) => panic!("Error"),
            Err(e) => e,
        };
    }
}
