#![cfg(feature = "gll")]

use nmea_kit::nmea::sentences::Gll;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGLL,5958.613,N,02325.928,E,121022,A,D*40").expect("valid");
    let gll = Gll::parse(&frame.fields).expect("parse");
    let sentence = gll.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gll2 = Gll::parse(&frame2.fields).expect("parse");
    assert_eq!(gll, gll2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGLL,5958.613,N,02325.928,E,121022,A,D*40").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gll(_)));
}

#[test]
fn roundtrip() {
    let original = Gll {
        lat: Some(4807.038),
        ns: Some('N'),
        lon: Some(1131.0),
        ew: Some('E'),
        time: Some("120000".to_string()),
        status: Some('A'),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gll::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
