//! AIS position report — Types 1, 2, 3 (Class A).

use crate::ais::armor::{extract_i32, extract_u32};

use super::common::{AisClass, NavigationStatus};
use super::utils::{
    decode_cog, decode_heading, decode_latitude, decode_longitude, decode_rot, decode_sog,
};

/// AIS Position Report — Types 1, 2, 3 (Class A) and 18 (Class B) and 19 (B+).
#[derive(Debug, Clone, PartialEq)]
pub struct PositionReport {
    pub msg_type: u8,
    pub mmsi: u32,
    pub nav_status: Option<NavigationStatus>,
    /// Rate of turn — raw ITU ROT_AIS indicator (not deg/min). None if not available.
    pub rate_of_turn: Option<f32>,
    pub sog: Option<f32>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub cog: Option<f32>,
    /// True heading in degrees (integer, 0-359). AIS has no fractional resolution for heading.
    pub heading: Option<u16>,
    pub timestamp: Option<u8>,
    pub ais_class: AisClass,
}

impl PositionReport {
    /// Decode a Type 1/2/3 Class A position report from AIS bits.
    pub fn decode_class_a(bits: &[u8]) -> Option<Self> {
        if bits.len() < 168 {
            return None;
        }

        let msg_type = extract_u32(bits, 0, 6)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let nav_status_raw = extract_u32(bits, 38, 4)? as u8;
        let rot_raw = extract_i32(bits, 42, 8)?;
        let sog_raw = extract_u32(bits, 50, 10)?;
        let accuracy = extract_u32(bits, 60, 1)? == 1;
        let lon_raw = extract_i32(bits, 61, 28)?;
        let lat_raw = extract_i32(bits, 89, 27)?;
        let cog_raw = extract_u32(bits, 116, 12)?;
        let hdg_raw = extract_u32(bits, 128, 9)?;
        let ts_raw = extract_u32(bits, 137, 6)? as u8;

        Some(Self {
            msg_type,
            mmsi,
            nav_status: Some(NavigationStatus::from(nav_status_raw)),
            rate_of_turn: decode_rot(rot_raw),
            sog: decode_sog(sog_raw),
            position_accuracy: accuracy,
            longitude: decode_longitude(lon_raw),
            latitude: decode_latitude(lat_raw),
            cog: decode_cog(cog_raw),
            heading: decode_heading(hdg_raw),
            timestamp: if ts_raw < 60 { Some(ts_raw) } else { None },
            ais_class: AisClass::A,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn type1_sentinels_decode_to_none() {
        // Build a 168-bit Type-1 buffer with every "not available" sentinel raw.
        let mut bits = vec![0u8; 168];
        set_bits(&mut bits, 0, 6, 1); // msg_type 1
        set_bits(&mut bits, 42, 8, 0x80); // ROT raw -128 (two's complement in 8 bits)
        set_bits(&mut bits, 50, 10, 1023); // SOG not available
        set_bits(&mut bits, 61, 28, 108_600_000); // lon 181deg (181 * 600000) - not available
        set_bits(&mut bits, 89, 27, 54_600_000); // lat 91deg (91 * 600000) - not available
        set_bits(&mut bits, 116, 12, 3600); // COG not available
        set_bits(&mut bits, 128, 9, 511); // heading not available

        let pos = PositionReport::decode_class_a(&bits).expect("decodes");
        assert_eq!(pos.rate_of_turn, None);
        assert_eq!(pos.sog, None);
        assert_eq!(pos.longitude, None);
        assert_eq!(pos.latitude, None);
        assert_eq!(pos.cog, None);
        assert_eq!(pos.heading, None);
    }
}
