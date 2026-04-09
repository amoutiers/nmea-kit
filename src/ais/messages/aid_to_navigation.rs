//! AIS aid-to-navigation report — Type 21.
//!
//! Transmitted by AIS-equipped buoys, lighthouses, and beacons. Provides real-time
//! position and identity for navigational aids. Always single-frame.

use crate::ais::armor::{extract_i32, extract_string, extract_u32};

use super::utils::{decode_latitude, decode_longitude};

/// AIS Type 21 — Aid-to-Navigation Report.
///
/// ITU-R M.1371 bit layout (minimum 272 bits):
/// - bits   0–5:   message type (= 21)
/// - bits   6–7:   repeat indicator
/// - bits   8–37:  MMSI (30 bits)
/// - bits  38–42:  type of AID (5 bits, 1–31)
/// - bits  43–162: name (20 × 6-bit ASCII chars)
/// - bit   163:    position accuracy
/// - bits 164–191: longitude (28 bits, 1/10000 min, same encoding as Type 1)
/// - bits 192–218: latitude  (27 bits, 1/10000 min, same encoding as Type 1)
#[derive(Debug, Clone, PartialEq)]
pub struct AidToNavigation {
    /// MMSI of the aid-to-navigation transponder.
    pub mmsi: u32,
    /// Type of navigational aid (ITU-R M.1371 Table 67).
    /// E.g. 1=default/unspecified, 16=buoy, 20=LANBY, 31=IALA special mark.
    pub aid_type: u8,
    /// Name of the aid (e.g. "PORTLAND BILL LT"), trimmed of padding.
    pub name: String,
    /// Latitude in decimal degrees. `None` if not available (sentinel 91°).
    pub lat: Option<f64>,
    /// Longitude in decimal degrees. `None` if not available (sentinel 181°).
    pub lon: Option<f64>,
}

impl AidToNavigation {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 272 {
            return None;
        }
        let mmsi = extract_u32(bits, 8, 30)?;
        let aid_type = extract_u32(bits, 38, 5)? as u8;
        let name = extract_string(bits, 43, 20)?.trim().to_string();
        let lon_raw = extract_i32(bits, 164, 28)?;
        let lat_raw = extract_i32(bits, 192, 27)?;
        Some(Self {
            mmsi,
            aid_type,
            name,
            lat: decode_latitude(lat_raw),
            lon: decode_longitude(lon_raw),
        })
    }
}
