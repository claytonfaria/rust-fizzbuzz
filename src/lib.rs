use std::fmt;

pub fn fizzbuzz(value: i32) -> FizzBuzz {
    match (value % 3, value % 5) {
        (0, 0) => FizzBuzz::FizzBuzz,
        (_, 0) => FizzBuzz::Buzz,
        (0, _) => FizzBuzz::Fizz,
        (_, _) => FizzBuzz::Number(value),
    }
}
#[derive(Debug, PartialEq)]
pub enum FizzBuzz {
    FizzBuzz,
    Fizz,
    Buzz,
    Number(i32),
}

impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FizzBuzz::Number(n) => write!(f, "{}", n),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizzbuzz(1), FizzBuzz::Number(1));
        assert_eq!(fizzbuzz(3), FizzBuzz::Fizz);
        assert_eq!(fizzbuzz(5), FizzBuzz::Buzz);
        assert_eq!(fizzbuzz(7), FizzBuzz::Number(7));
        assert_eq!(fizzbuzz(15), FizzBuzz::FizzBuzz);
        assert_eq!(fizzbuzz(30), FizzBuzz::FizzBuzz);
        assert_eq!(fizzbuzz(33), FizzBuzz::Fizz);
    }
}
