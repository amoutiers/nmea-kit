#![cfg(feature = "rmb")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Rmb;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$ECRMB,A,0.000,L,001,002,4653.550,N,07115.984,W,2.505,334.205,0.000,V*04")
            .expect("valid");
    let rmb = Rmb::parse(&frame.fields).expect("parse");
    let sentence = rmb.to_sentence("EC");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rmb2 = Rmb::parse(&frame2.fields).expect("parse");
    assert_eq!(rmb, rmb2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$ECRMB,A,0.000,L,001,002,4653.550,N,07115.984,W,2.505,334.205,0.000,V*04")
            .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rmb(_)));
}

#[test]
fn roundtrip() {
    let original = Rmb {
        status: Some('A'),
        ctrkerr: Some(0.5),
        dirs: Some('L'),
        wpt_origin: Some("001".to_string()),
        wpt_dest: Some("002".to_string()),
        dest_lat: Some(4653.55),
        ns: Some('N'),
        dest_lon: Some(7115.984),
        ew: Some('W'),
        range: Some(2.505),
        bearing: Some(334.205),
        velclos: Some(0.0),
        arrstatus: Some('V'),
        valstatus: Some('A'),
    };
    let sentence = original.to_sentence("EC");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rmb::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
