//! AIS Type 15 — Interrogation.

use crate::ais::armor::extract_u32;

/// AIS Type 15 — Interrogation.
///
/// Requests specific message types from one or two stations.
/// Variable length: 88–160 bits.
#[derive(Debug, Clone, PartialEq)]
pub struct Interrogation {
    pub mmsi: u32,
    /// First interrogated MMSI.
    pub mmsi_1: u32,
    /// First requested message type.
    pub msg_type_1_1: u8,
    /// First slot offset.
    pub slot_offset_1_1: u16,
    /// Second requested message type from station 1 (if present).
    pub msg_type_1_2: Option<u8>,
    /// Second slot offset from station 1 (if present).
    pub slot_offset_1_2: Option<u16>,
    /// Second interrogated MMSI (if present).
    pub mmsi_2: Option<u32>,
    /// Requested message type from station 2 (if present).
    pub msg_type_2_1: Option<u8>,
    /// Slot offset from station 2 (if present).
    pub slot_offset_2_1: Option<u16>,
}

impl Interrogation {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 88 {
            return None;
        }
        let mmsi = extract_u32(bits, 8, 30)?;
        let mmsi_1 = extract_u32(bits, 40, 30)?;
        let msg_type_1_1 = extract_u32(bits, 70, 6)? as u8;
        let slot_offset_1_1 = extract_u32(bits, 76, 12)? as u16;

        // Optional second message type for station 1
        let (msg_type_1_2, slot_offset_1_2) = if bits.len() >= 108 {
            let mt = extract_u32(bits, 90, 6)? as u8;
            let so = extract_u32(bits, 96, 12)? as u16;
            if mt == 0 {
                (None, None)
            } else {
                (Some(mt), Some(so))
            }
        } else {
            (None, None)
        };

        // Optional second station
        let (mmsi_2, msg_type_2_1, slot_offset_2_1) = if bits.len() >= 158 {
            let m2 = extract_u32(bits, 110, 30)?;
            let mt = extract_u32(bits, 140, 6)? as u8;
            let so = extract_u32(bits, 146, 12)? as u16;
            if m2 == 0 {
                (None, None, None)
            } else {
                (Some(m2), Some(mt), Some(so))
            }
        } else {
            (None, None, None)
        };

        Some(Self {
            mmsi,
            mmsi_1,
            msg_type_1_1,
            slot_offset_1_1,
            msg_type_1_2,
            slot_offset_1_2,
            mmsi_2,
            msg_type_2_1,
            slot_offset_2_1,
        })
    }
}
