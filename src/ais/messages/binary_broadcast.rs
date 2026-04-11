//! AIS Type 8 — Binary Broadcast Message.

use crate::ais::armor::extract_u32;

/// AIS Type 8 — Binary Broadcast Message.
///
/// Broadcasts application-specific data identified by DAC and FID.
/// The `data` field contains raw bits (one byte per bit).
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryBroadcast {
    pub mmsi: u32,
    /// Designated Area Code.
    pub dac: u16,
    /// Functional ID.
    pub fid: u8,
    /// Application-specific binary data (raw bits).
    pub data: Vec<u8>,
}

impl BinaryBroadcast {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 56 {
            return None;
        }
        let mmsi = extract_u32(bits, 8, 30)?;
        let dac = extract_u32(bits, 40, 10)? as u16;
        let fid = extract_u32(bits, 50, 6)? as u8;
        let data = if bits.len() > 56 {
            bits[56..].to_vec()
        } else {
            Vec::new()
        };
        Some(Self {
            mmsi,
            dac,
            fid,
            data,
        })
    }
}
