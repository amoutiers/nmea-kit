#![cfg(feature = "hdg")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Hdg;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$SDHDG,181.9,,,0.6,E*32").expect("valid");
    let hdg = Hdg::parse(&frame.fields).expect("parse");
    let sentence = hdg.to_sentence("SD");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let hdg2 = Hdg::parse(&frame2.fields).expect("parse");
    assert_eq!(hdg, hdg2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$SDHDG,181.9,,,0.6,E*32").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hdg(_)));
}

#[test]
fn roundtrip() {
    let original = Hdg {
        heading_mag: Some(181.9),
        deviation: Some(2.5),
        deviation_ew: Some('E'),
        variation: Some(0.6),
        variation_ew: Some('E'),
    };
    let sentence = original.to_sentence("SD");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Hdg::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
