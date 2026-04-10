# Contributing to nmea-kit

## Getting started

```sh
git clone https://github.com/amoutiers/nmea-kit
cd nmea-kit
git config core.hooksPath .githooks   # enable pre-commit checks (fmt + clippy)
cargo test --all-features             # verify everything passes
```

## TDD workflow

Tests come first. Every change follows this cycle:

1. **Write a failing test** — use a real NMEA/AIS fixture from SignalK, GPSD, or pynmeagps. Never invent sentences — use documented wire formats with verified checksums.
2. **Watch it fail** — `cargo test --all-features` must show the new test failing for the right reason (missing struct, wrong value, etc.).
3. **Write the minimum code** to make it pass.
4. **Add a roundtrip test** — for every NMEA sentence: construct a struct, call `to_sentence()`, re-parse with `parse_frame()` + `Type::parse()`, assert fields match. This catches encode/decode asymmetry.
5. **Run the full suite** before committing:
   ```sh
   cargo fmt
   cargo clippy --all-features --all-targets -- -D warnings
   cargo test --all-features
   ```
   CI enforces all three — a commit that fails any of them blocks the publish workflow.

## Adding a NMEA sentence type

1. Create `src/nmea/sentences/xyz.rs` with struct + `parse()` + `NmeaEncodable` impl + `#[cfg(test)]` block
2. Add `xyz = []` to `[features]` in `Cargo.toml`, and add `"xyz"` to the `nmea` feature list
3. Add `#[cfg(feature = "xyz")] mod xyz;` and `#[cfg(feature = "xyz")] pub use xyz::*;` to `sentences/mod.rs`
4. Add `#[cfg(feature = "xyz")] Xyz(sentences::Xyz)` variant to `NmeaSentence` enum in `nmea/mod.rs`
5. Add `#[cfg(feature = "xyz")] "XYZ" => try_parse!(sentences::Xyz::parse, Xyz)` to the dispatch match
6. Add `feature = "xyz"` to the `any(...)` gate in `lib.rs`
7. Create `tests/nmea/xyz.rs` with `dispatch`, `decode_encode`, and `roundtrip` tests

## What each sentence file must contain

**Unit tests** in `src/nmea/sentences/{type}.rs`:

```rust
#[cfg(test)] mod tests {
    // Required:
    //   {type}_empty           — parse sentence with all empty fields, assert all None
    //   {type}_encode_roundtrip — construct → to_sentence() → parse_frame → parse → assert_eq
    // At least one fixture test:
    //   {type}_{description}_{source} — parse a real sentence, assert key fields
}
```

**Integration tests** in `tests/nmea/{type}.rs`:

```rust
#![cfg(feature = "{type}")]
// Required:
//   dispatch      — NmeaSentence::parse() returns the correct variant
//   decode_encode — parse raw sentence → encode → re-parse → assert_eq
//   roundtrip     — construct struct → to_sentence() → parse → assert_eq
```

## Test naming conventions

- `{type}_empty` — all fields None (no source suffix)
- `{type}_encode_roundtrip` — construct → encode → re-parse → assert equal
- `{type}_{description}_signalk` — from SignalK fixtures
- `{type}_{description}_gpsd` — from GPSD fixtures
- `{type}_{description}_pynmeagps` — from pynmeagps fixtures
- `{type}_{description}_gonmea` — from go-nmea fixtures
- Source suffix is always at the end of the test name
- Tests within each `mod tests` block are sorted alphabetically

## Test fixture sources (in order of preference)

1. **SignalK** — `external_fixtures/signalk-parser-nmea0183/` — real device fixtures with edge cases
2. **GPSD** — `gitlab.com/gpsd/gpsd/test/daemon/` — real device captures (Garmin, Humminbird, Saab, pypilot)
3. **pynmeagps** — `external_fixtures/pynmeagps/` — canonical field definitions for struct design
4. **go-nmea** — `external_fixtures/go-nmea/` — 80+ sentence types with test fixtures
5. **Synthetic** — only when no real fixture exists. Compute checksum manually: XOR all bytes between `$`/`!` (exclusive) and `*` (exclusive), format as 2-digit uppercase hex.

## Test rules

- Every sentence must have an `_empty` test and an `_encode_roundtrip` test
- Every fixture test must have a source suffix (`_signalk`, `_gpsd`, `_pynmeagps`, `_gonmea`)
- Use `assert_eq!(original, parsed)` for roundtrip tests — not field-by-field comparison
- Use `expect("description")` instead of `unwrap()` (clippy enforces this)

## Code style

- Zero external dependencies — no `nom`, no proc-macro, no `serde`
- No `unwrap()` in library code — `expect("description")` in tests only
- No `panic!`, `todo!`, or `#[allow(dead_code)]` in `src/`
- Rust edition 2024, MSRV 1.85.0
- AIS is read-only — encoding would go behind an `ais-encode` feature (not yet implemented)

## Known design issues

- **`FieldWriter::f32` formatting** — `format!("{v}")` may drop trailing `.0` or leading zeros vs original wire format (checksums still match).

## Reporting issues

Open an issue on GitHub. Include the raw NMEA/AIS sentence that triggered the problem.
