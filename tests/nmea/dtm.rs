#![cfg(feature = "dtm")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Dtm;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F").expect("valid");
    let dtm = Dtm::parse(&frame.fields).expect("parse");
    let sentence = dtm.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dtm2 = Dtm::parse(&frame2.fields).expect("parse");
    assert_eq!(dtm, dtm2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dtm(_)));
}

#[test]
fn roundtrip() {
    let original = Dtm {
        datum: Some("W84".to_string()),
        sub_datum: None,
        lat_offset: Some(0.0),
        ns: Some('N'),
        lon_offset: Some(0.0),
        ew: Some('E'),
        alt_offset: Some(0.0),
        ref_datum: Some("W84".to_string()),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dtm::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
