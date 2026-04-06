#![cfg(feature = "nmea")]

use nmea_kit::parse_frame;

#[cfg(feature = "dpt")]
use nmea_kit::nmea::sentences::Dpt;
#[cfg(feature = "mwd")]
use nmea_kit::nmea::sentences::Mwd;

#[cfg(feature = "dpt")]
#[test]
fn dpt_roundtrip() {
    let original = Dpt {
        depth: Some(12.7),
        offset: Some(-1.5),
        rangescale: None,
    };
    let sentence = original.to_sentence("II");
    assert!(sentence.starts_with("$IIDPT,"));

    let frame = parse_frame(sentence.trim()).expect("re-parse DPT sentence");
    let parsed = Dpt::parse(&frame.fields).expect("parse DPT from re-encoded frame");

    assert_eq!(original.depth, parsed.depth);
    assert_eq!(original.offset, parsed.offset);
}

#[cfg(feature = "mwd")]
#[test]
fn mwd_encode_all_none_fields() {
    let empty = Mwd {
        wind_dir_true: None,
        wind_dir_mag: None,
        wind_speed_kts: None,
        wind_speed_ms: None,
    };
    let sentence = empty.to_sentence("WI");
    let frame = parse_frame(sentence.trim()).expect("re-parse all-None MWD");
    assert_eq!(frame.sentence_type, "MWD");
    assert_eq!(frame.fields.len(), 8);
    assert_eq!(frame.fields[1], "T");
    assert_eq!(frame.fields[3], "M");
    assert_eq!(frame.fields[5], "N");
    assert_eq!(frame.fields[7], "M");
}

#[cfg(feature = "mwd")]
#[test]
fn mwd_roundtrip() {
    let original = Mwd {
        wind_dir_true: Some(270.0),
        wind_dir_mag: Some(268.5),
        wind_speed_kts: Some(12.4),
        wind_speed_ms: Some(6.4),
    };
    let sentence = original.to_sentence("WI");
    assert!(sentence.starts_with("$WIMWD,"));

    let frame = parse_frame(sentence.trim()).expect("re-parse MWD sentence");
    let parsed = Mwd::parse(&frame.fields).expect("parse MWD from re-encoded frame");

    assert_eq!(original.wind_dir_true, parsed.wind_dir_true);
    assert_eq!(original.wind_dir_mag, parsed.wind_dir_mag);
    assert_eq!(original.wind_speed_kts, parsed.wind_speed_kts);
    assert_eq!(original.wind_speed_ms, parsed.wind_speed_ms);
}
