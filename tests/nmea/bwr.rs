#![cfg(feature = "bwr")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Bwr;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBWR,225444,4917.24,N,12309.57,W,051.9,T,031.6,M,001.3,N,004*38")
        .expect("valid");
    let bwr = Bwr::parse(&frame.fields).expect("parse");
    let sentence = bwr.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bwr2 = Bwr::parse(&frame2.fields).expect("parse");
    assert_eq!(bwr, bwr2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPBWR,225444,4917.24,N,12309.57,W,051.9,T,031.6,M,001.3,N,004*38")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Bwr(_)));
}

#[test]
fn roundtrip() {
    let original = Bwr {
        time: Some("225444".to_string()),
        lat: Some(4917.24),
        ns: Some('N'),
        lon: Some(12309.57),
        ew: Some('W'),
        bear_true: Some(51.9),
        bear_mag: Some(31.6),
        dist: Some(1.3),
        wpt: Some("004".to_string()),
        mode: None,
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bwr::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
