//! AIS Type 27 — Long Range Position Report.
#![cfg(feature = "ais")]

use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type_27_long_range_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,KCQ9r=hrFUnH7P00,0*41").expect("valid Type 27 frame");
    let msg = parser.decode(&frame).expect("Type 27 should decode");

    match msg {
        AisMessage::LongRangePosition(pos) => {
            assert!(pos.mmsi > 0, "MMSI should be non-zero");
            if let (Some(lat), Some(lon)) = (pos.latitude, pos.longitude) {
                assert!((-90.0..=90.0).contains(&lat), "latitude {lat} out of range");
                assert!(
                    (-180.0..=180.0).contains(&lon),
                    "longitude {lon} out of range"
                );
            }
        }
        other => panic!("expected LongRangePosition, got {other:?}"),
    }
}
