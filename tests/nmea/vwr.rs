#![cfg(feature = "vwr")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Vwr;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIVWR,75,R,1.0,N,0.51,M,1.85,K*6C").expect("valid");
    let vwr = Vwr::parse(&frame.fields).expect("parse");
    let sentence = vwr.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vwr2 = Vwr::parse(&frame2.fields).expect("parse");
    assert_eq!(vwr, vwr2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIVWR,75,R,1.0,N,0.51,M,1.85,K*6C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vwr(_)));
}

#[test]
fn roundtrip() {
    let original = Vwr {
        angle: Some(75.0),
        angle_lr: Some('R'),
        speed_knots: Some(1.0),
        speed_ms: Some(0.51),
        speed_kmh: Some(1.85),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vwr::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
