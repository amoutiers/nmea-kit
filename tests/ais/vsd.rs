#![cfg(feature = "vsd")]
use nmea_kit::ais::sentences::{AisSentence, Vsd};
use nmea_kit::parse_frame;

#[test]
fn vsd_values() {
    let frame =
        parse_frame("$RAVSD,0,4.5,6,@@@@@@@@@@@@@@@@@@@@,220516,01,02,8,*6E").expect("valid");
    let vsd = Vsd::parse(&frame.fields).expect("parse");

    assert_eq!(vsd.type_of_ship, Some(0));
    assert!((vsd.draught.expect("draught") - 4.5).abs() < 0.0001);
    assert_eq!(vsd.persons, Some(6));
    assert_eq!(vsd.destination.as_deref(), Some("@@@@@@@@@@@@@@@@@@@@"));
    assert_eq!(vsd.arrival_time.as_deref(), Some("220516"));
    assert_eq!(vsd.arrival_day, Some(1));
    assert_eq!(vsd.arrival_month, Some(2));
    assert_eq!(vsd.nav_status, Some(8));
    assert!(vsd.regional.is_none());
}

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$RAVSD,0,4.5,6,@@@@@@@@@@@@@@@@@@@@,220516,01,02,8,*6E").expect("valid");
    let vsd = Vsd::parse(&frame.fields).expect("parse");
    let sentence2 = vsd.to_sentence("RA").expect("encode");
    assert!(sentence2.starts_with("$RAVSD,"));
    let frame2 = parse_frame(sentence2.trim()).expect("re-parse");
    let vsd2 = Vsd::parse(&frame2.fields).expect("parse");
    assert_eq!(vsd, vsd2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$RAVSD,0,4.5,6,@@@@@@@@@@@@@@@@@@@@,220516,01,02,8,*6E").expect("valid");
    assert!(matches!(AisSentence::parse(&frame), AisSentence::Vsd(_)));
}

#[test]
fn roundtrip() {
    let original = Vsd {
        type_of_ship: Some(0),
        draught: Some(4.5),
        persons: Some(6),
        destination: Some("PORT".to_string()),
        arrival_time: Some("220516".to_string()),
        arrival_day: Some(1),
        arrival_month: Some(2),
        nav_status: Some(8),
        regional: None,
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vsd::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
