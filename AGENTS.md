# nmea-kit

Bidirectional NMEA 0183 parser/encoder + AIS decoder. Standalone crate, MIT/Apache-2.0.

For contribution workflow, test rules, and the sentence-type checklist see [CONTRIBUTING.md](CONTRIBUTING.md).

## Structure

```
src/
├── lib.rs              # public API: parse_frame, encode_frame, NmeaSentence
├── frame.rs            # frame layer: checksum, tag blocks, $ and ! prefix
├── error.rs            # FrameError enum
├── nmea/
│   ├── mod.rs          # NmeaSentence dispatch enum (27 variants + Unknown)
│   ├── field.rs        # FieldReader (parse) + FieldWriter (encode) + geo helpers
│   └── sentences/      # one file per sentence type (feature-gated)
│       ├── mod.rs      # #[cfg(feature = "xyz")] mod/pub use per sentence
│       └── mwd.rs      # example: struct Mwd { parse(), encode() }
└── ais/
    ├── mod.rs          # AisParser + AisMessage enum + reset()
    ├── armor.rs        # 6-bit ASCII armor decode + bit extraction (manual Vec<u8>)
    ├── fragments.rs    # multi-fragment reassembly (10 slots)
    └── messages/       # one file per AIS message type
        ├── common.rs       # NavigationStatus, AisClass
        ├── utils.rs        # shared extract_u32/i32/string helpers
        ├── position_a.rs   # Types 1/2/3 — Class A position
        ├── position_b.rs   # Type 18 — Class B position
        ├── position_b_ext.rs # Type 19 — Class B+ extended
        ├── voyage_a.rs     # Type 5 — static & voyage data
        ├── static_b.rs     # Type 24 — Class B static data
        ├── base_station.rs # Type 4 — base station report
        ├── aid_to_navigation.rs # Type 21 — AtoN
        ├── safety_broadcast.rs  # Type 14 — safety broadcast
        └── long_range.rs   # Type 27 — long range / satellite AIS

tests/
├── frame.rs            # frame-level integration tests
├── nmea.rs             # NMEA integration entry point
├── ais.rs              # AIS integration entry point
├── nmea/               # one file per sentence type
│   └── {type}.rs       # dispatch, decode_encode, roundtrip
└── ais/                # one file per AIS message type
    └── type{N}.rs
```

## Key patterns

### NMEA sentence implementation

Every sentence type follows the same pattern using `FieldReader`/`FieldWriter`:

- `SENTENCE_TYPE` — 3-char const (`"MWD"`, `"RMC"`, etc.)
- `parse(fields: &[&str]) -> Option<Self>` — sequential field reading (always returns `Some`, lenient)
- `encode(&self) -> Vec<String>` — sequential field writing
- `to_sentence(&self, talker: &str) -> String` — default impl on `NmeaEncodable` trait

Fixed indicator fields (T, M, N, K, f, F) are handled with `r.skip()` on parse and `w.fixed('T')` on encode.

**Design note**: `parse()` always returns `Some` for known types. Missing or malformed fields become `None`. This is intentional for marine instruments that often produce partial data.

### AIS message implementation

AIS types use bit-level extraction from decoded 6-bit armor:

- `extract_u32(bits, offset, len)` / `extract_i32(bits, offset, len)` for numeric fields
- `extract_string(bits, offset, num_chars)` for AIS 6-bit text
- Sentinel values (91/181 lat/lon, 511 heading, 1023 SOG, 3600 COG) filtered to `None`
- All lat/lon use `f64` (not `f32`), heading uses `u16` (integer degrees per AIS spec)
- Bit extraction uses manual `Vec<u8>` (one byte per bit), not `bitvec`
- `AisParser::reset()` clears in-progress fragment buffers

Helper functions in `position_a.rs` are `pub(crate)` — shared by `position_b.rs` and `position_b_ext.rs`.

## Field definitions reference

Sentence field layouts are sourced from [pynmeagps](https://github.com/semuconsulting/pynmeagps) (`nmeatypes_get.py`). Test fixtures from [SignalK](https://github.com/SignalK/signalk-parser-nmea0183) and [GPSD](https://gitlab.com/gpsd/gpsd). Full sentence coverage tracked in [SENTENCES.md](SENTENCES.md).

## Commands

```sh
cargo test --all-features                                # run all tests
cargo doc --all-features --open                          # browse docs locally
cargo clippy --all-features --all-targets -- -D warnings # lint
cargo fmt                                                # format
```

## Constraints

- No `nom`, no proc-macro, no `syn`/`quote` — keep compile times minimal
- Zero dependencies (serde was removed as unused)
- AIS is read-only — encoding AIS would go behind an `ais-encode` feature flag (not yet implemented)
- Edition 2024, MSRV 1.85.0
