//! AIS Type 4 — Base Station Report.
#![cfg(feature = "ais")]

use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type_4_base_station_gpsd() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,403OviQuMGCqWrRO9>E6fE700@GO,0*4D").expect("valid Type 4 frame");
    let msg = parser.decode(&frame).expect("Type 4 should decode");

    match msg {
        AisMessage::BaseStation(report) => {
            assert!(report.mmsi > 0, "MMSI should be non-zero");
            if let (Some(lat), Some(lon)) = (report.latitude, report.longitude) {
                assert!((-90.0..=90.0).contains(&lat), "latitude {lat} out of range");
                assert!(
                    (-180.0..=180.0).contains(&lon),
                    "longitude {lon} out of range"
                );
            }
            if let Some(h) = report.hour {
                assert!(h < 24, "hour sentinel not filtered: {h}");
            }
            if let Some(m) = report.minute {
                assert!(m < 60, "minute sentinel not filtered: {m}");
            }
        }
        other => panic!("expected BaseStation, got {other:?}"),
    }
}
