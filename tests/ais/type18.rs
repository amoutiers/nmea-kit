//! AIS Type 18 — Class B Standard Position Report.
#![cfg(feature = "ais")]

use nmea_kit::ais::{AisClass, AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type_18_class_b_position() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,B6CdCm0t3`tba35f@V9faHi7kP06,0*58")
        .expect("valid Type 18 frame");
    let msg = parser.decode(&frame);

    if let Some(AisMessage::Position(pos)) = &msg {
        assert_eq!(pos.ais_class, AisClass::B, "Type 18 should be Class B");
        assert!(pos.mmsi > 0, "MMSI should be non-zero");
    }
}
