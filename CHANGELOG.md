# Changelog

All notable changes to nmea-kit are documented here.

## [0.3.1] — 2026-04-11

### Added
- THS (True Heading and Status) sentence type
- TXT (Text Transmission) sentence type
- `ddmm_to_decimal` / `decimal_to_ddmm` coordinate conversion helpers
- CONTRIBUTING.md, CHANGELOG.md, `.githooks/pre-commit`

## [0.3.0] — 2026-04-10

### Added
- AIS message types 4 (Base Station Report) and 27 (Long Range / Satellite AIS)
- Shared bit-extraction helpers extracted to `ais/messages/utils.rs`
- AIS message type coverage added to SENTENCES.md and README.md

### Changed
- Tests reorganized into `tests/nmea/` and `tests/ais/` subdirectories

## [0.2.3] — 2026-04-09

### Added
- RSA (Rudder Sensor Angle) and VLW (Distance Traveled through Water) sentence types

### Changed
- Dispatch macro refactored; integration tests cleaned up

## [0.2.2] — 2026-04-09

### Fixed
- Review findings: doc corrections, test naming, minor API clean-up

## [0.2.1] — 2026-04-09

### Added
- MTW (Mean Temperature of Water) sentence type
- VBW (Dual Ground/Water Speed) sentence type

## [0.2.0] — 2026-04-09

### Changed
- Introduced `NmeaEncodable` trait with a `to_sentence()` default method — replaces the previous per-struct boilerplate

## [0.1.5] — 2026-04-08

### Added
- XDR (Transducer Measurement) sentence type with `to_sentences()` for multi-transducer payloads

## [0.1.4] — 2026-04-08

### Added
- DBK (Depth Below Keel) sentence type
- ZDA (Time & Date) sentence type

## [0.1.3] — 2026-04-08

### Fixed
- `lib.rs` feature gates, test naming conventions, Wire-format doc corrections

## [0.1.2] — 2026-04-07

### Added
- GBS (GPS Satellite Fault Detection) sentence type
- GST (GPS Pseudorange Noise Statistics) sentence type

## [0.1.1] — 2026-04-07

### Added
- ROT (Rate of Turn) sentence type
- RMB (Recommended Minimum Navigation Information) sentence type
- Revamped integration test structure

## [0.1.0] — 2026-04-07

### Added
- Initial release
- NMEA 0183 bidirectional parser/encoder: 18 sentence types (DBT, DBS, DPT, GGA, GLL, GNS, GST, HDG, HDM, HDT, MTW, MWD, MWV, RMC, VHW, VTG, VLW, ZDA)
- AIS decoder: Types 1/2/3 (Class A), 5 (Voyage), 14 (Safety Broadcast), 18 (Class B), 19 (Class B+), 21 (AtoN), 24 (Class B Static)
- Shared frame layer: `parse_frame` / `encode_frame`, IEC 61162-450 tag block support
- Zero external dependencies
- CI: tests, clippy, rustfmt, doc checks on stable + MSRV 1.85.0

[0.3.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.3.1
[0.3.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.3.0
[0.2.3]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.2.3
[0.2.2]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.2.2
[0.2.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.2.1
[0.2.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.2.0
[0.1.5]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.5
[0.1.4]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.4
[0.1.3]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.3
[0.1.2]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.2
[0.1.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.1
[0.1.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.0
