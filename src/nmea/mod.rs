//! NMEA 0183 sentence parsing and encoding.

mod field;
pub mod sentences;

pub use field::*;
pub use sentences::*;

use crate::NmeaFrame;

/// Unified enum covering all supported NMEA 0183 sentence types.
///
/// Use `NmeaSentence::parse(&frame)` to dispatch a parsed frame to the
/// appropriate typed struct. Unknown sentence types are captured in the
/// `Unknown` variant.
#[derive(Debug, Clone, PartialEq)]
pub enum NmeaSentence {
    // Position
    #[cfg(feature = "rmc")]
    Rmc(sentences::Rmc),
    #[cfg(feature = "gga")]
    Gga(sentences::Gga),
    #[cfg(feature = "gll")]
    Gll(sentences::Gll),
    #[cfg(feature = "gns")]
    Gns(sentences::Gns),
    // Wind
    #[cfg(feature = "mwd")]
    Mwd(sentences::Mwd),
    #[cfg(feature = "mwv")]
    Mwv(sentences::Mwv),
    // Heading
    #[cfg(feature = "hdt")]
    Hdt(sentences::Hdt),
    #[cfg(feature = "hdg")]
    Hdg(sentences::Hdg),
    #[cfg(feature = "hdm")]
    Hdm(sentences::Hdm),
    #[cfg(feature = "rot")]
    Rot(sentences::Rot),
    // Navigation
    #[cfg(feature = "rmb")]
    Rmb(sentences::Rmb),
    // Speed
    #[cfg(feature = "vtg")]
    Vtg(sentences::Vtg),
    #[cfg(feature = "vhw")]
    Vhw(sentences::Vhw),
    // Depth
    #[cfg(feature = "dpt")]
    Dpt(sentences::Dpt),
    #[cfg(feature = "dbt")]
    Dbt(sentences::Dbt),
    #[cfg(feature = "dbs")]
    Dbs(sentences::Dbs),
    // Unknown
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
            ($parser:expr, $variant:ident) => {
                match $parser(&frame.fields) {
                    Some(v) => Self::$variant(v),
                    None => Self::from_frame(frame),
                }
            };
        }

        match frame.sentence_type {
            // Position
            #[cfg(feature = "rmc")]
            "RMC" => try_parse!(sentences::Rmc::parse, Rmc),
            #[cfg(feature = "gga")]
            "GGA" => try_parse!(sentences::Gga::parse, Gga),
            #[cfg(feature = "gll")]
            "GLL" => try_parse!(sentences::Gll::parse, Gll),
            #[cfg(feature = "gns")]
            "GNS" => try_parse!(sentences::Gns::parse, Gns),
            // Wind
            #[cfg(feature = "mwd")]
            "MWD" => try_parse!(sentences::Mwd::parse, Mwd),
            #[cfg(feature = "mwv")]
            "MWV" => try_parse!(sentences::Mwv::parse, Mwv),
            // Heading
            #[cfg(feature = "hdt")]
            "HDT" => try_parse!(sentences::Hdt::parse, Hdt),
            #[cfg(feature = "hdg")]
            "HDG" => try_parse!(sentences::Hdg::parse, Hdg),
            #[cfg(feature = "hdm")]
            "HDM" => try_parse!(sentences::Hdm::parse, Hdm),
            #[cfg(feature = "rot")]
            "ROT" => try_parse!(sentences::Rot::parse, Rot),
            // Navigation
            #[cfg(feature = "rmb")]
            "RMB" => try_parse!(sentences::Rmb::parse, Rmb),
            // Speed
            #[cfg(feature = "vtg")]
            "VTG" => try_parse!(sentences::Vtg::parse, Vtg),
            #[cfg(feature = "vhw")]
            "VHW" => try_parse!(sentences::Vhw::parse, Vhw),
            // Depth
            #[cfg(feature = "dpt")]
            "DPT" => try_parse!(sentences::Dpt::parse, Dpt),
            #[cfg(feature = "dbt")]
            "DBT" => try_parse!(sentences::Dbt::parse, Dbt),
            #[cfg(feature = "dbs")]
            "DBS" => try_parse!(sentences::Dbs::parse, Dbs),
            // Unknown
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

#[cfg(test)]
mod dispatch_tests {
    use super::*;
    use crate::parse_frame;

    #[cfg(feature = "dpt")]
    #[test]
    fn dispatch_dpt() {
        let frame = parse_frame("$IIDPT,4.1,0.0*45").expect("valid");
        let sentence = NmeaSentence::parse(&frame);
        assert!(matches!(sentence, NmeaSentence::Dpt(_)));
    }

    #[cfg(feature = "hdt")]
    #[test]
    fn dispatch_hdt() {
        let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid");
        let sentence = NmeaSentence::parse(&frame);
        assert!(matches!(sentence, NmeaSentence::Hdt(_)));
    }

    #[cfg(feature = "mwd")]
    #[test]
    fn dispatch_mwd() {
        let frame = parse_frame("$IIMWD,,,046.,M,10.1,N,05.2,M*0B").expect("valid");
        let sentence = NmeaSentence::parse(&frame);
        assert!(matches!(sentence, NmeaSentence::Mwd(_)));
    }

    #[test]
    fn dispatch_unknown() {
        let frame = parse_frame("$GPXYZ,1,2,3").expect("valid");
        let sentence = NmeaSentence::parse(&frame);
        match sentence {
            NmeaSentence::Unknown { sentence_type, .. } => assert_eq!(sentence_type, "XYZ"),
            _ => panic!("expected Unknown"),
        }
    }

    #[cfg(feature = "vtg")]
    #[test]
    fn dispatch_vtg() {
        let frame = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid");
        let sentence = NmeaSentence::parse(&frame);
        assert!(matches!(sentence, NmeaSentence::Vtg(_)));
    }
}
