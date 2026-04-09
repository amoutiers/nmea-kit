//! AIS parser-level tests: frame filtering, fragment reassembly, reset.
#![cfg(feature = "ais")]

use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn ignores_nmea_dollar_frames() {
    let mut parser = AisParser::new();
    let frame = parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77")
        .expect("valid NMEA sentence");
    assert!(
        parser.decode(&frame).is_none(),
        "parser should ignore $ NMEA frames"
    );
}

#[test]
fn unknown_message_type_signalk() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,85Mv070j2d>=<e<<=PQhhg`59P00,0*26").expect("valid Type 8 frame");
    match parser.decode(&frame) {
        Some(AisMessage::Unknown { msg_type }) => assert_eq!(msg_type, 8),
        other => panic!("expected Unknown {{ msg_type: 8 }}, got {other:?}"),
    }
}
