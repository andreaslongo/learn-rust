const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";

#[allow(dead_code)]
fn convert(number: usize) -> String {
    let multiple_of = |factor| number % factor == 0;
    match number {
        _ if multiple_of(15) => format!("{FIZZ}{BUZZ}"),
        _ if multiple_of(5) => BUZZ.to_string(),
        _ if multiple_of(3) => FIZZ.to_string(),
        _ => number.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_to_string() {
        assert_eq!(convert(1), "1".to_string());
        assert_eq!(convert(2), "2".to_string());
        assert_eq!(convert(4), "4".to_string());
    }

    #[test]
    fn multiple_of_3_is_fizz() {
        assert_eq!(convert(3), "Fizz".to_string());
        assert_eq!(convert(6), "Fizz".to_string());
        assert_eq!(convert(9), "Fizz".to_string());
    }

    #[test]
    fn multiple_of_5_is_buzz() {
        assert_eq!(convert(5), "Buzz".to_string());
        assert_eq!(convert(10), "Buzz".to_string());
        assert_eq!(convert(20), "Buzz".to_string());
    }

    #[test]
    fn multiple_of_15_is_fizzbuzz() {
        assert_eq!(convert(15), "FizzBuzz".to_string());
        assert_eq!(convert(30), "FizzBuzz".to_string());
        assert_eq!(convert(45), "FizzBuzz".to_string());
    }
}
