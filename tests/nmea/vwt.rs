#![cfg(feature = "vwt")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Vwt;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIVWT,030.,R,10.1,N,05.2,M,018.7,K*75").expect("valid");
    let vwt = Vwt::parse(&frame.fields).expect("parse");
    let sentence = vwt.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vwt2 = Vwt::parse(&frame2.fields).expect("parse");
    assert_eq!(vwt, vwt2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIVWT,030.,R,10.1,N,05.2,M,018.7,K*75").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vwt(_)));
}

#[test]
fn roundtrip() {
    let original = Vwt {
        angle: Some(30.0),
        angle_lr: Some('R'),
        speed_knots: Some(10.1),
        speed_ms: Some(5.2),
        speed_kmh: Some(18.7),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vwt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
