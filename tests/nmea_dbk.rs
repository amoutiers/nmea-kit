#![cfg(feature = "dbk")]

use nmea_kit::nmea::sentences::Dbk;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$SDDBK,12.3,f,3.7,M,2.0,F*2F").expect("valid");
    let dbk = Dbk::parse(&frame.fields).expect("parse");
    let sentence = dbk.to_sentence("SD");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dbk2 = Dbk::parse(&frame2.fields).expect("parse");
    assert_eq!(dbk, dbk2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIDBK,035.53,f,010.83,M,005.85,F*3C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dbk(_)));
}

#[test]
fn roundtrip() {
    let original = Dbk {
        depth_feet: Some(35.53),
        depth_meters: Some(10.83),
        depth_fathoms: Some(5.85),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dbk::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
