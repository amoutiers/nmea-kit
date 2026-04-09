#![cfg(feature = "gga")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Gga;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame(
        "$GPGGA,172814.0,3723.46587704,N,12202.26957864,W,2,6,1.2,18.893,M,-25.669,M,2.0,0031*4F",
    )
    .expect("valid");
    let gga = Gga::parse(&frame.fields).expect("parse");
    let sentence = gga.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gga2 = Gga::parse(&frame2.fields).expect("parse");
    assert_eq!(gga, gga2);
}

#[test]
fn dispatch() {
    let frame = parse_frame(
        "$GPGGA,172814.0,3723.46587704,N,12202.26957864,W,2,6,1.2,18.893,M,-25.669,M,2.0,0031*4F",
    )
    .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gga(_)));
}

#[test]
fn roundtrip() {
    let original = Gga {
        time: Some("120000.00".to_string()),
        lat: Some(4807.038),
        ns: Some('N'),
        lon: Some(1131.0),
        ew: Some('E'),
        quality: Some(1),
        num_sats: Some(8),
        hdop: Some(0.9),
        altitude: Some(545.4),
        alt_unit: Some('M'),
        geoid_sep: Some(46.9),
        geoid_unit: Some('M'),
        dgps_age: None,
        dgps_station: None,
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gga::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
