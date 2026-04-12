#![cfg(feature = "rte")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Rte;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIRTE,4,1,c,Rte 1,411,412,413,414,415*6F").expect("valid");
    let rte = Rte::parse(&frame.fields).expect("parse");
    let sentence = rte.to_sentence("II");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rte2 = Rte::parse(&frame2.fields).expect("parse");
    assert_eq!(rte, rte2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIRTE,4,1,c,Rte 1,411,412,413,414,415*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rte(_)));
}

#[test]
fn roundtrip() {
    let original = Rte {
        num_sentences: Some(4),
        sentence_num: Some(1),
        mode: Some('c'),
        name: Some("Rte 1".to_string()),
        idents: vec![
            "411".to_string(),
            "412".to_string(),
            "413".to_string(),
            "414".to_string(),
            "415".to_string(),
        ],
    };
    let sentence = original.to_sentence("II");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rte::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
