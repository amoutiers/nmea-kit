#![cfg(feature = "ais")]
use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type11_utc_date_response_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,;4R33:1uUK2F`q?mOt@@GoQ00000,0*5D").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
    if let AisMessage::UtcDateResponse(utc) = msg {
        assert!(utc.mmsi > 0);
        if let Some(lat) = utc.latitude {
            assert!((-90.0..=90.0).contains(&lat));
        }
        if let Some(lon) = utc.longitude {
            assert!((-180.0..=180.0).contains(&lon));
        }
        if let Some(h) = utc.hour {
            assert!(h < 24, "hour sentinel not filtered");
        }
    } else {
        panic!("expected UtcDateResponse (type 11), got {msg:?}");
    }
}
