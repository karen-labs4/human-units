//! Human-readable formatting and parsing for byte sizes and durations.
//!
//! Every public function here is pure: given the same input it always
//! returns the same output, and none of them touch the clock, the
//! filesystem, or any other ambient state. That makes them safe to call
//! from config loaders, CLI argument parsers, or log formatters without
//! worrying about side effects, and it makes them trivial to unit test.

pub mod bytes;
pub mod duration;

pub use bytes::{format_bytes, parse_bytes, ParseSizeError};
pub use duration::{format_duration, parse_duration, ParseDurationError};
