#![cfg(feature = "ais")]
use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type13_safety_ack_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,=39UOj0jFs9R,0*65").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
    if let AisMessage::BinaryAck(ack) = msg {
        assert_eq!(ack.msg_type, 13);
        assert!(ack.mmsi > 0);
        assert!(!ack.acks.is_empty());
    } else {
        panic!("expected BinaryAck (type 13), got {msg:?}");
    }
}
