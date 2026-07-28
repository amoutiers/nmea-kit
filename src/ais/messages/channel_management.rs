//! AIS Type 22 — Channel management.

use crate::ais::armor::{extract_i32, extract_u32};

/// Area or destination layout selected by a Type 22 message.
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelManagementScope {
    Geographic {
        northeast_longitude: Option<f64>,
        northeast_latitude: Option<f64>,
        southwest_longitude: Option<f64>,
        southwest_latitude: Option<f64>,
    },
    Addressed {
        station_1_mmsi: u32,
        station_2_mmsi: u32,
    },
}

/// AIS Type 22 channel management message.
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelManagement {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub channel_a: u16,
    pub channel_b: u16,
    pub tx_rx_mode: u8,
    pub power_low: bool,
    pub scope: ChannelManagementScope,
    pub channel_a_bandwidth: bool,
    pub channel_b_bandwidth: bool,
    pub transitional_zone_size: u8,
}

impl ChannelManagement {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 168 {
            return None;
        }
        let addressed = extract_u32(bits, 139, 1)? == 1;
        let scope = if addressed {
            let station_1_mmsi =
                (extract_u32(bits, 69, 18)? << 12) | (extract_u32(bits, 87, 17)? >> 5);
            let station_2_mmsi =
                (extract_u32(bits, 104, 18)? << 12) | (extract_u32(bits, 122, 17)? >> 5);
            ChannelManagementScope::Addressed {
                station_1_mmsi,
                station_2_mmsi,
            }
        } else {
            ChannelManagementScope::Geographic {
                northeast_longitude: decode_longitude(extract_i32(bits, 69, 18)?),
                northeast_latitude: decode_latitude(extract_i32(bits, 87, 17)?),
                southwest_longitude: decode_longitude(extract_i32(bits, 104, 18)?),
                southwest_latitude: decode_latitude(extract_i32(bits, 122, 17)?),
            }
        };
        Some(Self {
            repeat_indicator: extract_u32(bits, 6, 2)? as u8,
            mmsi: extract_u32(bits, 8, 30)?,
            channel_a: extract_u32(bits, 40, 12)? as u16,
            channel_b: extract_u32(bits, 52, 12)? as u16,
            tx_rx_mode: extract_u32(bits, 64, 4)? as u8,
            power_low: extract_u32(bits, 68, 1)? == 1,
            scope,
            channel_a_bandwidth: extract_u32(bits, 140, 1)? == 1,
            channel_b_bandwidth: extract_u32(bits, 141, 1)? == 1,
            transitional_zone_size: extract_u32(bits, 142, 3)? as u8,
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
