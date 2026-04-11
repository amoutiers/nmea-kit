#![cfg(feature = "ais")]
use nmea_kit::ais::{AisMessage, AisParser};
use nmea_kit::parse_frame;

#[test]
fn type8_binary_broadcast_gpsd() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,85Mwp`1Kf3aCnsNvBWLi=wQuNhA5t43N`5nCuI=p<IBfVqnMgPGs,0*47")
            .expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
    if let AisMessage::BinaryBroadcast(bb) = msg {
        assert!(bb.mmsi > 0);
        assert!(!bb.data.is_empty(), "should have binary data");
    } else {
        panic!("expected BinaryBroadcast (type 8), got {msg:?}");
    }
}

#[test]
fn type8_dac_fid_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,83aDChPj2d<dL<uM=hhhI?a@6HP0,0*40").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
    if let AisMessage::BinaryBroadcast(bb) = msg {
        assert!(bb.dac > 0 || bb.fid > 0, "DAC or FID should be nonzero");
    } else {
        panic!("expected BinaryBroadcast, got {msg:?}");
    }
}
