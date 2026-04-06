//! AIS position report — Type 18 (Class B standard).

use crate::ais::armor::{extract_i32, extract_u32};

use super::common::AisClass;
use super::position_a::{
    PositionReport, decode_cog, decode_heading, decode_latitude, decode_longitude, decode_sog,
};

impl PositionReport {
    /// Decode a Type 18 Class B standard position report.
    pub fn decode_class_b(bits: &[u8]) -> Option<Self> {
        if bits.len() < 168 {
            return None;
        }

        let msg_type = extract_u32(bits, 0, 6)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let sog_raw = extract_u32(bits, 46, 10)?;
        let accuracy = extract_u32(bits, 56, 1)? == 1;
        let lon_raw = extract_i32(bits, 57, 28)?;
        let lat_raw = extract_i32(bits, 85, 27)?;
        let cog_raw = extract_u32(bits, 112, 12)?;
        let hdg_raw = extract_u32(bits, 124, 9)?;
        let ts_raw = extract_u32(bits, 133, 6)? as u8;

        Some(Self {
            msg_type,
            mmsi,
            nav_status: None,
            rate_of_turn: None,
            sog: decode_sog(sog_raw),
            position_accuracy: accuracy,
            longitude: decode_longitude(lon_raw),
            latitude: decode_latitude(lat_raw),
            cog: decode_cog(cog_raw),
            heading: decode_heading(hdg_raw),
            timestamp: if ts_raw < 60 { Some(ts_raw) } else { None },
            ais_class: AisClass::B,
        })
    }
}
