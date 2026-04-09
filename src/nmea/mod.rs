//! NMEA 0183 sentence parsing and encoding.

mod field;
pub mod sentences;

pub use field::*;
pub use sentences::*;

use crate::NmeaFrame;

macro_rules! nmea_sentences {
    ( $( [$feat:literal, $variant:ident, $wire:literal] ),* $(,)? ) => {
        /// Unified enum covering all supported NMEA 0183 sentence types.
        ///
        /// Use `NmeaSentence::parse(&frame)` to dispatch a parsed frame to the
        /// appropriate typed struct. Unknown sentence types are captured in the
        /// `Unknown` variant.
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq)]
        pub enum NmeaSentence {
            $(
                #[cfg(feature = $feat)]
                $variant(sentences::$variant),
            )*
            Unknown {
                sentence_type: String,
                fields: Vec<String>,
            },
        }

        impl NmeaSentence {
            /// Parse a frame into a typed sentence variant.
            ///
            /// Dispatches on `frame.sentence_type`. Returns `Unknown` for unrecognized
            /// types **and** for recognized types that fail to parse (preserving the
            /// sentence_type and raw fields for diagnostics).
            pub fn parse(frame: &NmeaFrame<'_>) -> Self {
                macro_rules! try_parse {
                    ($parser:expr, $v:ident) => {
                        match $parser(&frame.fields) {
                            Some(v) => Self::$v(v),
                            None => unreachable!("parse() always returns Some for known sentence types"),
                        }
                    };
                }
                match frame.sentence_type {
                    $(
                        #[cfg(feature = $feat)]
                        $wire => try_parse!(sentences::$variant::parse, $variant),
                    )*
                    _ => Self::from_frame(frame),
                }
            }

            /// Build an `Unknown` variant preserving the frame's sentence type and fields.
            fn from_frame(frame: &NmeaFrame<'_>) -> Self {
                Self::Unknown {
                    sentence_type: frame.sentence_type.to_string(),
                    fields: frame.fields.iter().map(|f| f.to_string()).collect(),
                }
            }
        }
    };
}

nmea_sentences![
    // Position
    ["rmc", Rmc, "RMC"],
    // Satellites
    ["gbs", Gbs, "GBS"],
    ["gst", Gst, "GST"],
    ["gga", Gga, "GGA"],
    ["gll", Gll, "GLL"],
    ["gns", Gns, "GNS"],
    // Wind
    ["mwd", Mwd, "MWD"],
    ["mwv", Mwv, "MWV"],
    // Heading
    ["hdt", Hdt, "HDT"],
    ["hdg", Hdg, "HDG"],
    ["hdm", Hdm, "HDM"],
    ["rot", Rot, "ROT"],
    // Navigation
    ["rmb", Rmb, "RMB"],
    // Rudder
    ["rsa", Rsa, "RSA"],
    // Speed
    ["vbw", Vbw, "VBW"],
    ["vlw", Vlw, "VLW"],
    ["vtg", Vtg, "VTG"],
    ["vhw", Vhw, "VHW"],
    // Depth
    ["dpt", Dpt, "DPT"],
    ["dbt", Dbt, "DBT"],
    ["dbs", Dbs, "DBS"],
    ["dbk", Dbk, "DBK"],
    // Environment
    ["mtw", Mtw, "MTW"],
    ["xdr", Xdr, "XDR"],
    // Time
    ["zda", Zda, "ZDA"],
];
