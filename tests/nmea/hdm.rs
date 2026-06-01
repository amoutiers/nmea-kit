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

#[test]
fn hdm_values() {
    // (a) value half
    let frame = parse_frame("$GPHDM,223.12,M*05").expect("valid");
    let x = Hdm::parse(&frame.fields).expect("parse");
    assert!((x.heading_mag.expect("heading_mag") - 223.12).abs() < 1e-2);

    // (b) wire half
    let s = x.to_sentence("GP");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "GPHDM,223.12,M");
}
