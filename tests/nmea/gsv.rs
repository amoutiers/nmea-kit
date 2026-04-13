#![cfg(feature = "gsv")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::{Gsv, SatInfo};
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGSV,3,1,09,09,73,246,35,02,51,060,40,06,16,058,37,07,16,291,25*78")
        .expect("valid");
    let gsv = Gsv::parse(&frame.fields).expect("parse");
    let sentence = gsv.to_sentence("GP");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gsv2 = Gsv::parse(&frame2.fields).expect("parse");
    assert_eq!(gsv, gsv2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGSV,3,1,09,09,73,246,35,02,51,060,40,06,16,058,37,07,16,291,25*78")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gsv(_)));
}

#[test]
fn roundtrip() {
    let original = Gsv {
        total_msgs: Some(3),
        msg_num: Some(1),
        sats_in_view: Some(9),
        sats: vec![
            SatInfo {
                prn: Some(9),
                elevation: Some(73),
                azimuth: Some(246),
                snr: Some(35),
            },
            SatInfo {
                prn: Some(2),
                elevation: Some(51),
                azimuth: Some(60),
                snr: Some(40),
            },
            SatInfo {
                prn: Some(6),
                elevation: Some(16),
                azimuth: Some(58),
                snr: Some(37),
            },
            SatInfo {
                prn: Some(7),
                elevation: Some(16),
                azimuth: Some(35),
                snr: Some(25),
            },
        ],
        signal_id: None,
    };
    let sentence = original.to_sentence("GP");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gsv::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
