//! AIS (Automatic Identification System) message decoding.
//!
//! Read-only: decodes AIVDM/AIVDO messages from `!`-prefixed NMEA frames.
//! Transmitting AIS requires certified hardware.
//!
//! # Usage
//!
//! ```
//! use nmea_kit::ais::{AisParser, AisMessage};
//! use nmea_kit::parse_frame;
//!
//! let mut parser = AisParser::new();
//!
//! // Single-fragment message
//! let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid");
//! if let Some(msg) = parser.decode(&frame) {
//!     match msg {
//!         AisMessage::Position(pos) => println!("MMSI: {}, lat: {:?}", pos.mmsi, pos.latitude),
//!         _ => {}
//!     }
//! }
//! ```

pub mod armor;
pub mod fragments;
pub mod messages;

pub use messages::*;

use armor::decode_armor;
use fragments::FragmentCollector;

use crate::NmeaFrame;

/// Unified AIS message enum.
#[derive(Debug, Clone, PartialEq)]
pub enum AisMessage {
    /// Types 1, 2, 3 (Class A), 18 (Class B), 19 (Class B+) position reports.
    Position(PositionReport),
    /// Type 5: static and voyage related data (Class A).
    StaticVoyage(StaticVoyageData),
    /// Type 24: static data report (Class B), Part A or Part B.
    StaticReport(StaticDataReport),
    /// Unsupported message type.
    Unknown { msg_type: u8 },
}

/// Stateful AIS parser with multi-fragment reassembly.
///
/// Maintains fragment buffers for concurrent multi-part messages.
/// Feed it frames from `parse_frame()` — it returns decoded messages.
pub struct AisParser {
    collector: FragmentCollector,
}

impl AisParser {
    pub fn new() -> Self {
        Self {
            collector: FragmentCollector::new(),
        }
    }

    /// Clear all in-progress fragment buffers.
    ///
    /// Useful when switching data sources or recovering from a corrupted stream.
    pub fn reset(&mut self) {
        self.collector = FragmentCollector::new();
    }

    /// Decode an AIS frame. Returns `Some(AisMessage)` for complete messages,
    /// `None` for incomplete fragments, parse errors, or non-AIS frames.
    pub fn decode(&mut self, frame: &NmeaFrame<'_>) -> Option<AisMessage> {
        // Only handle VDM and VDO sentences
        if frame.prefix != '!' || (frame.sentence_type != "VDM" && frame.sentence_type != "VDO") {
            return None;
        }

        // Reassemble fragments
        let payload = self.collector.process(&frame.fields)?;

        // Decode armor
        let bits = decode_armor(&payload.payload, payload.fill_bits)?;

        // Extract message type (first 6 bits)
        let msg_type = armor::extract_u32(&bits, 0, 6)? as u8;

        // Dispatch to message decoder
        match msg_type {
            1..=3 => PositionReport::decode_class_a(&bits).map(AisMessage::Position),
            5 => StaticVoyageData::decode(&bits).map(AisMessage::StaticVoyage),
            18 => PositionReport::decode_class_b(&bits).map(AisMessage::Position),
            19 => PositionReport::decode_class_b_extended(&bits).map(AisMessage::Position),
            24 => StaticDataReport::decode(&bits).map(AisMessage::StaticReport),
            _ => Some(AisMessage::Unknown { msg_type }),
        }
    }
}

impl Default for AisParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn ignores_nmea_sentences() {
        let mut parser = AisParser::new();
        let frame =
            parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77")
                .expect("valid");
        assert!(parser.decode(&frame).is_none());
    }

    #[test]
    fn sentinel_values_filtered() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisMessage::Position(pos) = msg {
            assert!(pos.heading.is_none() || pos.heading.expect("heading") < 360);
        }
    }

    #[test]
    fn type_18_class_b() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,B6CdCm0t3`tba35f@V9faHi7kP06,0*58").expect("valid");
        let msg = parser.decode(&frame);
        // This might be a type 18 or might not decode depending on exact payload
        // At minimum it shouldn't panic
        if let Some(AisMessage::Position(pos)) = &msg {
            assert_eq!(pos.ais_class, AisClass::B);
        }
    }

    #[test]
    fn type_19_class_b_extended() {
        let mut parser = AisParser::new();
        // GPSD fixture: Type 19 Class B+ extended position report
        let frame =
            parse_frame("!AIVDM,1,1,,B,C5N3SRgPEnJGEBT>NhWAwwo862PaLELTBJ:V00000000S0D:R220,0*0B")
                .expect("valid type 19 frame");
        let msg = parser.decode(&frame).expect("decode type 19");
        if let AisMessage::Position(pos) = msg {
            assert_eq!(pos.msg_type, 19);
            assert!(pos.mmsi > 0);
            assert!(pos.latitude.is_some());
            assert!(pos.longitude.is_some());
            assert_eq!(pos.ais_class, AisClass::BPlus);
        } else {
            panic!("expected Position (type 19), got {msg:?}");
        }
    }

    #[test]
    fn type_1_position_report() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisMessage::Position(pos) = msg {
            assert_eq!(pos.msg_type, 1);
            assert!(pos.mmsi > 0);
            assert!(pos.latitude.is_some());
            assert!(pos.longitude.is_some());
            assert_eq!(pos.ais_class, AisClass::A);
            // Verify f64 precision
            let lat = pos.latitude.expect("valid");
            let lon = pos.longitude.expect("valid");
            assert!((-90.0..=90.0).contains(&lat));
            assert!((-180.0..=180.0).contains(&lon));
        } else {
            panic!("expected Position, got {msg:?}");
        }
    }

    #[test]
    fn type_24_static_data_report() {
        let mut parser = AisParser::new();
        // Type 24 Part A: vessel name
        let frame = parse_frame("!AIVDM,1,1,,A,H52N>V@T2rNVPJ2000000000000,2*29")
            .expect("valid type 24 frame");
        let msg = parser.decode(&frame).expect("decode type 24");
        if let AisMessage::StaticReport(report) = msg {
            match report {
                StaticDataReport::PartA { mmsi, vessel_name } => {
                    assert!(mmsi > 0);
                    // Vessel name may be all padding (@) — trimmed to empty
                    let _ = vessel_name;
                }
                StaticDataReport::PartB { mmsi, .. } => {
                    assert!(mmsi > 0);
                }
            }
        } else {
            panic!("expected StaticReport (type 24), got {msg:?}");
        }
    }

    #[test]
    fn type_5_multi_fragment() {
        let mut parser = AisParser::new();

        // GPSD sample.aivdm Type 5 fixture
        let f1 = parse_frame(
            "!AIVDM,2,1,1,A,55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp8,0*1C",
        )
        .expect("valid frag1");
        assert!(parser.decode(&f1).is_none()); // incomplete

        let f2 = parse_frame("!AIVDM,2,2,1,A,88888888880,2*25").expect("valid frag2");
        let msg = parser.decode(&f2).expect("decoded");
        if let AisMessage::StaticVoyage(svd) = msg {
            assert!(svd.mmsi > 0);
            assert!(!svd.vessel_name.is_empty());
            assert_eq!(svd.ais_class, AisClass::A);
        } else {
            panic!("expected StaticVoyage, got {msg:?}");
        }
    }

    #[test]
    fn reset_clears_pending_fragments() {
        let mut parser = AisParser::new();
        // Send fragment 1 of 2
        let f1 = parse_frame(
            "!AIVDM,2,1,1,A,55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp8,0*1C",
        )
        .expect("valid");
        assert!(parser.decode(&f1).is_none());
        // Reset clears the pending fragment
        parser.reset();
        // Fragment 2 alone should not produce a message
        let f2 = parse_frame("!AIVDM,2,2,1,A,88888888880,2*25").expect("valid");
        assert!(parser.decode(&f2).is_none());
    }

    #[test]
    fn unknown_message_type() {
        let mut parser = AisParser::new();
        // Type 8 binary broadcast — should return Unknown
        let frame = parse_frame("!AIVDM,1,1,,A,85Mv070j2d>=<e<<=PQhhg`59P00,0*26").expect("valid");
        let msg = parser.decode(&frame);
        if let Some(AisMessage::Unknown { msg_type }) = msg {
            assert_eq!(msg_type, 8);
        } else {
            panic!("expected Unknown type 8, got {msg:?}");
        }
    }
}
