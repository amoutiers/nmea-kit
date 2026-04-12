#![cfg(feature = "pashr")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Pashr;
use nmea_kit::parse_frame;

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$PASHR,085335.000,224.19,T,-01.26,+00.83,+00.10,0.101,0.113,0.267,1,0*07")
            .expect("valid");
    let pashr = Pashr::parse(&frame.fields).expect("parse");
    let sentence = pashr.to_proprietary_sentence();
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pashr2 = Pashr::parse(&frame2.fields).expect("parse");
    assert_eq!(pashr, pashr2);
}

#[test]
fn roundtrip() {
    let original = Pashr {
        time: Some("085335.000".to_string()),
        heading: Some(224.19),
        roll: Some(-1.26),
        pitch: Some(0.83),
        heave: Some(0.10),
        roll_accuracy: Some(0.101),
        pitch_accuracy: Some(0.113),
        heading_accuracy: Some(0.267),
        gnss_quality: Some(1),
        imu_alignment: Some(0),
    };
    let sentence = original.to_proprietary_sentence();
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pashr::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
