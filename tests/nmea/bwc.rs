#![cfg(feature = "bwc")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Bwc;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBWC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*21")
        .expect("valid");
    let bwc = Bwc::parse(&frame.fields).expect("parse");
    let sentence = bwc.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bwc2 = Bwc::parse(&frame2.fields).expect("parse");
    assert_eq!(bwc, bwc2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPBWC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*21")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Bwc(_)));
}

#[test]
fn roundtrip() {
    let original = Bwc {
        time: Some("225444".to_string()),
        lat: Some(4917.24),
        ns: Some('N'),
        lon: Some(12309.57),
        ew: Some('W'),
        bear_true: Some(51.9),
        bear_mag: Some(31.6),
        dist: Some(1.3),
        wpt: Some("004".to_string()),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bwc::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
