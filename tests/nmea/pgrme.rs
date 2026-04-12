#![cfg(feature = "pgrme")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Pgrme;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PGRME,3.3,M,4.9,M,6.0,M*25").expect("valid");
    let pgrme = Pgrme::parse(&frame.fields).expect("parse");
    let sentence = pgrme.to_proprietary_sentence();
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pgrme2 = Pgrme::parse(&frame2.fields).expect("parse");
    assert_eq!(pgrme, pgrme2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PGRME,3.3,M,4.9,M,6.0,M*25").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pgrme(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pgrme {
        horizontal: Some(3.3),
        vertical: Some(4.9),
        spherical: Some(6.0),
    };
    let sentence = original.to_proprietary_sentence();
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pgrme::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
