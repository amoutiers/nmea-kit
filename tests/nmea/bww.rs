#![cfg(feature = "bww")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Bww;
use nmea_kit::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBWW,097.0,T,103.2,M,POINTB,POINTA*41").expect("valid");
    let bww = Bww::parse(&frame.fields).expect("parse");
    let sentence = bww.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bww2 = Bww::parse(&frame2.fields).expect("parse");
    assert_eq!(bww, bww2);
}

#[test]
fn roundtrip() {
    let original = Bww {
        bear_true: Some(97.0),
        bear_true_type: Some('T'),
        bear_mag: Some(103.2),
        bear_mag_type: Some('M'),
        wpt_dest: Some("POINTB".to_string()),
        wpt_origin: Some("POINTA".to_string()),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bww::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
