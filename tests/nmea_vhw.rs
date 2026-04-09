#![cfg(feature = "vhw")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Vhw;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$SDVHW,182.5,T,181.8,M,0.0,N,0.0,K*4C").expect("valid");
    let vhw = Vhw::parse(&frame.fields).expect("parse");
    let sentence = vhw.to_sentence("SD");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vhw2 = Vhw::parse(&frame2.fields).expect("parse");
    assert_eq!(vhw, vhw2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$SDVHW,182.5,T,181.8,M,0.0,N,0.0,K*4C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vhw(_)));
}

#[test]
fn roundtrip() {
    let original = Vhw {
        heading_true: Some(182.5),
        heading_mag: Some(181.8),
        speed_kts: Some(12.5),
        speed_kmh: Some(23.1),
    };
    let sentence = original.to_sentence("SD");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vhw::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
