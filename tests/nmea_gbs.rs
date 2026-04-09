#![cfg(feature = "gbs")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Gbs;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGBS,194907.00,3.0,1.9,4.2,,,,*4E").expect("valid");
    let gbs = Gbs::parse(&frame.fields).expect("parse");
    let sentence = gbs.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gbs2 = Gbs::parse(&frame2.fields).expect("parse");
    assert_eq!(gbs, gbs2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGBS,194907.00,3.0,1.9,4.2,,,,*4E").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gbs(_)));
}

#[test]
fn roundtrip() {
    let original = Gbs {
        time: Some("194907.00".to_string()),
        err_lat: Some(3.0),
        err_lon: Some(1.9),
        err_alt: Some(4.2),
        svid: Some(12),
        prob: Some(0.5),
        bias: Some(1.1),
        stddev: Some(0.8),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gbs::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
