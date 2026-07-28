use nmea_kit::ais::messages::NavigationStatus;
use nmea_kit::ais::transmit::{
    AisChannel, AisEncodable, AisTransmitOptions, ClassAPosition, ClassAPositionType,
    ClassAStaticVoyage, ClassBCommunicationState, ClassBPosition, ClassBStaticPartA,
    ClassBStaticPartB,
};
use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::{EncodeError, parse_frame};

#[test]
fn class_a_position_encodes_to_a_decodable_vdm_sentence() {
    let report = ClassAPosition {
        message_type: ClassAPositionType::PositionReport,
        repeat_indicator: 0,
        mmsi: 244_670_316,
        navigation_status: NavigationStatus::UnderWayEngine,
        rate_of_turn: None,
        sog: Some(10.0),
        position_accuracy: true,
        longitude: Some(4.379_285),
        latitude: Some(51.894_75),
        cog: Some(70.6),
        heading: Some(71),
        timestamp: Some(5),
        maneuver_indicator: 1,
        raim: false,
        communication_state: 0,
    };

    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("encode type 1");
    assert_eq!(lines.len(), 1);
    assert!(lines[0].starts_with("!AIVDM,1,1,,A,"));
    assert!(lines[0].len() <= 82);

    let frame = parse_frame(&lines[0]).expect("parse encoded type 1");
    let decoded = AisParser::new().decode(&frame).expect("decode type 1");
    match decoded {
        AisMessage::Position(position) => {
            assert_eq!(position.msg_type, 1);
            assert_eq!(position.mmsi, report.mmsi);
            assert_eq!(position.latitude, report.latitude);
            assert_eq!(position.longitude, report.longitude);
            assert_eq!(position.cog, report.cog);
            assert_eq!(position.heading, report.heading);
        }
        other => panic!("expected position report, got {other:?}"),
    }
}

#[test]
fn class_a_static_voyage_encodes_to_two_decodable_vdm_fragments() {
    let report = ClassAStaticVoyage {
        repeat_indicator: 0,
        mmsi: 366_123_456,
        ais_version: 3,
        imo: 9_876_543,
        callsign: "WDC1234".to_string(),
        vessel_name: "TEST VESSEL".to_string(),
        ship_type: 70,
        dimension_to_bow: 100,
        dimension_to_stern: 30,
        dimension_to_port: 10,
        dimension_to_starboard: 10,
        position_fixing_device: 1,
        eta_month: Some(7),
        eta_day: Some(28),
        eta_hour: Some(12),
        eta_minute: Some(30),
        draught_meters: Some(6.5),
        destination: "LE HAVRE".to_string(),
        dte: false,
    };

    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(3))
        .expect("encode type 5");
    assert_eq!(lines.len(), 2);
    assert!(lines.iter().all(|line| line.len() <= 82));

    let mut parser = AisParser::new();
    assert!(
        parser
            .decode(&parse_frame(&lines[0]).expect("parse first type 5 fragment"))
            .is_none()
    );
    let decoded = parser
        .decode(&parse_frame(&lines[1]).expect("parse second type 5 fragment"))
        .expect("decode type 5");
    match decoded {
        AisMessage::StaticVoyage(data) => {
            assert_eq!(data.mmsi, report.mmsi);
            assert_eq!(data.imo, report.imo);
            assert_eq!(data.callsign, report.callsign);
            assert_eq!(data.vessel_name, report.vessel_name);
            assert_eq!(data.ship_type, report.ship_type);
        }
        other => panic!("expected static voyage data, got {other:?}"),
    }
}

#[test]
fn class_b_position_encodes_to_a_decodable_vdm_sentence() {
    let report = ClassBPosition {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        sog: Some(5.4),
        position_accuracy: true,
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        cog: Some(91.2),
        heading: Some(91),
        timestamp: Some(15),
        transmit_power_low: false,
        class_b_cs: true,
        display_available: true,
        dsc_capable: false,
        full_band_capable: true,
        message_22_capable: true,
        assigned_mode: false,
        raim: false,
        communication_state: ClassBCommunicationState::Itdma(0),
    };

    let mut invalid_heading = report.clone();
    invalid_heading.heading = Some(360);
    assert_eq!(
        invalid_heading.to_sentences(AisTransmitOptions::vdm(AisChannel::B)),
        Err(EncodeError::InvalidAisField("heading"))
    );

    let mut invalid_timestamp = report.clone();
    invalid_timestamp.timestamp = Some(61);
    assert_eq!(
        invalid_timestamp.to_sentences(AisTransmitOptions::vdm(AisChannel::B)),
        Err(EncodeError::InvalidAisField("timestamp"))
    );

    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::B))
        .expect("encode type 18");
    assert_eq!(lines.len(), 1);
    assert!(lines[0].starts_with("!AIVDM,1,1,,B,"));
    assert!(lines[0].len() <= 82);

    let decoded = AisParser::new()
        .decode(&parse_frame(&lines[0]).expect("parse encoded type 18"))
        .expect("decode type 18");
    match decoded {
        AisMessage::Position(position) => {
            assert_eq!(position.msg_type, 18);
            assert_eq!(position.mmsi, report.mmsi);
            assert_eq!(position.latitude, report.latitude);
            assert_eq!(position.longitude, report.longitude);
            assert_eq!(position.cog, report.cog);
            assert_eq!(position.heading, report.heading);
        }
        other => panic!("expected Class B position report, got {other:?}"),
    }
}

#[test]
fn class_b_static_parts_encode_to_decodable_vdm_sentences() {
    let part_a = ClassBStaticPartA {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        vessel_name: "SIM BOAT".to_string(),
    };
    let part_b = ClassBStaticPartB {
        repeat_indicator: 0,
        mmsi: part_a.mmsi,
        ship_type: 37,
        manufacturer_id: "SIM".to_string(),
        model_code: 1,
        serial_number: 42,
        callsign: "FTEST01".to_string(),
        dimension_to_bow: 8,
        dimension_to_stern: 4,
        dimension_to_port: 2,
        dimension_to_starboard: 2,
        position_fixing_device: 1,
        vdes_capabilities: 0,
    };

    let part_a_line = part_a
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("encode type 24A");
    let part_b_line = part_b
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("encode type 24B");
    assert_eq!(part_a_line.len(), 1);
    assert_eq!(part_b_line.len(), 1);
    assert!(part_a_line[0].len() <= 82);
    assert!(part_b_line[0].len() <= 82);

    let mut parser = AisParser::new();
    let decoded_a = parser
        .decode(&parse_frame(&part_a_line[0]).expect("parse type 24A"))
        .expect("decode type 24A");
    let decoded_b = parser
        .decode(&parse_frame(&part_b_line[0]).expect("parse type 24B"))
        .expect("decode type 24B");
    assert!(matches!(
        decoded_a,
        AisMessage::StaticReport(nmea_kit::ais::StaticDataReport::PartA { mmsi, vessel_name })
            if mmsi == part_a.mmsi && vessel_name == part_a.vessel_name
    ));
    assert!(matches!(
        decoded_b,
        AisMessage::StaticReport(nmea_kit::ais::StaticDataReport::PartB { mmsi, callsign, ship_type })
            if mmsi == part_b.mmsi && callsign == part_b.callsign && ship_type == part_b.ship_type
    ));
}

#[test]
fn class_a_static_voyage_requires_sequence_id_for_fragments() {
    let report = ClassAStaticVoyage {
        repeat_indicator: 0,
        mmsi: 366_123_456,
        ais_version: 0,
        imo: 0,
        callsign: String::new(),
        vessel_name: "SIMULATOR".to_string(),
        ship_type: 0,
        dimension_to_bow: 0,
        dimension_to_stern: 0,
        dimension_to_port: 0,
        dimension_to_starboard: 0,
        position_fixing_device: 0,
        eta_month: None,
        eta_day: None,
        eta_hour: None,
        eta_minute: None,
        draught_meters: None,
        destination: String::new(),
        dte: false,
    };

    assert_eq!(
        report.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::MissingAisSequenceId)
    );
}

#[test]
fn ais_transmitter_rejects_invalid_field_values() {
    let invalid_text = ClassBStaticPartA {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        vessel_name: "navire_école".to_string(),
    };
    assert_eq!(
        invalid_text.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::InvalidAisField("vessel_name"))
    );

    let too_long = ClassBStaticPartA {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        vessel_name: "A".repeat(21),
    };
    assert_eq!(
        too_long.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::AisTextTooLong {
            field: "vessel_name",
            max_chars: 20,
            actual_chars: 21,
        })
    );

    let invalid_range = ClassBStaticPartA {
        repeat_indicator: 4,
        mmsi: 235_009_217,
        vessel_name: "SIM BOAT".to_string(),
    };
    assert_eq!(
        invalid_range.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::InvalidAisField("repeat_indicator"))
    );
}
