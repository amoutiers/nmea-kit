#![cfg(feature = "gns")]

use nmea_kit::nmea::sentences::Gns;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn dispatch() {
    let frame =
        parse_frame("$GPGNS,111648.00,0235.0379,S,04422.1450,W,ANN,12,0.8,8.5,-22.3,,,S*5D")
            .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gns(_)));
}

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$GPGNS,111648.00,0235.0379,S,04422.1450,W,ANN,12,0.8,8.5,-22.3,,,S*5D")
            .expect("valid");
    let gns = Gns::parse(&frame.fields).expect("parse");
    let sentence = gns.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gns2 = Gns::parse(&frame2.fields).expect("parse");
    assert_eq!(gns, gns2);
}

#[test]
fn roundtrip() {
    let original = Gns {
        time: Some("120000.00".to_string()),
        lat: Some(4807.038),
        ns: Some('N'),
        lon: Some(1131.0),
        ew: Some('E'),
        mode: Some("AAN".to_string()),
        num_sats: Some(10),
        hdop: Some(0.9),
        altitude: Some(100.5),
        geoid_sep: Some(-23.0),
        dgps_age: None,
        dgps_station: None,
        nav_status: Some('S'),
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gns::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
