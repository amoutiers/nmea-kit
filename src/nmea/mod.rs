//! NMEA 0183 sentence parsing and encoding.

mod field;
pub mod sentences;

pub use field::*;
pub use sentences::*;

use crate::NmeaFrame;

macro_rules! nmea_sentences {
    (
        standard: [ $( [$feat:literal, $variant:ident, $wire:literal] ),* $(,)? ],
        proprietary: [ $( [$pfeat:literal, $pvariant:ident, $pwire:literal] ),* $(,)? ]
    ) => {
        /// Unified enum covering all supported NMEA 0183 sentence types.
        ///
        /// Use `NmeaSentence::parse(&frame)` to dispatch a parsed frame to the
        /// appropriate typed struct. Unknown sentence types are captured in the
        /// `Unknown` variant.
        ///
        /// Proprietary sentences (`$P...`) are dispatched separately from standard
        /// sentences, so there is no risk of collision between e.g. `$PSKPDPT` and
        /// standard `$IIDPT`.
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq)]
        pub enum NmeaSentence {
            $(
                #[cfg(feature = $feat)]
                $variant(sentences::$variant),
            )*
            $(
                #[cfg(feature = $pfeat)]
                $pvariant(sentences::$pvariant),
            )*
            Unknown {
                sentence_type: String,
                fields: Vec<String>,
            },
        }

        impl NmeaSentence {
            /// Parse a frame into a typed sentence variant.
            ///
            /// Standard sentences are dispatched on `frame.sentence_type` (3-char code).
            /// Proprietary sentences (where `frame.talker` is empty) are dispatched on
            /// the full address in `frame.sentence_type` (e.g. `"PASHR"`, `"PSKPDPT"`).
            ///
            /// Returns `Unknown` for unrecognized types.
            pub fn parse(frame: &NmeaFrame<'_>) -> Self {
                macro_rules! try_parse {
                    ($parser:expr, $v:ident) => {
                        match $parser(&frame.fields) {
                            Some(v) => Self::$v(v),
                            None => unreachable!("parse() always returns Some for known sentence types"),
                        }
                    };
                }

                if frame.talker.is_empty() {
                    // Proprietary path: sentence_type is the full address
                    match frame.sentence_type {
                        $(
                            #[cfg(feature = $pfeat)]
                            $pwire => try_parse!(sentences::$pvariant::parse, $pvariant),
                        )*
                        _ => Self::from_frame(frame),
                    }
                } else {
                    // Standard path: sentence_type is the 3-char code
                    match frame.sentence_type {
                        $(
                            #[cfg(feature = $feat)]
                            $wire => try_parse!(sentences::$variant::parse, $variant),
                        )*
                        _ => Self::from_frame(frame),
                    }
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
    standard: [
        // Position
        ["dtm", Dtm, "DTM"],
        ["rmc", Rmc, "RMC"],
        // Satellites
        ["gbs", Gbs, "GBS"],
        ["gsa", Gsa, "GSA"],
        ["gsv", Gsv, "GSV"],
        ["gst", Gst, "GST"],
        ["gga", Gga, "GGA"],
        ["gll", Gll, "GLL"],
        ["gns", Gns, "GNS"],
        // Wind
        ["mwd", Mwd, "MWD"],
        ["mwv", Mwv, "MWV"],
        ["vwr", Vwr, "VWR"],
        // Heading
        ["hdt", Hdt, "HDT"],
        ["hdg", Hdg, "HDG"],
        ["hdm", Hdm, "HDM"],
        ["rot", Rot, "ROT"],
        ["ths", Ths, "THS"],
        // Navigation
        ["apb", Apb, "APB"],
        ["bwc", Bwc, "BWC"],
        ["bwr", Bwr, "BWR"],
        ["rmb", Rmb, "RMB"],
        ["wpl", Wpl, "WPL"],
        ["xte", Xte, "XTE"],
        // Rudder
        ["rsa", Rsa, "RSA"],
        // Speed
        ["vbw", Vbw, "VBW"],
        ["vlw", Vlw, "VLW"],
        ["vtg", Vtg, "VTG"],
        ["vhw", Vhw, "VHW"],
        ["rpm", Rpm, "RPM"],
        ["vdr", Vdr, "VDR"],
        // Depth
        ["dpt", Dpt, "DPT"],
        ["dbt", Dbt, "DBT"],
        ["dbs", Dbs, "DBS"],
        ["dbk", Dbk, "DBK"],
        // Environment
        ["mda", Mda, "MDA"],
        ["mtw", Mtw, "MTW"],
        ["xdr", Xdr, "XDR"],
        // Communication
        ["txt", Txt, "TXT"],
        // Time
        ["zda", Zda, "ZDA"],
    ],
    proprietary: [
        // Proprietary — Ashtech/Trimble
        ["pashr", Pashr, "PASHR"],
        // Proprietary — Garmin
        ["pgrme", Pgrme, "PGRME"],
        // Proprietary — Skipper
        ["pskpdpt", Pskpdpt, "PSKPDPT"],
    ]
];
