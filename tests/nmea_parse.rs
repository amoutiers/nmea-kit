#![cfg(feature = "nmea")]

use nmea_kit::{NmeaSentence, parse_frame};

#[cfg(feature = "dpt")]
#[test]
fn parse_dpt_depth_signalk() {
    let frame = parse_frame("$IIDPT,4.1,0.0*45").expect("valid DPT fixture");
    let sentence = NmeaSentence::parse(&frame);
    match sentence {
        NmeaSentence::Dpt(dpt) => {
            let depth = dpt.depth.expect("depth present");
            assert!((depth - 4.1).abs() < 0.01);
        }
        other => panic!("expected Dpt, got {other:?}"),
    }
}

#[cfg(feature = "hdt")]
#[test]
fn parse_hdt_heading_true_gpsd() {
    let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid HDT fixture");
    let sentence = NmeaSentence::parse(&frame);
    match sentence {
        NmeaSentence::Hdt(hdt) => {
            let heading = hdt.heading_true.expect("heading_true present");
            assert!((heading - 4.0).abs() < 0.01);
        }
        other => panic!("expected Hdt, got {other:?}"),
    }
}

#[cfg(feature = "mwd")]
#[test]
fn parse_mwd_wind_direction_signalk() {
    let frame = parse_frame("$IIMWD,,,046.,M,10.1,N,05.2,M*0B").expect("valid MWD fixture");
    let sentence = NmeaSentence::parse(&frame);
    match sentence {
        NmeaSentence::Mwd(mwd) => {
            assert!(mwd.wind_dir_true.is_none(), "true dir should be empty");
            let mag = mwd.wind_dir_mag.expect("magnetic direction present");
            assert!(
                (mag - 46.0).abs() < 0.1,
                "wind_dir_mag should be ~46.0, got {mag}"
            );
        }
        other => panic!("expected Mwd, got {other:?}"),
    }
}

#[cfg(feature = "rmc")]
#[test]
fn parse_rmc_position_signalk() {
    let frame =
        parse_frame("$GPRMC,085412.000,A,5222.3198,N,00454.5784,E,0.58,251.34,030414,,,A*65")
            .expect("valid RMC fixture");
    let sentence = NmeaSentence::parse(&frame);
    match sentence {
        NmeaSentence::Rmc(rmc) => {
            assert_eq!(rmc.status, Some('A'));
            assert!(rmc.lat.is_some());
        }
        other => panic!("expected Rmc, got {other:?}"),
    }
}

#[test]
fn parse_unknown_sentence_type() {
    let frame = parse_frame("$GPXYZ,1,2,3").expect("valid unknown sentence");
    let sentence = NmeaSentence::parse(&frame);
    match sentence {
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

#[cfg(feature = "vtg")]
#[test]
fn parse_vtg_course_and_speed_signalk() {
    let frame = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid VTG fixture");
    let sentence = NmeaSentence::parse(&frame);
    match sentence {
        NmeaSentence::Vtg(vtg) => {
            let course = vtg.course_true.expect("course_true present");
            assert!((course - 0.0).abs() < 0.01);
            assert_eq!(vtg.mode, Some('A'));
        }
        other => panic!("expected Vtg, got {other:?}"),
    }
}

#[test]
fn unsupported_types_become_unknown_pynmeagps() {
    let frame = parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C")
        .expect("valid APB fixture");
    let sentence = NmeaSentence::parse(&frame);
    match sentence {
        NmeaSentence::Unknown { sentence_type, .. } => assert_eq!(sentence_type, "APB"),
        other => panic!("expected Unknown for APB, got {other:?}"),
    }
}
