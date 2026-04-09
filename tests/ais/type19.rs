//! AIS Type 19 — Class B Extended Position Report.
#![cfg(feature = "ais")]

use nmea_kit::ais::{AisClass, AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type_19_class_b_extended_gpsd() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,B,C5N3SRgPEnJGEBT>NhWAwwo862PaLELTBJ:V00000000S0D:R220,0*0B")
            .expect("valid Type 19 frame");
    let msg = parser.decode(&frame).expect("Type 19 should decode");

    match msg {
        AisMessage::Position(pos) => {
            assert_eq!(pos.msg_type, 19);
            assert_eq!(pos.ais_class, AisClass::BPlus, "Type 19 should be Class B+");
            assert!(pos.mmsi > 0);
        }
        other => panic!("expected Position, got {other:?}"),
    }
}
