#![cfg(feature = "hdm")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Hdm;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPHDM,223.12,M*05").expect("valid");
    let hdm = Hdm::parse(&frame.fields).expect("parse");
    let sentence = hdm.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let hdm2 = Hdm::parse(&frame2.fields).expect("parse");
    assert_eq!(hdm, hdm2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPHDM,223.12,M*05").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hdm(_)));
}

#[test]
fn roundtrip() {
    let original = Hdm {
        heading_mag: Some(186.5),
    };
    let sentence = original.to_sentence("04");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Hdm::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
