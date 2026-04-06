//! # nmea-kit
//!
//! Bidirectional NMEA 0183 parser/encoder with AIS message decoding.
//!
//! ## Architecture
//!
//! ```text
//! raw line ──→ parse_frame() ──→ NmeaFrame { prefix, talker, sentence_type, fields }
//!                                     │
//!                      ┌──────────────┼──────────────┐
//!                      ▼              ▼               ▼
//!                $ + known      $ + unknown     ! (AIVDM/AIVDO)
//!                      │              │               │
//!                      ▼              ▼               ▼
//!               Typed struct    Raw fields      AisMessage enum
//! ```
//!
//! ## Features
//!
//! - `nmea` (default) — all 14 NMEA sentence types
//! - `ais` (default) — AIS message decoding
//! - `dbs`, `dbt`, `dpt`, … — individual sentence types

mod error;
mod frame;

#[cfg(any(
    feature = "nmea",
    feature = "dbs",
    feature = "dbt",
    feature = "dpt",
    feature = "gga",
    feature = "gll",
    feature = "gns",
    feature = "hdg",
    feature = "hdm",
    feature = "hdt",
    feature = "mwd",
    feature = "mwv",
    feature = "rmc",
    feature = "vhw",
    feature = "vtg",
))]
pub mod nmea;

#[cfg(feature = "ais")]
pub mod ais;

pub use error::*;
pub use frame::*;

#[cfg(any(
    feature = "nmea",
    feature = "dbs",
    feature = "dbt",
    feature = "dpt",
    feature = "gga",
    feature = "gll",
    feature = "gns",
    feature = "hdg",
    feature = "hdm",
    feature = "hdt",
    feature = "mwd",
    feature = "mwv",
    feature = "rmc",
    feature = "vhw",
    feature = "vtg",
))]
pub use nmea::NmeaSentence;
