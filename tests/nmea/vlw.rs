#![cfg(feature = "vlw")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Vlw;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GNVLW,,N,,N,0.000,N,0.000,N*44").expect("valid");
    let vlw = Vlw::parse(&frame.fields).expect("parse");
    let sentence = vlw.to_sentence("GN");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vlw2 = Vlw::parse(&frame2.fields).expect("parse");
    assert_eq!(vlw, vlw2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIVLW,10.1,N,3.2,N*7C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vlw(_)));
}

#[test]
fn roundtrip() {
    let original = Vlw {
        total_water_dist: Some(10.1),
        total_water_dist_unit: Some('N'),
        water_dist: Some(3.2),
        water_dist_unit: Some('N'),
        total_ground_dist: None,
        total_ground_dist_unit: None,
        ground_dist: None,
        ground_dist_unit: None,
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vlw::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
