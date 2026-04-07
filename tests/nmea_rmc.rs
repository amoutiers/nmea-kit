#![cfg(feature = "rmc")]

use nmea_kit::nmea::sentences::Rmc;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn dispatch() {
    let frame =
        parse_frame("$GPRMC,085412.000,A,5222.3198,N,00454.5784,E,0.58,251.34,030414,,,A*65")
            .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rmc(_)));
}

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$GPRMC,085412.000,A,5222.3198,N,00454.5784,E,0.58,251.34,030414,,,A*65")
            .expect("valid");
    let rmc = Rmc::parse(&frame.fields).expect("parse");
    let sentence = rmc.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rmc2 = Rmc::parse(&frame2.fields).expect("parse");
    assert_eq!(rmc, rmc2);
}

#[test]
fn roundtrip() {
    let original = Rmc {
        time: Some("120000.00".to_string()),
        status: Some('A'),
        lat: Some(4807.038),
        ns: Some('N'),
        lon: Some(1131.0),
        ew: Some('E'),
        sog: Some(5.5),
        cog: Some(54.7),
        date: Some("230394".to_string()),
        mag_var: Some(3.1),
        mag_var_ew: Some('E'),
        pos_mode: Some('A'),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rmc::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
