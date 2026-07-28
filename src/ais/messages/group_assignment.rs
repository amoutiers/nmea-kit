//! AIS Type 23 — Group assignment command.

use crate::ais::armor::{extract_i32, extract_u32};

/// AIS Type 23 group assignment command.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupAssignment {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub northeast_longitude: Option<f64>,
    pub northeast_latitude: Option<f64>,
    pub southwest_longitude: Option<f64>,
    pub southwest_latitude: Option<f64>,
    pub station_type: u8,
    pub ship_type: u8,
    pub tx_rx_mode: u8,
    pub reporting_interval: u8,
    pub quiet_time: u8,
}

impl GroupAssignment {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 160 {
            return None;
        }
        Some(Self {
            repeat_indicator: extract_u32(bits, 6, 2)? as u8,
            mmsi: extract_u32(bits, 8, 30)?,
            northeast_longitude: decode_longitude(extract_i32(bits, 40, 18)?),
            northeast_latitude: decode_latitude(extract_i32(bits, 58, 17)?),
            southwest_longitude: decode_longitude(extract_i32(bits, 75, 18)?),
            southwest_latitude: decode_latitude(extract_i32(bits, 93, 17)?),
            station_type: extract_u32(bits, 110, 4)? as u8,
            ship_type: extract_u32(bits, 114, 8)? as u8,
            tx_rx_mode: extract_u32(bits, 144, 2)? as u8,
            reporting_interval: extract_u32(bits, 146, 4)? as u8,
            quiet_time: extract_u32(bits, 150, 4)? as u8,
        })
    }
}

fn decode_longitude(value: i32) -> Option<f64> {
    let value = f64::from(value) / 600.0;
    (-180.0..=180.0).contains(&value).then_some(value)
}

fn decode_latitude(value: i32) -> Option<f64> {
    let value = f64::from(value) / 600.0;
    (-90.0..=90.0).contains(&value).then_some(value)
}
