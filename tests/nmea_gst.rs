#![cfg(feature = "gst")]

use nmea_kit::nmea::sentences::Gst;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGST,131519.00,11,,,,0.70,0.49,1.1*53").expect("valid");
    let gst = Gst::parse(&frame.fields).expect("parse");
    let sentence = gst.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gst2 = Gst::parse(&frame2.fields).expect("parse");
    assert_eq!(gst, gst2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGST,131519.00,11,,,,0.70,0.49,1.1*53").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gst(_)));
}

#[test]
fn roundtrip() {
    let original = Gst {
        time: Some("131519.00".to_string()),
        range_rms: Some(11.0),
        std_major: Some(5.2),
        std_minor: Some(3.1),
        orient: Some(45.0),
        std_lat: Some(0.7),
        std_lon: Some(0.49),
        std_alt: Some(1.1),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gst::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
