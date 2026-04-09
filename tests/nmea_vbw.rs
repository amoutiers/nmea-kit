#![cfg(feature = "vbw")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Vbw;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPVBW,12.3,0.07,A,11.78,0.12,A*6F").expect("valid");
    let vbw = Vbw::parse(&frame.fields).expect("parse");
    let sentence = vbw.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vbw2 = Vbw::parse(&frame2.fields).expect("parse");
    assert_eq!(vbw, vbw2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPVBW,12.3,0.07,A,11.78,0.12,A*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vbw(_)));
}

#[test]
fn roundtrip() {
    let original = Vbw {
        long_water_spd: Some(5.2),
        trans_water_spd: Some(0.1),
        water_spd_status: Some('A'),
        long_ground_spd: Some(5.3),
        trans_ground_spd: Some(0.2),
        ground_spd_status: Some('A'),
        stern_trans_water_spd: None,
        stern_water_spd_status: None,
        stern_trans_ground_spd: None,
        stern_ground_spd_status: None,
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vbw::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
