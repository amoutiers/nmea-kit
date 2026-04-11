//! AIS Type 6 — Addressed Binary Message.

use crate::ais::armor::extract_u32;

/// AIS Type 6 — Addressed Binary Message.
///
/// Carries application-specific data identified by DAC (Designated Area Code)
/// and FID (Functional ID). The `data` field contains raw bits (one byte per bit).
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryAddressed {
    pub mmsi: u32,
    /// Sequence number (0-3).
    pub sequence: u8,
    /// Destination MMSI.
    pub dest_mmsi: u32,
    /// Retransmit flag.
    pub retransmit: bool,
    /// Designated Area Code.
    pub dac: u16,
    /// Functional ID.
    pub fid: u8,
    /// Application-specific binary data (raw bits).
    pub data: Vec<u8>,
}

impl BinaryAddressed {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 88 {
            return None;
        }
        let mmsi = extract_u32(bits, 8, 30)?;
        let sequence = extract_u32(bits, 38, 2)? as u8;
        let dest_mmsi = extract_u32(bits, 40, 30)?;
        let retransmit = extract_u32(bits, 70, 1)? == 1;
        let dac = extract_u32(bits, 72, 10)? as u16;
        let fid = extract_u32(bits, 82, 6)? as u8;
        let data = if bits.len() > 88 {
            bits[88..].to_vec()
        } else {
            Vec::new()
        };
        Some(Self {
            mmsi,
            sequence,
            dest_mmsi,
            retransmit,
            dac,
            fid,
            data,
        })
    }
}
