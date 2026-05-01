#![cfg(feature = "wpl")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Wpl;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn dispatch() {
    let frame = parse_frame("$IIWPL,5503.4530,N,01037.2742,E,411*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Wpl(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIWPL,5503.4530,N,01037.2742,E,411*6F").expect("valid");
    let wpl = Wpl::parse(&frame.fields).expect("parse");
    let sentence = wpl.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let wpl2 = Wpl::parse(&frame2.fields).expect("parse");
    assert_eq!(wpl, wpl2);
}

#[test]
fn roundtrip() {
    let original = Wpl {
        lat: Some(5503.453),
        ns: Some('N'),
        lon: Some(1037.2742),
        ew: Some('E'),
        ident: Some("411".to_string()),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Wpl::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
