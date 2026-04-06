//! AIS static and voyage data — Type 5 (Class A).

use crate::ais::armor::{extract_string, extract_u32};

use super::common::AisClass;

/// AIS Static and Voyage Data — Type 5 (Class A).
#[derive(Debug, Clone, PartialEq)]
pub struct StaticVoyageData {
    pub mmsi: u32,
    pub imo: u32,
    pub callsign: String,
    pub vessel_name: String,
    pub ship_type: u8,
    pub ais_class: AisClass,
}

impl StaticVoyageData {
    /// Decode a Type 5 static and voyage data message.
    pub fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 424 {
            return None;
        }

        let mmsi = extract_u32(bits, 8, 30)?;
        let imo = extract_u32(bits, 40, 30)?;
        let callsign = extract_string(bits, 70, 7)?;
        let vessel_name = extract_string(bits, 112, 20)?;
        let ship_type = extract_u32(bits, 232, 8)? as u8;

        Some(Self {
            mmsi,
            imo,
            callsign,
            vessel_name,
            ship_type,
            ais_class: AisClass::A,
        })
    }
}
