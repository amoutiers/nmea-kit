#![cfg(feature = "mtw")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Mtw;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$YXMTW,15.2,C*14").expect("valid");
    let mtw = Mtw::parse(&frame.fields).expect("parse");
    let sentence = mtw.to_sentence("YX");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let mtw2 = Mtw::parse(&frame2.fields).expect("parse");
    assert_eq!(mtw, mtw2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$YXMTW,15.2,C*14").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mtw(_)));
}

#[test]
fn roundtrip() {
    let original = Mtw {
        temperature: Some(15.2),
        units: Some('C'),
    };
    let sentence = original.to_sentence("YX");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Mtw::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
