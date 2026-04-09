#![cfg(feature = "nmea")]

use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn dispatch_unknown() {
    let frame = parse_frame("$GPXYZ,1,2,3").expect("valid");
    match NmeaSentence::parse(&frame) {
        NmeaSentence::Unknown {
            sentence_type,
            fields,
        } => {
            assert_eq!(sentence_type, "XYZ");
            assert_eq!(fields, vec!["1", "2", "3"]);
        }
        other => panic!("expected Unknown, got {other:?}"),
    }
}

#[test]
fn unsupported_becomes_unknown() {
    let frame = parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C").expect("valid");
    match NmeaSentence::parse(&frame) {
        NmeaSentence::Unknown { sentence_type, .. } => assert_eq!(sentence_type, "APB"),
        other => panic!("expected Unknown for APB, got {other:?}"),
    }
}
