#![cfg(feature = "vtg")]

use nmea_kit::nmea::sentences::Vtg;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid");
    let vtg = Vtg::parse(&frame.fields).expect("parse");
    let sentence = vtg.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vtg2 = Vtg::parse(&frame2.fields).expect("parse");
    assert_eq!(vtg, vtg2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vtg(_)));
}

#[test]
fn roundtrip() {
    let original = Vtg {
        course_true: Some(0.0),
        course_mag: Some(359.3),
        speed_kts: Some(5.0),
        speed_kmh: Some(9.26),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vtg::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
