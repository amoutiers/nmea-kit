//! AIS Types 7/13 — Binary / Safety Acknowledge.

use crate::ais::armor::extract_u32;

/// A single acknowledgement entry (MMSI + sequence number).
#[derive(Debug, Clone, PartialEq)]
pub struct AckEntry {
    pub mmsi: u32,
    pub sequence: u8,
}

/// AIS Types 7/13 — Binary / Safety Acknowledge.
///
/// Acknowledges receipt of Type 6 (binary) or Type 12 (safety) messages.
/// Contains up to 4 acknowledgement entries.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryAck {
    pub msg_type: u8,
    pub mmsi: u32,
    /// Up to 4 acknowledgement entries.
    pub acks: Vec<AckEntry>,
}

impl BinaryAck {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 72 {
            return None;
        }
        let msg_type = extract_u32(bits, 0, 6)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let mut acks = Vec::new();
        // Each ack entry is 32 bits (30 MMSI + 2 sequence), starting at bit 40
        let mut offset = 40;
        while offset + 32 <= bits.len() && acks.len() < 4 {
            let ack_mmsi = extract_u32(bits, offset, 30)?;
            let seq = extract_u32(bits, offset + 30, 2)? as u8;
            acks.push(AckEntry {
                mmsi: ack_mmsi,
                sequence: seq,
            });
            offset += 32;
        }
        Some(Self {
            msg_type,
            mmsi,
            acks,
        })
    }
}
