#![cfg(feature = "ais")]
use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type15_interrogation_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,?5OP=l00052HD00,2*5B").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
    if let AisMessage::Interrogation(intr) = msg {
        assert!(intr.mmsi > 0);
        assert!(intr.mmsi_1 > 0);
    } else {
        panic!("expected Interrogation (type 15), got {msg:?}");
    }
}

#[test]
fn type15_with_second_station_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,?39a?2PjKFFPD01o:Gq1igvp2<3w,0*0B").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
    if let AisMessage::Interrogation(intr) = msg {
        assert!(intr.mmsi > 0);
    } else {
        panic!("expected Interrogation, got {msg:?}");
    }
}
