use std::str::FromStr;

/// A type representing a percentage value.
///
/// Wraps an `f64` value where `1.0` represents 100%. This is useful for CLI
/// parsing where users might input `0.5`, `50`, or `50%`, all of which
/// should resolve to a 50% representation (i.e. `0.5`).
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
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

impl From<f32> for Percent {
    fn from(value: f32) -> Self {
        Percent(value.into())
    }
}

impl From<f64> for Percent {
    fn from(value: f64) -> Self {
        Percent(value)
    }
}

impl std::ops::Mul<f64> for Percent {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl std::ops::Div<f64> for Percent {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.0 / rhs
    }
}

impl std::ops::Add for Percent {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Percent(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Percent {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Percent(self.0 - rhs.0)
    }
}

impl std::ops::AddAssign for Percent {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::SubAssign for Percent {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::iter::Sum for Percent {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Percent(0.0), |a, b| a + b)
    }
}

#[cfg(feature = "types-serde")]
impl serde::Serialize for Percent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "types-serde")]
impl<'de> serde::Deserialize<'de> for Percent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        f64::deserialize(deserializer).map(Percent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_f64_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() < f64::EPSILON,
            "left {a} != right {b}"
        );
    }

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

    #[test]
    fn default_is_zero() {
        let p = Percent::default();
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn mul_applies_percentage() {
        let p = Percent::new(0.25);
        assert_eq!(p * 200.0, 50.0);
    }

    #[test]
    fn div_reverses_percentage() {
        let p = Percent::new(0.5);
        assert_eq!(p * 200.0 / 0.5, 200.0);
    }

    #[test]
    fn add_percents() {
        let a = Percent::new(0.1);
        let b = Percent::new(0.2);
        assert_f64_eq((a + b).value(), 0.3);
    }

    #[test]
    fn sub_percents() {
        let a = Percent::new(0.5);
        let b = Percent::new(0.2);
        assert_f64_eq((a - b).value(), 0.3);
    }

    #[test]
    fn add_assign_percents() {
        let mut a = Percent::new(0.1);
        a += Percent::new(0.2);
        assert_f64_eq(a.value(), 0.3);
    }

    #[test]
    fn sub_assign_percents() {
        let mut a = Percent::new(0.5);
        a -= Percent::new(0.2);
        assert_f64_eq(a.value(), 0.3);
    }

    #[test]
    fn sum_percents() {
        let values = [
            Percent::new(0.1),
            Percent::new(0.2),
            Percent::new(0.3),
        ];
        let total: Percent = values.into_iter().sum();
        assert_f64_eq(total.value(), 0.6);
    }
}
