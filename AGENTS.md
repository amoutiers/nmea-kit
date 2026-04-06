# nmea-kit

Bidirectional NMEA 0183 parser/encoder + AIS decoder. Standalone crate, MIT/Apache-2.0.

## Structure

```
src/
├── lib.rs              # public API: parse_frame, encode_frame, NmeaSentence
├── frame.rs            # frame layer: checksum, tag blocks, $ and ! prefix
├── error.rs            # FrameError enum
├── nmea/
│   ├── mod.rs          # NmeaSentence dispatch enum (14 variants + Unknown)
│   ├── field.rs        # FieldReader (parse) + FieldWriter (encode) helpers
│   └── sentences/      # one file per sentence type (14 files, each feature-gated)
│       ├── mod.rs      # #[cfg(feature = "xyz")] mod/pub use per sentence
│       ├── mwd.rs      # example: struct Mwd { parse(), encode(), to_sentence() }
│       └── ...
└── ais/
    ├── mod.rs          # AisParser + AisMessage enum
    ├── armor.rs        # 6-bit ASCII armor decode + bit extraction (manual Vec<u8>)
    ├── fragments.rs    # multi-fragment reassembly (10 slots)
    └── messages/       # one file per AIS message type
        ├── common.rs       # NavigationStatus, AisClass
        ├── position_a.rs   # Types 1/2/3 — Class A position
        ├── position_b.rs   # Type 18 — Class B position
        ├── position_b_ext.rs # Type 19 — Class B+ extended
        ├── voyage_a.rs     # Type 5 — static & voyage data
        └── static_b.rs     # Type 24 — Class B static data
```

## Supported sentence types

14 NMEA sentence types, each behind its own feature flag:
DBS, DBT, DPT, GGA, GLL, GNS, HDG, HDM, HDT, MWD, MWV, RMC, VHW, VTG

The `nmea` umbrella feature enables all 14. Individual features can be cherry-picked:
```toml
# Only RMC and MWD, nothing else
nmea-kit = { version = "0.1", default-features = false, features = ["rmc", "mwd"] }
```

## Key patterns

### NMEA sentence implementation

Every sentence type follows the same pattern using `FieldReader`/`FieldWriter`:

- `SENTENCE_TYPE` — 3-char const (`"MWD"`, `"RMC"`, etc.)
- `parse(fields: &[&str]) -> Option<Self>` — sequential field reading (always returns `Some`, lenient)
- `encode(&self) -> Vec<String>` — sequential field writing
- `to_sentence(&self, talker: &str) -> String` — full sentence with checksum

Fixed indicator fields (T, M, N, K, f, F) are handled with `r.skip()` on parse and `w.fixed('T')` on encode.

**Design note**: `parse()` always returns `Some` for known types. Missing or malformed fields become `None` values in the struct. This is intentional for marine instruments that often produce partial data. The `try_parse!` macro in `nmea/mod.rs` has a fallback-to-Unknown path but it's currently unreachable.

### Adding a sentence type

1. Create `src/nmea/sentences/xyz.rs` with struct + impl + tests
2. Add `xyz = []` to `[features]` in `Cargo.toml`, and add `"xyz"` to the `nmea` feature list
3. Add `#[cfg(feature = "xyz")] mod xyz;` and `#[cfg(feature = "xyz")] pub use xyz::*;` to `sentences/mod.rs`
4. Add `#[cfg(feature = "xyz")] Xyz(sentences::Xyz)` variant to `NmeaSentence` enum in `nmea/mod.rs`
5. Add `#[cfg(feature = "xyz")] "XYZ" => try_parse!(sentences::Xyz::parse, Xyz)` to the dispatch match
6. Add `feature = "xyz"` to the `any(...)` gate in `lib.rs`

### AIS message implementation

AIS types use bit-level extraction from decoded 6-bit armor:
- `extract_u32(bits, offset, len)` / `extract_i32(bits, offset, len)` for numeric fields
- `extract_string(bits, offset, num_chars)` for AIS 6-bit text
- Sentinel values (91/181 lat/lon, 511 heading, 1023 SOG, 3600 COG) filtered to `None`
- All lat/lon use `f64` (not `f32`)
- Bit extraction uses manual `Vec<u8>` (one byte per bit), not `bitvec`

Helper functions in `position_a.rs` are `pub(crate)` — shared by `position_b.rs` and `position_b_ext.rs`.

## Field definitions reference

Sentence field layouts are sourced from [pynmeagps](https://github.com/semuconsulting/pynmeagps) (`nmeatypes_get.py`). Test fixtures from [SignalK](https://github.com/SignalK/signalk-parser-nmea0183) and [GPSD](https://gitlab.com/gpsd/gpsd).

## Commands

From the workspace root:

```sh
cargo test -p nmea-kit       # run all tests
cargo doc -p nmea-kit        # generate docs
cargo clippy -p nmea-kit     # lint check
```

## TDD workflow

Tests come first. Every change follows this cycle:

1. **Write the test** — use a real NMEA/AIS fixture from SignalK, GPSD, or pynmeagps. Never invent sentences — use documented wire formats with verified checksums.
2. **Watch it fail** — `cargo test -p nmea-kit` must show the new test failing for the right reason (missing struct, wrong value, etc.).
3. **Write the minimum code** to make it pass.
4. **Roundtrip test** — for every NMEA sentence: construct a struct, call `to_sentence()`, re-parse with `parse_frame()` + `Type::parse()`, assert fields match. This catches encode/decode asymmetry.
5. **Run full suite** — `cargo test -p nmea-kit` must pass with zero failures before moving on. Never leave broken tests behind.

### Test fixture sources (in order of preference)

1. **SignalK** — `github.com/SignalK/signalk-parser-nmea0183/test/` — real fixtures with edge cases (empty fields, invalid units, lowercase indicators)
2. **GPSD** — `gitlab.com/gpsd/gpsd/test/daemon/` — real device captures (Garmin, Humminbird, Saab, pypilot)
3. **pynmeagps** — `github.com/semuconsulting/pynmeagps` — canonical field definitions for struct design
4. **Synthetic** — only when no real fixture exists. Compute checksum manually: XOR all bytes between `$`/`!` (exclusive) and `*` (exclusive), format as 2-digit uppercase hex.

### What each sentence file must contain

```rust
// 1. Struct + impl (parse, encode, to_sentence)
// 2. #[cfg(test)] mod tests with:
//    - At least one real fixture parse test
//    - One roundtrip test (construct -> encode -> re-parse -> assert equal)
//    - Edge cases if fixtures exist (empty fields, partial data)
```

## Known design issues

- **`to_sentence()` boilerplate** — identical 4-line method duplicated across all 14 files. Should become a trait with a default impl.
- **Lat/lon format inconsistency** — NMEA sentences store raw `DDMM.MMMM`, AIS converts to decimal degrees. No conversion helper exists.
- **`FieldWriter::f32` formatting** — `format!("{v}")` may drop trailing `.0` or leading zeros vs original wire format.
- **`serde` feature** — declared but no struct has `#[derive(Serialize, Deserialize)]`. Needs implementation or removal.

## Constraints

- No `nom`, no proc-macro, no `syn`/`quote` — keep compile times minimal
- `serde` is the only optional dependency
- AIS is read-only — encoding AIS would go behind an `ais-encode` feature flag (not yet implemented)
- Edition 2024 — `macro_rules!` TT muncher doesn't work due to hygiene; use `FieldReader`/`FieldWriter` instead
