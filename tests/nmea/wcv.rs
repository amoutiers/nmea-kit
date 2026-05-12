#![cfg(feature = "wcv")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Wcv;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let wcv = Wcv {
        vel: Some(5.3),
        wpt: Some("DEST".to_string()),
        mode: Some('A'),
    };
    let sentence = wcv.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("valid");
    let wcv2 = Wcv::parse(&frame.fields).expect("parse");
    let sentence2 = wcv2.to_sentence("GP");
    let frame2 = parse_frame(sentence2.trim()).expect("re-parse");
    let wcv3 = Wcv::parse(&frame2.fields).expect("parse");
    assert_eq!(wcv2, wcv3);
}

#[test]
fn dispatch() {
    let wcv = Wcv {
        vel: Some(5.3),
        wpt: Some("DEST".to_string()),
        mode: Some('A'),
    };
    let sentence = wcv.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Wcv(_)));
}

#[test]
fn roundtrip() {
    let original = Wcv {
        vel: Some(5.3),
        wpt: Some("DEST".to_string()),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Wcv::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
