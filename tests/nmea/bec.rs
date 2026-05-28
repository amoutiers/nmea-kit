#![cfg(feature = "bec")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Bec;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBEC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*33")
        .expect("valid");
    let bec = Bec::parse(&frame.fields).expect("parse");
    let sentence = bec.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bec2 = Bec::parse(&frame2.fields).expect("parse");
    assert_eq!(bec, bec2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPBEC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*33")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Bec(_)));
}

#[test]
fn roundtrip() {
    let original = Bec {
        time: Some("220516".to_string()),
        lat: Some(5130.02),
        ns: Some('N'),
        lon: Some(46.34),
        ew: Some('W'),
        bear_true: Some(213.8),
        bear_mag: Some(218.0),
        dist: Some(4.6),
        wpt: Some("EGLM".to_string()),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bec::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
