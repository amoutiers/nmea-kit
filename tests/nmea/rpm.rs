#![cfg(feature = "rpm")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Rpm;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIRPM,E,1,2418.2,10.5,A*5F").expect("valid");
    let rpm = Rpm::parse(&frame.fields).expect("parse");
    let sentence = rpm.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rpm2 = Rpm::parse(&frame2.fields).expect("parse");
    assert_eq!(rpm, rpm2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIRPM,E,1,2418.2,10.5,A*5F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rpm(_)));
}

#[test]
fn roundtrip() {
    let original = Rpm {
        source: Some('E'),
        engine_shaft_num: Some(1),
        rpm: Some(2418.2),
        pitch: Some(10.5),
        status: Some('A'),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rpm::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
