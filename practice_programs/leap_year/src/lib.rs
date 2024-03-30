#[allow(dead_code)]
fn is_leap_year(year: usize) -> bool {
    let divisible_by = |divisor| year % divisor == 0;
    divisible_by(400) || !divisible_by(100) && divisible_by(4)
    // Alternative:
    // divisible_by(if divisible_by(100) { 400 } else { 4 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_is_divisible_by_4() {
        assert!(is_leap_year(1996));
        assert!(!is_leap_year(1997));
    }

    #[test]
    fn leap_year_is_divisible_by_400() {
        assert!(is_leap_year(1600));
        assert!(!is_leap_year(1700));
    }

    #[test]
    fn leap_year_is_divisible_by_100_and_400() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1800));
    }
}
