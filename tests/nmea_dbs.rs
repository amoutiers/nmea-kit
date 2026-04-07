#![cfg(feature = "dbs")]

use nmea_kit::nmea::sentences::Dbs;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn dispatch() {
    let frame = parse_frame("$IIDBS,035.53,f,010.83,M,005.85,F*24").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dbs(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIDBS,035.53,f,010.83,M,005.85,F*24").expect("valid");
    let dbs = Dbs::parse(&frame.fields).expect("parse");
    let sentence = dbs.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dbs2 = Dbs::parse(&frame2.fields).expect("parse");
    assert_eq!(dbs, dbs2);
}

#[test]
fn roundtrip() {
    let original = Dbs {
        depth_feet: Some(35.53),
        depth_meters: Some(10.83),
        depth_fathoms: Some(5.85),
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dbs::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
