#![cfg(feature = "xdr")]

use nmea_kit::nmea::sentences::{Xdr, XdrGroup};
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$SDXDR,C,23.15,C,WTHI*70").expect("valid");
    let xdr = Xdr::parse(&frame.fields).expect("parse");
    let sentence = xdr.to_sentence("SD");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let xdr2 = Xdr::parse(&frame2.fields).expect("parse");
    assert_eq!(xdr, xdr2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$HCXDR,A,171,D,PITCH,A,-37,D,ROLL,G,367,,MAGX,G,2420,,MAGY,G,-8984,,MAGZ*41")
            .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Xdr(_)));
}

#[test]
fn roundtrip() {
    let original = Xdr {
        groups: vec![
            XdrGroup {
                sensor_type: Some('P'),
                value: Some(1013.25),
                unit: Some('B'),
                name: Some("Baro".to_string()),
            },
            XdrGroup {
                sensor_type: Some('C'),
                value: Some(22.5),
                unit: Some('C'),
                name: Some("Temp".to_string()),
            },
        ],
    };
    let sentence = original.to_sentence("WI");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Xdr::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
