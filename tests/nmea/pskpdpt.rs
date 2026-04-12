#![cfg(feature = "pskpdpt")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Pskpdpt;
use nmea_kit::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$PSKPDPT,0002.5,+00.0,0010,10,03,*77").expect("valid");
    let pskpdpt = Pskpdpt::parse(&frame.fields).expect("parse");
    let sentence = pskpdpt.to_proprietary_sentence();
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pskpdpt2 = Pskpdpt::parse(&frame2.fields).expect("parse");
    assert_eq!(pskpdpt, pskpdpt2);
}

#[test]
fn roundtrip() {
    let original = Pskpdpt {
        depth: Some(2.5),
        offset: Some(0.0),
        range_scale: Some(10),
        echo_strength: Some(10),
        channel: Some(3),
        transducer_location: None,
    };
    let sentence = original.to_proprietary_sentence();
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pskpdpt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
