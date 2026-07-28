//! AIS Type 25 — Single slot binary message.

use crate::ais::armor::extract_u32;

/// AIS Type 25 single-slot binary message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinarySingleSlot {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub destination_mmsi: Option<u32>,
    /// `true` when an application identifier prefixes the binary data.
    pub binary_data_flag: bool,
    /// Raw binary payload, one bit per byte.
    pub data: Vec<u8>,
}

impl BinarySingleSlot {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 40 {
            return None;
        }
        let addressed = extract_u32(bits, 38, 1)? == 1;
        let data_start = if addressed { 72 } else { 40 };
        if bits.len() < data_start {
            return None;
        }
        Some(Self {
            repeat_indicator: extract_u32(bits, 6, 2)? as u8,
            mmsi: extract_u32(bits, 8, 30)?,
            destination_mmsi: if addressed {
                Some(extract_u32(bits, 40, 30)?)
            } else {
                None
            },
            binary_data_flag: extract_u32(bits, 39, 1)? == 1,
            data: bits[data_start..].to_vec(),
        })
    }
}
