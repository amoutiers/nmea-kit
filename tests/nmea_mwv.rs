#![cfg(feature = "mwv")]

use nmea_kit::nmea::sentences::Mwv;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn dispatch() {
    let frame = parse_frame("$IIMWV,336,R,13.41,N,A*22").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mwv(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIMWV,336,R,13.41,N,A*22").expect("valid");
    let mwv = Mwv::parse(&frame.fields).expect("parse");
    let sentence = mwv.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let mwv2 = Mwv::parse(&frame2.fields).expect("parse");
    assert_eq!(mwv, mwv2);
}

#[test]
fn roundtrip() {
    let original = Mwv {
        wind_angle: Some(336.0),
        reference: Some('R'),
        wind_speed: Some(13.41),
        speed_units: Some('N'),
        status: Some('A'),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Mwv::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
