use std::str::FromStr;

/// A type representing a percentage value.
///
/// Wraps an `f64` value where `1.0` represents 100%. This is useful for CLI
/// parsing where users might input `0.5`, `50`, or `50%`, all of which
/// should resolve to a 50% representation (i.e. `0.5`).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(f64);

impl Percent {
    /// Creates a new `Percent` from an `f64`.
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Gets the inner `f64` value.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Returns the value as a decimal string (e.g. "0.5" for 50%).
    pub fn as_decimal(&self) -> String {
        self.0.to_string()
    }

    /// Returns the value as a percentage string with percent sign
    /// (e.g. "50%" for 50%).
    pub fn as_percentage(&self) -> String {
        format!("{}%", self.0 * 100.0)
    }
}

impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0 * 100.0)
    }
}

/// The error type returned when parsing a `Percent` fails.
#[derive(Debug, PartialEq, Eq)]
pub struct ParsePercentError(std::num::ParseFloatError);

impl std::fmt::Display for ParsePercentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse percent: {}", self.0)
    }
}

impl std::error::Error for ParsePercentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl FromStr for Percent {
    type Err = ParsePercentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim();
        let mut has_percent_suffix = false;

        if s.ends_with('%') {
            s = &s[..s.len() - 1];
            has_percent_suffix = true;
        }

        let mut f: f64 = s.parse().map_err(ParsePercentError)?;

        if has_percent_suffix || f > 1.0 {
            // e.g. 80 -> 0.8
            f /= 100.0;
        }

        Ok(Percent(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal_less_than_one() {
        let p: Percent = "0.8".parse().unwrap();
        assert_eq!(p.value(), 0.8);
    }

    #[test]
    fn parse_decimal_with_percent_suffix() {
        let p: Percent = "80%".parse().unwrap();
        assert_eq!(p.value(), 0.8);
    }

    #[test]
    fn parse_whole_number_greater_than_one() {
        let p: Percent = "80".parse().unwrap();
        assert_eq!(p.value(), 0.8);
    }

    #[test]
    fn parse_exactly_one() {
        let p: Percent = "1.0".parse().unwrap();
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn parse_hundred_percent() {
        let p: Percent = "100%".parse().unwrap();
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn parse_hundred() {
        let p: Percent = "100".parse().unwrap();
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn parse_zero() {
        let p: Percent = "0".parse().unwrap();
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn parse_zero_percent() {
        let p: Percent = "0%".parse().unwrap();
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn invalid_string() {
        let res = "abc".parse::<Percent>();
        assert!(res.is_err());
    }

    #[test]
    fn display_error() {
        let err = "abc".parse::<Percent>().unwrap_err();
        assert!(err.to_string().contains("failed to parse percent:"));
    }

    #[test]
    fn as_decimal_returns_inner_value() {
        let p = Percent::new(0.5);
        assert_eq!(p.as_decimal(), "0.5");
    }

    #[test]
    fn as_decimal_zero() {
        let p = Percent::new(0.0);
        assert_eq!(p.as_decimal(), "0");
    }

    #[test]
    fn as_decimal_one() {
        let p = Percent::new(1.0);
        assert_eq!(p.as_decimal(), "1");
    }

    #[test]
    fn as_percentage_from_half() {
        let p = Percent::new(0.5);
        assert_eq!(p.as_percentage(), "50%");
    }

    #[test]
    fn as_percentage_from_one() {
        let p = Percent::new(1.0);
        assert_eq!(p.as_percentage(), "100%");
    }

    #[test]
    fn as_percentage_from_zero() {
        let p = Percent::new(0.0);
        assert_eq!(p.as_percentage(), "0%");
    }

    #[test]
    fn display_shows_percent_sign() {
        let p = Percent::new(0.8);
        assert_eq!(format!("{p}"), "80%");
    }

    #[test]
    fn display_shows_percent_sign_for_hundred() {
        let p = Percent::new(1.0);
        assert_eq!(format!("{p}"), "100%");
    }

    #[test]
    fn display_shows_percent_sign_for_zero() {
        let p = Percent::new(0.0);
        assert_eq!(format!("{p}"), "0%");
    }
}
