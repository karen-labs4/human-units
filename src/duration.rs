//! Duration formatting and parsing for compound human strings like
//! `"1h30m"` or `"250ms"`.

use std::fmt;
use std::time::Duration;

/// Renders a `Duration` as a compact human-readable string, e.g.
/// `"1h 5m 30s"` or `"250ms"` for anything under a second.
///
/// ```
/// use std::time::Duration;
/// use human_units::format_duration;
/// assert_eq!(format_duration(Duration::from_secs(0)), "0s");
/// assert_eq!(format_duration(Duration::from_millis(250)), "250ms");
/// assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
/// ```
pub fn format_duration(duration: Duration) -> String {
    if duration.is_zero() {
        return "0s".to_string();
    }

    let total_millis = duration.as_millis();
    if total_millis < 1000 {
        return format!("{total_millis}ms");
    }

    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    let mut parts = Vec::new();
    if hours > 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!("{seconds}s"));
    }
    parts.join(" ")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseDurationError {
    Empty,
    InvalidNumber(String),
    UnknownUnit(String),
}

impl fmt::Display for ParseDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseDurationError::Empty => write!(f, "duration string is empty"),
            ParseDurationError::InvalidNumber(s) => write!(f, "invalid number: {s:?}"),
            ParseDurationError::UnknownUnit(s) => write!(f, "unknown duration unit: {s:?}"),
        }
    }
}

impl std::error::Error for ParseDurationError {}

/// Parses a compound duration string such as `"1h30m"`, `"45s"`, or
/// `"500ms"` into a `Duration`. Recognized units are `ms`, `s`, `m`, and
/// `h`; multiple number-unit pairs can be concatenated with no separator.
///
/// ```
/// use std::time::Duration;
/// use human_units::parse_duration;
/// assert_eq!(parse_duration("1h30m").unwrap(), Duration::from_secs(5400));
/// assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
/// ```
pub fn parse_duration(input: &str) -> Result<Duration, ParseDurationError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseDurationError::Empty);
    }

    let mut total = Duration::ZERO;
    let mut chars = trimmed.chars().peekable();

    while chars.peek().is_some() {
        let mut number_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() || c == '.' {
                number_str.push(c);
                chars.next();
            } else {
                break;
            }
        }
        if number_str.is_empty() {
            return Err(ParseDurationError::InvalidNumber(trimmed.to_string()));
        }

        let mut unit_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_alphabetic() {
                unit_str.push(c);
                chars.next();
            } else {
                break;
            }
        }

        let number: f64 = number_str
            .parse()
            .map_err(|_| ParseDurationError::InvalidNumber(number_str.clone()))?;

        let millis_per_unit: f64 = match unit_str.as_str() {
            "ms" => 1.0,
            "s" => 1_000.0,
            "m" => 60_000.0,
            "h" => 3_600_000.0,
            other => return Err(ParseDurationError::UnknownUnit(other.to_string())),
        };

        total += Duration::from_millis((number * millis_per_unit).round() as u64);
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_zero() {
        assert_eq!(format_duration(Duration::ZERO), "0s");
    }

    #[test]
    fn formats_sub_second_as_millis() {
        assert_eq!(format_duration(Duration::from_millis(250)), "250ms");
    }

    #[test]
    fn formats_whole_seconds() {
        assert_eq!(format_duration(Duration::from_secs(45)), "45s");
    }

    #[test]
    fn formats_hours_minutes_seconds() {
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    }

    #[test]
    fn omits_zero_components_except_seconds() {
        assert_eq!(format_duration(Duration::from_secs(3600)), "1h");
        assert_eq!(format_duration(Duration::from_secs(60)), "1m");
    }

    #[test]
    fn parses_single_unit() {
        assert_eq!(parse_duration("45s").unwrap(), Duration::from_secs(45));
        assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
    }

    #[test]
    fn parses_compound_units() {
        assert_eq!(parse_duration("1h30m").unwrap(), Duration::from_secs(5400));
        assert_eq!(
            parse_duration("2h15m10s").unwrap(),
            Duration::from_secs(2 * 3600 + 15 * 60 + 10)
        );
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(parse_duration(""), Err(ParseDurationError::Empty));
    }

    #[test]
    fn rejects_unknown_unit() {
        assert_eq!(
            parse_duration("5d"),
            Err(ParseDurationError::UnknownUnit("d".to_string()))
        );
    }
}
