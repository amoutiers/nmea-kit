#![cfg(feature = "hdt")]

use nmea_kit::nmea::sentences::Hdt;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn dispatch() {
    let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hdt(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid");
    let hdt = Hdt::parse(&frame.fields).expect("parse");
    let sentence = hdt.to_sentence("HE");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let hdt2 = Hdt::parse(&frame2.fields).expect("parse");
    assert_eq!(hdt, hdt2);
}

#[test]
fn roundtrip() {
    let original = Hdt {
        heading_true: Some(123.456),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Hdt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
