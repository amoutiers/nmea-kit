//! AIS Type 11 — UTC/Date Response.

use super::utils::{decode_latitude, decode_longitude};
use crate::ais::armor::{extract_i32, extract_u32};

/// AIS Type 11 — UTC/Date Response.
///
/// Same bit layout as Type 4 (168 bits). Sent by mobile stations in response to Type 10 interrogation.
#[derive(Debug, Clone, PartialEq)]
pub struct UtcDateResponse {
    pub mmsi: u32,
    pub year: Option<u16>,  // 0 = not available
    pub month: Option<u8>,  // 0 = not available
    pub day: Option<u8>,    // 0 = not available
    pub hour: Option<u8>,   // 24 = not available
    pub minute: Option<u8>, // 60 = not available
    pub second: Option<u8>, // 60 = not available
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub type_of_epfd: u8,
}

impl UtcDateResponse {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 168 {
            return None;
        }
        let mmsi = extract_u32(bits, 8, 30)?;
        let year_raw = extract_u32(bits, 38, 14)?;
        let month_raw = extract_u32(bits, 52, 4)? as u8;
        let day_raw = extract_u32(bits, 56, 5)? as u8;
        let hour_raw = extract_u32(bits, 61, 5)? as u8;
        let minute_raw = extract_u32(bits, 66, 6)? as u8;
        let second_raw = extract_u32(bits, 72, 6)? as u8;
        let accuracy = extract_u32(bits, 78, 1)? == 1;
        let lon_raw = extract_i32(bits, 79, 28)?;
        let lat_raw = extract_i32(bits, 107, 27)?;
        let epfd = extract_u32(bits, 134, 4)? as u8;
        Some(Self {
            mmsi,
            year: if year_raw == 0 {
                None
            } else {
                Some(year_raw as u16)
            },
            month: if month_raw == 0 {
                None
            } else {
                Some(month_raw)
            },
            day: if day_raw == 0 { None } else { Some(day_raw) },
            hour: if hour_raw == 24 { None } else { Some(hour_raw) },
            minute: if minute_raw == 60 {
                None
            } else {
                Some(minute_raw)
            },
            second: if second_raw == 60 {
                None
            } else {
                Some(second_raw)
            },
            position_accuracy: accuracy,
            longitude: decode_longitude(lon_raw),
            latitude: decode_latitude(lat_raw),
            type_of_epfd: epfd,
        })
    }
}
