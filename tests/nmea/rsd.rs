#![cfg(feature = "rsd")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Rsd;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$RARSD,0.00,,2.50,005.0,0.00,,4.50,355.0,,,3.0,N,H*51").expect("valid");
    let rsd = Rsd::parse(&frame.fields).expect("parse");
    let sentence = rsd.to_sentence("RA");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rsd2 = Rsd::parse(&frame2.fields).expect("parse");
    assert_eq!(rsd, rsd2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$RARSD,0.00,,2.50,005.0,0.00,,4.50,355.0,,,3.0,N,H*51").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rsd(_)));
}

#[test]
fn roundtrip() {
    let original = Rsd {
        origin1_range: Some(0.0),
        origin1_bearing: None,
        vrm1: Some(2.5),
        bearing_line1: Some(5.0),
        origin2_range: Some(0.0),
        origin2_bearing: None,
        vrm2: Some(4.5),
        bearing_line2: Some(355.0),
        cursor_range: None,
        cursor_bearing: None,
        range_scale: Some(3.0),
        range_unit: Some('N'),
        display_rotation: Some('H'),
    };
    let sentence = original.to_sentence("RA");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rsd::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
