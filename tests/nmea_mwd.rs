#![cfg(feature = "mwd")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Mwd;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIMWD,046.,T,046.,M,10.1,N,05.2,M*43").expect("valid");
    let mwd = Mwd::parse(&frame.fields).expect("parse");
    let sentence = mwd.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let mwd2 = Mwd::parse(&frame2.fields).expect("parse");
    assert_eq!(mwd, mwd2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIMWD,,,046.,M,10.1,N,05.2,M*0B").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mwd(_)));
}

#[test]
fn roundtrip() {
    let original = Mwd {
        wind_dir_true: Some(270.0),
        wind_dir_mag: Some(268.5),
        wind_speed_kts: Some(12.4),
        wind_speed_ms: Some(6.4),
    };
    let sentence = original.to_sentence("WI");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Mwd::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
