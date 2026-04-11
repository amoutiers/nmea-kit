#![cfg(feature = "ais")]
use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type9_sar_aircraft_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,91b77=h3h00nHt0Q3r@@07000<0b,0*69").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
    if let AisMessage::SarAircraft(sar) = msg {
        assert!(sar.mmsi > 0);
        if let Some(lat) = sar.latitude {
            assert!((-90.0..=90.0).contains(&lat));
        }
        if let Some(lon) = sar.longitude {
            assert!((-180.0..=180.0).contains(&lon));
        }
    } else {
        panic!("expected SarAircraft (type 9), got {msg:?}");
    }
}

#[test]
fn type9_second_fixture_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,91b55wi;hbOS@OdQAC062Ch2089h,0*30").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
    if let AisMessage::SarAircraft(sar) = msg {
        assert!(sar.mmsi > 0);
    } else {
        panic!("expected SarAircraft, got {msg:?}");
    }
}
