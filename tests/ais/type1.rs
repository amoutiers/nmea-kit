//! AIS Types 1/2/3 — Class A Position Report.
#![cfg(feature = "ais")]

use nmea_kit::ais::{AisClass, AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type_1_single_fragment_class_a_signalk() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid Type 1 frame");
    let msg = parser.decode(&frame).expect("Type 1 should decode");

    match msg {
        AisMessage::Position(pos) => {
            assert_eq!(pos.msg_type, 1);
            assert!(pos.mmsi > 0);
            assert_eq!(pos.ais_class, AisClass::A);
            let lat = pos.latitude.expect("latitude present");
            let lon = pos.longitude.expect("longitude present");
            assert!((-90.0..=90.0).contains(&lat), "latitude {lat} out of range");
            assert!(
                (-180.0..=180.0).contains(&lon),
                "longitude {lon} out of range"
            );
        }
        other => panic!("expected Position, got {other:?}"),
    }
}

#[test]
fn sentinel_filtering_signalk() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid Type 1 frame");
    let msg = parser.decode(&frame).expect("Type 1 should decode");

    if let AisMessage::Position(pos) = msg {
        if let Some(lat) = pos.latitude {
            assert!((-90.0..=90.0).contains(&lat), "lat {lat} out of range");
        }
        if let Some(lon) = pos.longitude {
            assert!((-180.0..=180.0).contains(&lon), "lon {lon} out of range");
        }
        if let Some(hdg) = pos.heading {
            assert!(hdg < 360, "heading {hdg} should be < 360");
        }
    }
}
