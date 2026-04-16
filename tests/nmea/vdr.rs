#![cfg(feature = "vdr")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Vdr;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIVDR,10.1,T,12.3,M,1.2,N*3A").expect("valid");
    let vdr = Vdr::parse(&frame.fields).expect("parse");
    let sentence = vdr.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vdr2 = Vdr::parse(&frame2.fields).expect("parse");
    assert_eq!(vdr, vdr2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIVDR,10.1,T,12.3,M,1.2,N*3A").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vdr(_)));
}

#[test]
fn roundtrip() {
    let original = Vdr {
        direction_true: Some(10.1),
        direction_mag: Some(12.3),
        speed_knots: Some(1.2),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vdr::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
