//! AIS position report — Types 1, 2, 3 (Class A).

use crate::ais::armor::{extract_i32, extract_u32};

use super::common::{AisClass, NavigationStatus};

/// AIS Position Report — Types 1, 2, 3 (Class A) and 18 (Class B) and 19 (B+).
#[derive(Debug, Clone, PartialEq)]
pub struct PositionReport {
    pub msg_type: u8,
    pub mmsi: u32,
    pub nav_status: Option<NavigationStatus>,
    pub rate_of_turn: Option<f32>,
    pub sog: Option<f32>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub cog: Option<f32>,
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

// --- Decoding helpers (shared by types 18 and 19) ---

/// Decode latitude from 1/10000 minute to degrees. 91° = not available.
pub(crate) fn decode_latitude(raw: i32) -> Option<f64> {
    let deg = f64::from(raw) / 600_000.0;
    if !(-90.0..=90.0).contains(&deg) {
        None
    } else {
        Some(deg)
    }
}

/// Decode longitude from 1/10000 minute to degrees. 181° = not available.
pub(crate) fn decode_longitude(raw: i32) -> Option<f64> {
    let deg = f64::from(raw) / 600_000.0;
    if !(-180.0..=180.0).contains(&deg) {
        None
    } else {
        Some(deg)
    }
}

/// Decode SOG from 1/10 knot. 1023 = not available.
pub(crate) fn decode_sog(raw: u32) -> Option<f32> {
    if raw == 1023 {
        None
    } else {
        Some(raw as f32 / 10.0)
    }
}

/// Decode COG from 1/10 degree. 3600 = not available.
pub(crate) fn decode_cog(raw: u32) -> Option<f32> {
    if raw == 3600 {
        None
    } else {
        Some(raw as f32 / 10.0)
    }
}

/// Decode true heading. 511 = not available.
pub(crate) fn decode_heading(raw: u32) -> Option<u16> {
    if raw == 511 {
        None
    } else {
        Some(raw as u16)
    }
}

/// Decode rate of turn. -128 = not available.
pub(crate) fn decode_rot(raw: i32) -> Option<f32> {
    if raw == -128 {
        None
    } else {
        Some(raw as f32)
    }
}
