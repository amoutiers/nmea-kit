#![cfg(feature = "hbt")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Hbt;
use nmea_kit::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$HCHBT,1.5,A,1*23").expect("valid");
    let hbt = Hbt::parse(&frame.fields).expect("parse");
    let sentence = hbt.to_sentence("HC");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let hbt2 = Hbt::parse(&frame2.fields).expect("parse");
    assert_eq!(hbt, hbt2);
}

#[test]
fn roundtrip() {
    let original = Hbt {
        interval: Some(1.5),
        operation_status: Some('A'),
        msg_id: Some(1),
    };
    let sentence = original.to_sentence("HC");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Hbt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
