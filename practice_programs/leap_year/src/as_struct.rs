/// Wraps an i32 to represent a year.
#[derive(Debug)]
pub struct Year(pub i32);

impl Year {
    /// Returns `true` if `Year` is a leap year.
    ///
    /// # Examples
    ///
    /// ```
    /// use leap_year::Year;
    ///
    /// assert!(Year(2000).is_leap_year());
    /// assert!(!Year(1997).is_leap_year());
    /// ```
    #[must_use]
    pub fn is_leap_year(&self) -> bool {
        let divisible_by = |divisor| self.0 % divisor == 0;
        divisible_by(400) || !divisible_by(100) && divisible_by(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_is_divisible_by_4() {
        assert!(Year(1996).is_leap_year());
        assert!(!Year(1997).is_leap_year());
    }

    #[test]
    fn leap_year_is_divisible_by_400() {
        assert!(Year(1600).is_leap_year());
        assert!(!Year(1700).is_leap_year());
    }

    #[test]
    fn leap_year_is_divisible_by_100_and_400() {
        assert!(Year(2000).is_leap_year());
        assert!(!Year(1800).is_leap_year());
    }
}
