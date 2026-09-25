# human-units

Config files, CLI flags, and log lines are full of strings like `10MB`,
`1.5GiB`, and `1h30m`. Every project that reads them ends up writing its
own ad hoc parser, usually with a slightly different idea of whether `KB`
means 1000 or 1024 bytes. This crate is a small, dependency-free place to
put that logic once.

It has two modules:

- `human_units::bytes` — format a byte count as a string, or parse a size
  string back into a byte count.
- `human_units::duration` — format a `Duration` as a compact string, or
  parse a compound duration string back into a `Duration`.

All units are binary (1024-based): `KB` and `KiB` are treated the same
way. That matches what most tools in practice mean when they print `KB`,
even though it is not what the SI prefix technically says.

## Usage

```rust
use human_units::{format_bytes, parse_bytes, format_duration, parse_duration};
use std::time::Duration;

fn main() {
    assert_eq!(format_bytes(1_572_864), "1.50 MiB");
    assert_eq!(parse_bytes("1.5 MiB").unwrap(), 1_572_864);

    assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    assert_eq!(parse_duration("1h1m1s").unwrap(), Duration::from_secs(3661));
}
```

Every public function is pure: no clock reads, no I/O, no global state.
Same input always gives the same output, which makes them straightforward
to unit test and safe to call from anywhere.

## Status

Early. Byte parsing supports `B`, `KB`/`KiB`, `MB`/`MiB`, `GB`/`GiB`, and
`TB`/`TiB`. Duration parsing supports `ms`, `s`, `m`, `h`, `d`, and `w`.
See the issues for what is planned next.

## License

MIT, see [LICENSE](LICENSE).
