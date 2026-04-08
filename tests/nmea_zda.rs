#![cfg(feature = "zda")]

use nmea_kit::nmea::sentences::Zda;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPZDA,172809.456,12,07,1996,00,00*57").expect("valid");
    let zda = Zda::parse(&frame.fields).expect("parse");
    let sentence = zda.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let zda2 = Zda::parse(&frame2.fields).expect("parse");
    assert_eq!(zda.time, zda2.time);
    assert_eq!(zda.day, zda2.day);
    assert_eq!(zda.month, zda2.month);
    assert_eq!(zda.year, zda2.year);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GNZDA,103607.00,06,03,2021,00,00*7F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Zda(_)));
}

#[test]
fn roundtrip() {
    let original = Zda {
        time: Some("160012.71".to_string()),
        day: Some(11),
        month: Some(3),
        year: Some(2004),
        local_hour_offset: Some(-1.0),
        local_min_offset: Some(0),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Zda::parse(&frame.fields).expect("parse");
    assert_eq!(original.time, parsed.time);
    assert_eq!(original.day, parsed.day);
    assert_eq!(original.month, parsed.month);
    assert_eq!(original.year, parsed.year);
}
