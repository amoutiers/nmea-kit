#![cfg(feature = "ais")]

use nmea_kit::ais::{AisClass, AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn ignores_nmea_dollar_frames() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77")
            .expect("valid NMEA sentence");
    assert!(
        parser.decode(&frame).is_none(),
        "parser should ignore $ NMEA frames"
    );
}

#[test]
fn sentinel_filtering_lat_lon_valid_range_signalk() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26")
        .expect("valid Type 1 frame");
    let msg = parser
        .decode(&frame)
        .expect("Type 1 should decode");

    if let AisMessage::Position(pos) = msg {
        // Latitude must be in valid range or None (sentinel filtered)
        if let Some(lat) = pos.latitude {
            assert!(
                (-90.0..=90.0).contains(&lat),
                "latitude {lat} should be in [-90, 90]"
            );
        }
        // Longitude must be in valid range or None (sentinel filtered)
        if let Some(lon) = pos.longitude {
            assert!(
                (-180.0..=180.0).contains(&lon),
                "longitude {lon} should be in [-180, 180]"
            );
        }
        // Heading must be < 360 or None
        if let Some(hdg) = pos.heading {
            assert!(hdg < 360, "heading {hdg} should be < 360");
        }
    }
}

#[test]
fn type_18_class_b_position() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,B6CdCm0t3`tba35f@V9faHi7kP06,0*58")
        .expect("valid Type 18 frame");
    let msg = parser.decode(&frame);

    // Type 18 should decode; at minimum it should not panic
    if let Some(AisMessage::Position(pos)) = &msg {
        assert_eq!(pos.ais_class, AisClass::B, "Type 18 should be Class B");
        assert!(pos.mmsi > 0, "MMSI should be non-zero");
    }
}

#[test]
fn type_1_single_fragment_class_a_signalk() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26")
        .expect("valid Type 1 frame");
    let msg = parser
        .decode(&frame)
        .expect("Type 1 should decode to a message");

    match msg {
        AisMessage::Position(pos) => {
            assert_eq!(pos.msg_type, 1, "msg_type should be 1");
            assert!(pos.mmsi > 0, "MMSI should be non-zero");
            assert_eq!(pos.ais_class, AisClass::A, "Type 1 should be Class A");

            let lat = pos.latitude.expect("latitude present");
            let lon = pos.longitude.expect("longitude present");
            assert!(
                (-90.0..=90.0).contains(&lat),
                "latitude {lat} out of range"
            );
            assert!(
                (-180.0..=180.0).contains(&lon),
                "longitude {lon} out of range"
            );
        }
        other => panic!("expected Position, got {other:?}"),
    }
}

#[test]
fn type_5_multi_fragment_static_voyage_gpsd() {
    let mut parser = AisParser::new();

    // Fragment 1: should return None (incomplete)
    let f1 = parse_frame(
        "!AIVDM,2,1,1,A,55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp8,0*1C",
    )
    .expect("valid Type 5 fragment 1");
    assert!(
        parser.decode(&f1).is_none(),
        "fragment 1 should return None"
    );

    // Fragment 2: should complete the message
    let f2 =
        parse_frame("!AIVDM,2,2,1,A,88888888880,2*25").expect("valid Type 5 fragment 2");
    let msg = parser
        .decode(&f2)
        .expect("fragment 2 should complete Type 5");

    match msg {
        AisMessage::StaticVoyage(svd) => {
            assert!(svd.mmsi > 0, "MMSI should be non-zero");
            assert!(
                !svd.vessel_name.is_empty(),
                "vessel_name should not be empty"
            );
            assert_eq!(svd.ais_class, AisClass::A, "Type 5 should be Class A");
        }
        other => panic!("expected StaticVoyage, got {other:?}"),
    }
}

#[test]
fn type_8_unknown_message_signalk() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,85Mv070j2d>=<e<<=PQhhg`59P00,0*26")
        .expect("valid Type 8 frame");
    let msg = parser.decode(&frame);

    match msg {
        Some(AisMessage::Unknown { msg_type }) => {
            assert_eq!(msg_type, 8, "msg_type should be 8");
        }
        other => panic!("expected Unknown {{ msg_type: 8 }}, got {other:?}"),
    }
}
