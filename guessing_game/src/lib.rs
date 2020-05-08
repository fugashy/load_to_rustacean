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
        let r = Guess::new(100).unwrap();
        assert_eq!(r.value(), 100);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn a_value_that_is_larger_than_100_should_panic() {
        Guess::new(101).unwrap();
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn a_value_that_is_less_than_1_should_panic() {
        Guess::new(0).unwrap();
    }
}
