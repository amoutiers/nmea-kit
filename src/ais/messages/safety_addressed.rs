//! AIS Type 12 — Addressed Safety-Related Message.

use crate::ais::armor::{extract_string, extract_u32};

/// AIS Type 12 — Addressed Safety-Related Message.
///
/// Like Type 14 but addressed to a specific MMSI with sequence number and retransmit flag.
/// Variable length: 72 bits minimum, up to 1008 bits with safety text.
#[derive(Debug, Clone, PartialEq)]
pub struct SafetyAddressed {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    /// Sequence number (0-3).
    pub sequence: u8,
    /// Destination MMSI.
    pub dest_mmsi: u32,
    /// Retransmit flag.
    pub retransmit: bool,
    /// Safety-related text message.
    pub text: String,
}

impl SafetyAddressed {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 72 {
            return None;
        }
        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let sequence = extract_u32(bits, 38, 2)? as u8;
        let dest_mmsi = extract_u32(bits, 40, 30)?;
        let retransmit = extract_u32(bits, 70, 1)? == 1;
        let char_count = bits.len().saturating_sub(72) / 6;
        let text = if char_count > 0 {
            extract_string(bits, 72, char_count)?
        } else {
            String::new()
        };
        Some(Self {
            repeat_indicator,
            mmsi,
            sequence,
            dest_mmsi,
            retransmit,
            text,
        })
    }
}
