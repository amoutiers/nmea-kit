#![cfg(feature = "xte")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Xte;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPXTE,A,A,0.67,L,N*6F").expect("valid");
    let xte = Xte::parse(&frame.fields).expect("parse");
    let sentence = xte.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let xte2 = Xte::parse(&frame2.fields).expect("parse");
    assert_eq!(xte, xte2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPXTE,A,A,0.67,L,N*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Xte(_)));
}

#[test]
fn roundtrip() {
    let original = Xte {
        gwarn: Some('A'),
        lccwarn: Some('A'),
        ctrkerr: Some(0.67),
        dirs: Some('L'),
        disunit: Some('N'),
        mode: Some('D'),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Xte::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
