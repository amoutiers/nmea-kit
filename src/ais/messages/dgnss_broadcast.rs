//! AIS Type 17 — DGNSS broadcast binary message.

use crate::ais::armor::{extract_i32, extract_u32};

/// AIS Type 17 DGNSS correction broadcast.
#[derive(Debug, Clone, PartialEq)]
pub struct DgnssBroadcast {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    /// DGNSS reference-station longitude in decimal degrees (1/10 minute).
    pub longitude: Option<f64>,
    /// DGNSS reference-station latitude in decimal degrees (1/10 minute).
    pub latitude: Option<f64>,
    /// Opaque DGNSS correction data, one bit per byte.
    pub data: Vec<u8>,
}

impl DgnssBroadcast {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 80 {
            return None;
        }
        let longitude_raw = extract_i32(bits, 40, 18)?;
        let latitude_raw = extract_i32(bits, 58, 17)?;
        Some(Self {
            repeat_indicator: extract_u32(bits, 6, 2)? as u8,
            mmsi: extract_u32(bits, 8, 30)?,
            longitude: decode_longitude(longitude_raw),
            latitude: decode_latitude(latitude_raw),
            data: bits[80..].to_vec(),
        })
    }
}

fn decode_longitude(value: i32) -> Option<f64> {
    let value = f64::from(value) / 600.0;
    (-180.0..=180.0).contains(&value).then_some(value)
}

fn decode_latitude(value: i32) -> Option<f64> {
    let value = f64::from(value) / 600.0;
    (-90.0..=90.0).contains(&value).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn retains_dgnss_header_and_data() {
        let mut bits = vec![0; 83];
        set_bits(&mut bits, 0, 6, 17);
        set_bits(&mut bits, 6, 2, 2);
        set_bits(&mut bits, 8, 30, 123_456_789);
        set_bits(&mut bits, 40, 18, 600);
        set_bits(&mut bits, 58, 17, 1_200);
        bits[80..].copy_from_slice(&[1, 0, 1]);

        let message = DgnssBroadcast::decode(&bits).expect("decode");
        assert_eq!(message.repeat_indicator, 2);
        assert_eq!(message.mmsi, 123_456_789);
        assert_eq!(message.longitude, Some(1.0));
        assert_eq!(message.latitude, Some(2.0));
        assert_eq!(message.data, vec![1, 0, 1]);
    }
}
