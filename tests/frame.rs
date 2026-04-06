use nmea_kit::{encode_frame, parse_frame, FrameError};

#[test]
fn ais_multi_fragment_fixture_signalk() {
    let frame = parse_frame(
        "!AIVDM,2,1,0,A,53brRt4000010SG700iE@LE8@Tp4000000000153P615t0Ht0SCkjH4jC1C,0*25",
    )
    .expect("AIS multi-fragment fixture");
    assert_eq!(frame.prefix, '!');
    assert_eq!(frame.sentence_type, "VDM");
    assert_eq!(frame.fields[1], "1"); // fragment number
}

#[test]
fn apb_fixture_signalk() {
    let frame = parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C")
        .expect("SignalK APB fixture");
    assert_eq!(frame.sentence_type, "APB");
    assert_eq!(frame.fields[9], "DEST");
}

#[test]
fn dbt_sounder_fixture_gpsd() {
    let frame = parse_frame("$SDDBT,7.7,f,2.3,M,1.3,F*05").expect("GPSD DBT sounder fixture");
    assert_eq!(frame.sentence_type, "DBT");
    assert_eq!(frame.fields[2], "2.3"); // meters
}

#[test]
fn dpt_fixtures_signalk() {
    let fixtures = [
        ("$IIDPT,4.1,0.0*45", "4.1", "0.0"),
        ("$IIDPT,4.1,1.0*44", "4.1", "1.0"),
        ("$IIDPT,4.1,-1.0*69", "4.1", "-1.0"),
    ];
    for (fix, depth, offset) in &fixtures {
        let frame = parse_frame(fix).unwrap_or_else(|e| panic!("failed to parse {fix}: {e}"));
        assert_eq!(frame.sentence_type, "DPT");
        assert_eq!(frame.fields[0], *depth);
        assert_eq!(frame.fields[1], *offset);
    }
}

#[test]
fn encode_ais_prefix() {
    let result = encode_frame('!', "AI", "VDM", &["1", "1", "", "A", "payload", "0"]);
    assert!(result.starts_with("!AIVDM,"));
    let frame = parse_frame(result.trim()).expect("re-parse AIS encoded");
    assert_eq!(frame.prefix, '!');
    assert_eq!(frame.sentence_type, "VDM");
}

#[test]
fn encode_no_fields() {
    let result = encode_frame('$', "GP", "RMC", &[]);
    assert!(result.starts_with("$GPRMC*"));
    assert!(result.ends_with("\r\n"));
}

#[test]
fn encode_then_parse_roundtrip() {
    let encoded = encode_frame(
        '$',
        "WI",
        "MWD",
        &["270.0", "T", "268.5", "M", "12.4", "N", "6.4", "M"],
    );
    assert!(encoded.starts_with("$WIMWD,"));
    assert!(encoded.ends_with("\r\n"));

    let frame = parse_frame(encoded.trim()).expect("re-parse encoded sentence");
    assert_eq!(frame.talker, "WI");
    assert_eq!(frame.sentence_type, "MWD");
    assert_eq!(frame.fields.len(), 8);
    assert_eq!(frame.fields[0], "270.0");
    assert_eq!(frame.fields[7], "M");
}

#[test]
fn error_bad_checksum() {
    let result = parse_frame("$GPRMC,175957.917,A*FF");
    assert!(
        matches!(result, Err(FrameError::BadChecksum { .. })),
        "expected BadChecksum, got {result:?}"
    );
}

#[test]
fn error_empty_input() {
    assert_eq!(parse_frame(""), Err(FrameError::Empty));
    assert_eq!(parse_frame("   "), Err(FrameError::Empty));
}

#[test]
fn error_invalid_prefix() {
    let result = parse_frame("GPRMC,175957.917,A*00");
    assert!(
        matches!(result, Err(FrameError::InvalidPrefix(_))),
        "expected InvalidPrefix, got {result:?}"
    );
}

#[test]
fn hdt_fixture_gpsd() {
    let frame = parse_frame("$HEHDT,4.0,T*2B").expect("GPSD HDT saab-r4 fixture");
    assert_eq!(frame.talker, "HE");
    assert_eq!(frame.sentence_type, "HDT");
    assert_eq!(frame.fields[0], "4.0");
    assert_eq!(frame.fields[1], "T");
}

#[test]
fn no_checksum_sentence_accepted() {
    let result =
        parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A");
    assert!(result.is_ok(), "sentence without checksum should be accepted");
    let frame = result.expect("valid frame");
    assert_eq!(frame.sentence_type, "RMC");
    assert_eq!(frame.fields[0], "175957.917");
}

#[test]
fn parse_valid_ais_sentence() {
    let frame = parse_frame("!AIVDM,1,1,,A,13u@Dt002s000000000000000000,0*60")
        .expect("valid AIS sentence");
    assert_eq!(frame.prefix, '!');
    assert_eq!(frame.talker, "AI");
    assert_eq!(frame.sentence_type, "VDM");
    assert_eq!(frame.fields[0], "1");
    assert_eq!(frame.fields[1], "1");
}

#[test]
fn parse_valid_nmea_sentence() {
    let frame =
        parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77")
            .expect("valid NMEA sentence");
    assert_eq!(frame.prefix, '$');
    assert_eq!(frame.talker, "GP");
    assert_eq!(frame.sentence_type, "RMC");
    assert_eq!(frame.fields[0], "175957.917");
    assert_eq!(frame.fields[1], "A");
    assert!(frame.tag_block.is_none());
}

#[test]
fn parse_with_tag_block() {
    let frame = parse_frame(
        "\\s:FooBar,c:1234567890*xx\\$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77",
    )
    .expect("sentence with tag block");
    assert!(frame.tag_block.is_some());
    let tag = frame.tag_block.expect("tag_block present");
    assert!(tag.contains("FooBar"));
    assert_eq!(frame.prefix, '$');
    assert_eq!(frame.sentence_type, "RMC");
}

#[test]
fn roundtrip_preserves_empty_fields() {
    let encoded = encode_frame('$', "GP", "APB", &["", "", "", "", "", "", ""]);
    let frame = parse_frame(encoded.trim()).expect("re-parse with empty fields");
    assert_eq!(frame.sentence_type, "APB");
    assert!(
        frame.fields.iter().all(|f| f.is_empty()),
        "all fields should be empty"
    );
}

#[test]
fn wind_fixture_signalk() {
    let frame =
        parse_frame("$WIMWD,270.0,T,268.5,M,12.4,N,6.4,M*63").expect("SignalK MWD fixture");
    assert_eq!(frame.talker, "WI");
    assert_eq!(frame.sentence_type, "MWD");
    assert_eq!(frame.fields[0], "270.0");
    assert_eq!(frame.fields[4], "12.4");
}
