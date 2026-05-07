#![cfg(feature = "ttm")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Ttm;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$RATTM,02,1.43,170.5,T,0.16,264.4,T,1.42,36.9,N,,T,,,M*2A").expect("valid");
    let ttm = Ttm::parse(&frame.fields).expect("parse");
    let sentence = ttm.to_sentence("RA");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let ttm2 = Ttm::parse(&frame2.fields).expect("parse");
    assert_eq!(ttm, ttm2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$RATTM,02,1.43,170.5,T,0.16,264.4,T,1.42,36.9,N,,T,,,M*2A").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Ttm(_)));
}

#[test]
fn roundtrip() {
    let original = Ttm {
        target_num: Some(2),
        dist: Some(1.43),
        bearing: Some(170.5),
        bearing_type: Some('T'),
        speed: Some(0.16),
        course: Some(264.4),
        course_type: Some('T'),
        dist_cpa: Some(1.42),
        time_cpa: Some(36.9),
        speed_units: Some('N'),
        name: None,
        status: Some('T'),
        ref_target: None,
        time: None,
        acq_type: Some('M'),
    };
    let sentence = original.to_sentence("RA");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Ttm::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
