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
//! - `nmea` (default) — all 38 NMEA sentence types
//! - `ais` (default) — 16 AIS message types (read-only decode)
//! - `dbs`, `dbt`, `dpt`, … — individual sentence types

mod error;
mod frame;

// Single source of truth for the "any NMEA sentence feature is active" predicate.
// Add new sentence feature names here when wiring a new type.
macro_rules! nmea_item {
    ($item:item) => {
        #[cfg(any(
            feature = "nmea",
            feature = "apb",
            feature = "bod",
            feature = "bwc",
            feature = "bwr",
            feature = "dbk",
            feature = "dbs",
            feature = "dbt",
            feature = "dpt",
            feature = "dtm",
            feature = "gbs",
            feature = "gga",
            feature = "gll",
            feature = "gns",
            feature = "gsa",
            feature = "gsv",
            feature = "gst",
            feature = "hdg",
            feature = "hdm",
            feature = "hdt",
            feature = "mda",
            feature = "mtw",
            feature = "mwd",
            feature = "mwv",
            feature = "pashr",
            feature = "pgrme",
            feature = "pskpdpt",
            feature = "rmb",
            feature = "rmc",
            feature = "rot",
            feature = "rpm",
            feature = "rsa",
            feature = "rsd",
            feature = "ths",
            feature = "txt",
            feature = "vbw",
            feature = "vdr",
            feature = "vhw",
            feature = "vlw",
            feature = "vtg",
            feature = "vwr",
            feature = "wpl",
            feature = "xdr",
            feature = "xte",
            feature = "zda",
        ))]
        $item
    };
}

nmea_item! { pub mod nmea; }

#[cfg(feature = "ais")]
pub mod ais;

pub use error::*;
pub use frame::*;

nmea_item! { pub use nmea::NmeaSentence; }
nmea_item! { pub use nmea::NmeaEncodable; }
