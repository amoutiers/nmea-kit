//! AIS Type 9 — Standard SAR Aircraft Position Report.

use super::utils::{decode_cog, decode_latitude, decode_longitude};
use crate::ais::armor::{extract_i32, extract_u32};

/// AIS Type 9 — Standard SAR Aircraft Position Report.
///
/// 168-bit fixed-length message from search and rescue aircraft.
/// Unlike vessel position reports, altitude is in meters and SOG is in whole knots (not 1/10).
#[derive(Debug, Clone, PartialEq)]
pub struct SarAircraftReport {
    pub mmsi: u32,
    /// Altitude in meters. None if not available (4095).
    pub altitude: Option<u16>,
    /// Speed over ground in knots (integer, NOT 1/10 knot). None if not available (1023).
    pub sog: Option<f32>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    /// Course over ground in 1/10 degree. None if not available (3600).
    pub cog: Option<f32>,
    /// UTC second (0-59). None if not available (60).
    pub timestamp: Option<u8>,
    /// DTE flag.
    pub dte: bool,
    /// Assigned mode flag.
    pub assigned: bool,
    /// RAIM flag.
    pub raim: bool,
}

impl SarAircraftReport {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 168 {
            return None;
        }
        let mmsi = extract_u32(bits, 8, 30)?;
        let alt_raw = extract_u32(bits, 38, 12)?;
        let sog_raw = extract_u32(bits, 50, 10)?;
        let accuracy = extract_u32(bits, 60, 1)? == 1;
        let lon_raw = extract_i32(bits, 61, 28)?;
        let lat_raw = extract_i32(bits, 89, 27)?;
        let cog_raw = extract_u32(bits, 116, 12)?;
        let ts_raw = extract_u32(bits, 128, 6)? as u8;
        let dte = extract_u32(bits, 134, 1)? == 1; // Note: DTE=0 means "DTE ready"
        let assigned = extract_u32(bits, 138, 1)? == 1;
        let raim = extract_u32(bits, 139, 1)? == 1;
        Some(Self {
            mmsi,
            altitude: if alt_raw == 4095 {
                None
            } else {
                Some(alt_raw as u16)
            },
            sog: if sog_raw == 1023 {
                None
            } else {
                Some(sog_raw as f32)
            },
            position_accuracy: accuracy,
            longitude: decode_longitude(lon_raw),
            latitude: decode_latitude(lat_raw),
            cog: decode_cog(cog_raw),
            timestamp: if ts_raw == 60 { None } else { Some(ts_raw) },
            dte,
            assigned,
            raim,
        })
    }
}
