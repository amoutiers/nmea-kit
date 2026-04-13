#![cfg(feature = "gsa")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Gsa;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39").expect("valid");
    let gsa = Gsa::parse(&frame.fields).expect("parse");
    let sentence = gsa.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gsa2 = Gsa::parse(&frame2.fields).expect("parse");
    assert_eq!(gsa, gsa2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gsa(_)));
}

#[test]
fn roundtrip() {
    let original = Gsa {
        mode: Some('A'),
        fix_type: Some(3),
        prns: [
            Some(4),
            Some(5),
            None,
            Some(9),
            Some(12),
            None,
            None,
            Some(24),
            None,
            None,
            None,
            None,
        ],
        pdop: Some(2.5),
        hdop: Some(1.3),
        vdop: Some(2.1),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gsa::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
