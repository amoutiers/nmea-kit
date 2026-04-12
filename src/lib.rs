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
//! ## Public API
//!
//! - [`parse_frame`] / [`encode_frame`] — frame layer (always available)
//! - [`NmeaSentence`] — dispatch enum for all typed NMEA sentences
//! - [`NmeaEncodable`] — trait for encoding NMEA sentences to wire format
//! - [`ais`] — AIS decoder (behind `ais` feature)
//!
//! ## Features
//!
//! - `nmea` (default) — all 30 NMEA sentence types
//! - `ais` (default) — 16 AIS message types (read-only decode)
//! - `ais` (default) — AIS message decoding
//! - `dbs`, `dbt`, `dpt`, … — individual sentence types

mod error;
mod frame;

#[cfg(any(
    feature = "nmea",
    feature = "dbk",
    feature = "dbs",
    feature = "dbt",
    feature = "dpt",
    feature = "gbs",
    feature = "gga",
    feature = "gll",
    feature = "gns",
    feature = "gst",
    feature = "hdg",
    feature = "hdm",
    feature = "hdt",
    feature = "mtw",
    feature = "mwd",
    feature = "mwv",
    feature = "pashr",
    feature = "pgrme",
    feature = "pskpdpt",
    feature = "rmb",
    feature = "rmc",
    feature = "rot",
    feature = "rsa",
    feature = "ths",
    feature = "txt",
    feature = "vbw",
    feature = "vhw",
    feature = "vlw",
    feature = "vtg",
    feature = "xdr",
    feature = "zda",
))]
pub mod nmea;

#[cfg(feature = "ais")]
pub mod ais;

pub use error::*;
pub use frame::*;

#[cfg(any(
    feature = "nmea",
    feature = "dbk",
    feature = "dbs",
    feature = "dbt",
    feature = "dpt",
    feature = "gbs",
    feature = "gga",
    feature = "gll",
    feature = "gns",
    feature = "gst",
    feature = "hdg",
    feature = "hdm",
    feature = "hdt",
    feature = "mtw",
    feature = "mwd",
    feature = "mwv",
    feature = "pashr",
    feature = "pgrme",
    feature = "pskpdpt",
    feature = "rmb",
    feature = "rmc",
    feature = "rot",
    feature = "rsa",
    feature = "ths",
    feature = "txt",
    feature = "vbw",
    feature = "vhw",
    feature = "vlw",
    feature = "vtg",
    feature = "xdr",
    feature = "zda",
))]
pub use nmea::NmeaSentence;

#[cfg(any(
    feature = "nmea",
    feature = "dbk",
    feature = "dbs",
    feature = "dbt",
    feature = "dpt",
    feature = "gbs",
    feature = "gga",
    feature = "gll",
    feature = "gns",
    feature = "gst",
    feature = "hdg",
    feature = "hdm",
    feature = "hdt",
    feature = "mtw",
    feature = "mwd",
    feature = "mwv",
    feature = "pashr",
    feature = "pgrme",
    feature = "pskpdpt",
    feature = "rmb",
    feature = "rmc",
    feature = "rot",
    feature = "rsa",
    feature = "ths",
    feature = "txt",
    feature = "vbw",
    feature = "vhw",
    feature = "vlw",
    feature = "vtg",
    feature = "xdr",
    feature = "zda",
))]
pub use nmea::NmeaEncodable;
