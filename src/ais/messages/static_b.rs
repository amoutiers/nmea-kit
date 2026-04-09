//! AIS static data report — Type 24 (Class B).

use crate::ais::armor::{extract_string, extract_u32};

/// AIS Static Data Report — Type 24 (Class B).
///
/// Type 24 comes in two parts:
/// - Part A: vessel name
/// - Part B: callsign + ship type
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum StaticDataReport {
    PartA {
        mmsi: u32,
        vessel_name: String,
    },
    PartB {
        mmsi: u32,
        callsign: String,
        ship_type: u8,
    },
}

impl StaticDataReport {
    /// Decode a Type 24 static data report.
    pub fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 160 {
            return None;
        }

        let mmsi = extract_u32(bits, 8, 30)?;
        let part_number = extract_u32(bits, 38, 2)?;

        match part_number {
            0 => {
                // Part A: vessel name
                let vessel_name = extract_string(bits, 40, 20)?;
                Some(Self::PartA { mmsi, vessel_name })
            }
            1 => {
                // Part B: callsign + ship type
                if bits.len() < 168 {
                    return None;
                }
                let callsign = extract_string(bits, 40, 7)?;
                let ship_type = extract_u32(bits, 82, 8)? as u8;
                Some(Self::PartB {
                    mmsi,
                    callsign,
                    ship_type,
                })
            }
            _ => None,
        }
    }
}
