#![cfg(feature = "bod")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Bod;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBOD,097.0,T,103.2,M,POINTB,POINTA*4A").expect("valid");
    let bod = Bod::parse(&frame.fields).expect("parse");
    let sentence = bod.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bod2 = Bod::parse(&frame2.fields).expect("parse");
    assert_eq!(bod, bod2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPBOD,097.0,T,103.2,M,POINTB,POINTA*4A").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Bod(_)));
}

#[test]
fn roundtrip() {
    let original = Bod {
        bear_true: Some(97.0),
        bear_true_type: Some('T'),
        bear_mag: Some(103.2),
        bear_mag_type: Some('M'),
        wpt_dest: Some("POINTB".to_string()),
        wpt_origin: Some("POINTA".to_string()),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bod::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
