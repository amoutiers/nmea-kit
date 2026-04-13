# Changelog

All notable changes to nmea-kit are documented here.

## [0.5.1] — 2026-04-13

### Added
- GSA (GPS DOP and Active Satellites) sentence type
- GSV (Satellites in View) sentence type

NMEA sentence coverage expanded from 30 to 32 types.

## [0.5.0] — 2026-04-12

### Added
- Proprietary sentence support — `parse_frame` now detects `$P...` addresses per NMEA 0183, setting `talker = ""` and `sentence_type` to the full address (e.g. `"PASHR"`, `"PSKPDPT"`). Standard and proprietary dispatch paths are separate, preventing collisions.
- `NmeaEncodable::PROPRIETARY_ID` constant and `to_proprietary_sentence()` method
- Two-path `nmea_sentences!` macro (`standard:` / `proprietary:` sections)
- PASHR — Roll, Pitch, Heading (Ashtech/Trimble proprietary)
- PGRME — Garmin Estimated Position Error (proprietary)
- PSKPDPT — Skipper Depth (proprietary)
- `FragmentCollector` payload size limits: `MAX_PAYLOAD_SIZE = 256` chars and `MAX_FRAGMENTS = 5`, grounded in ITU-R M.1371-5 (1152-bit / 5-slot TDMA ceiling)

### Fixed
- Removed `unwrap()` in RTE waypoint parsing loop (replaced with `while let`)

### Changed
- CONTRIBUTING.md documents proprietary sentence workflow

NMEA sentence coverage expanded from 27 to 30 types (27 standard + 3 proprietary).

## [0.4.0] — 2026-04-11

### Added
- AIS Type 6 — Addressed Binary Message
- AIS Type 7 / 13 — Binary / Safety Acknowledge
- AIS Type 8 — Binary Broadcast Message
- AIS Type 9 — Standard SAR Aircraft Position Report
- AIS Type 11 — UTC/Date Response
- AIS Type 12 — Addressed Safety-Related Message
- AIS Type 15 — Interrogation

AIS coverage expanded from 9 to 16 message types.

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

[0.5.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.1
[0.5.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.0
[0.4.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.4.0
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
