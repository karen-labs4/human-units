//! Byte size formatting and parsing, using binary (1024-based) units.

use std::fmt;

pub const KIB: u64 = 1024;
pub const MIB: u64 = KIB * 1024;
pub const GIB: u64 = MIB * 1024;
pub const TIB: u64 = GIB * 1024;

/// Renders a byte count as a human-readable string, picking the largest
/// unit that keeps the value at or above 1.0.
///
/// ```
/// use human_units::format_bytes;
/// assert_eq!(format_bytes(0), "0 B");
/// assert_eq!(format_bytes(1536), "1.50 KiB");
/// ```
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [(&str, u64); 4] = [("TiB", TIB), ("GiB", GIB), ("MiB", MIB), ("KiB", KIB)];
    for (name, size) in UNITS {
        if bytes >= size {
            return format!("{:.2} {}", bytes as f64 / size as f64, name);
        }
    }
    format!("{} B", bytes)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseSizeError {
    Empty,
    InvalidNumber(String),
    UnknownUnit(String),
}

impl fmt::Display for ParseSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseSizeError::Empty => write!(f, "size string is empty"),
            ParseSizeError::InvalidNumber(s) => write!(f, "invalid number: {s:?}"),
            ParseSizeError::UnknownUnit(s) => write!(f, "unknown size unit: {s:?}"),
        }
    }
}

impl std::error::Error for ParseSizeError {}

/// Parses a size string such as `"10KB"`, `"1.5 MiB"`, or `"2048"` (bytes,
/// no unit) into a byte count. Both SI-style (`KB`, `MB`, ...) and
/// binary-style (`KiB`, `MiB`, ...) suffixes are accepted and treated as
/// the same 1024-based multiplier, since that is the convention most
/// tools and config files actually use in practice.
///
/// ```
/// use human_units::parse_bytes;
/// assert_eq!(parse_bytes("1.5 MiB").unwrap(), 1_572_864);
/// assert_eq!(parse_bytes("2048").unwrap(), 2048);
/// ```
pub fn parse_bytes(input: &str) -> Result<u64, ParseSizeError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseSizeError::Empty);
    }

    let split_at = trimmed
        .find(|c: char| !c.is_ascii_digit() && c != '.')
        .unwrap_or(trimmed.len());
    let (number_part, unit_part) = trimmed.split_at(split_at);

    let number: f64 = number_part
        .parse()
        .map_err(|_| ParseSizeError::InvalidNumber(number_part.to_string()))?;

    let unit = unit_part.trim();
    let multiplier = match unit.to_ascii_uppercase().as_str() {
        "" | "B" => 1,
        "KB" | "KIB" => KIB,
        "MB" | "MIB" => MIB,
        "GB" | "GIB" => GIB,
        "TB" | "TIB" => TIB,
        other => return Err(ParseSizeError::UnknownUnit(other.to_string())),
    };

    Ok((number * multiplier as f64).round() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_plain_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn formats_each_unit_boundary() {
        assert_eq!(format_bytes(KIB), "1.00 KiB");
        assert_eq!(format_bytes(MIB), "1.00 MiB");
        assert_eq!(format_bytes(GIB), "1.00 GiB");
        assert_eq!(format_bytes(TIB), "1.00 TiB");
    }

    #[test]
    fn parses_bare_number_as_bytes() {
        assert_eq!(parse_bytes("2048").unwrap(), 2048);
    }

    #[test]
    fn parses_units_case_insensitively() {
        assert_eq!(parse_bytes("1kib").unwrap(), KIB);
        assert_eq!(parse_bytes("1 GB").unwrap(), GIB);
    }

    #[test]
    fn parses_fractional_values() {
        assert_eq!(parse_bytes("1.5 MiB").unwrap(), 1_572_864);
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(parse_bytes("   "), Err(ParseSizeError::Empty));
    }

    #[test]
    fn rejects_unknown_unit() {
        assert_eq!(
            parse_bytes("5 XB"),
            Err(ParseSizeError::UnknownUnit("XB".to_string()))
        );
    }

    #[test]
    fn rejects_invalid_number() {
        assert!(matches!(
            parse_bytes("abc KB"),
            Err(ParseSizeError::InvalidNumber(_))
        ));
    }
}
