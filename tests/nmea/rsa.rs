#![cfg(feature = "rsa")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Rsa;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIRSA,10.5,A,,V*4D").expect("valid");
    let rsa = Rsa::parse(&frame.fields).expect("parse");
    let sentence = rsa.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rsa2 = Rsa::parse(&frame2.fields).expect("parse");
    assert_eq!(rsa, rsa2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIRSA,10.5,A,,V*4D").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rsa(_)));
}

#[test]
fn roundtrip() {
    let original = Rsa {
        starboard_angle: Some(10.5),
        starboard_status: Some('A'),
        port_angle: Some(-5.2),
        port_status: Some('A'),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rsa::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
