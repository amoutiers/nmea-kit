#![cfg(feature = "dpt")]

use nmea_kit::nmea::sentences::Dpt;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIDPT,4.1,0.0*45").expect("valid");
    let dpt = Dpt::parse(&frame.fields).expect("parse");
    let sentence = dpt.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dpt2 = Dpt::parse(&frame2.fields).expect("parse");
    assert_eq!(dpt, dpt2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIDPT,4.1,0.0*45").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dpt(_)));
}

#[test]
fn roundtrip() {
    let original = Dpt {
        depth: Some(12.7),
        offset: Some(-1.5),
        rangescale: None,
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dpt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
