#![cfg(feature = "aam")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Aam;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPAAM,A,A,0.10,N,WPTNME*32").expect("valid");
    let aam = Aam::parse(&frame.fields).expect("parse");
    let sentence = aam.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let aam2 = Aam::parse(&frame2.fields).expect("parse");
    assert_eq!(aam, aam2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPAAM,A,A,0.10,N,WPTNME*32").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Aam(_)));
}

#[test]
fn roundtrip() {
    let original = Aam {
        arrce: Some('A'),
        perp: Some('A'),
        crad: Some(0.1),
        cunit: Some('N'),
        wpt: Some("DEST".to_string()),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Aam::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
