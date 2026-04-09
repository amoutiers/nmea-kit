//! AIS safety-related broadcast message — Type 14.
//!
//! Transmitted by vessels and shore stations (coast guard, port authority) to broadcast
//! safety-critical text messages. Always single-frame.

use crate::ais::armor::{extract_string, extract_u32};

/// AIS Type 14 — Safety-Related Broadcast Message.
///
/// ITU-R M.1371 bit layout:
/// - bits  0–5:  message type (= 14)
/// - bits  6–7:  repeat indicator
/// - bits  8–37: MMSI (30 bits)
/// - bits 38–39: spare
/// - bits 40+:   safety text (6-bit ASCII, up to 161 chars / 968 bits)
#[derive(Debug, Clone, PartialEq)]
pub struct SafetyBroadcast {
    /// MMSI of the transmitting station.
    pub mmsi: u32,
    /// Safety-related text message, trimmed of trailing padding.
    pub text: String,
}

impl SafetyBroadcast {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 40 {
            return None;
        }
        let mmsi = extract_u32(bits, 8, 30)?;
        let char_count = bits.len().saturating_sub(40) / 6;
        let text = if char_count > 0 {
            extract_string(bits, 40, char_count)?.trim().to_string()
        } else {
            String::new()
        };
        Some(Self { mmsi, text })
    }
}
