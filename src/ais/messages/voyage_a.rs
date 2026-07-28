//! AIS static and voyage data — Type 5 (Class A).

use crate::ais::armor::{extract_string, extract_u32};

use super::common::AisClass;

/// AIS Static and Voyage Data — Type 5 (Class A).
#[derive(Debug, Clone, PartialEq)]
pub struct StaticVoyageData {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub ais_version: u8,
    pub imo: u32,
    pub callsign: String,
    pub vessel_name: String,
    pub ship_type: u8,
    pub dimension_to_bow: u16,
    pub dimension_to_stern: u16,
    pub dimension_to_port: u8,
    pub dimension_to_starboard: u8,
    pub position_fixing_device: u8,
    pub eta_month: Option<u8>,
    pub eta_day: Option<u8>,
    pub eta_hour: Option<u8>,
    pub eta_minute: Option<u8>,
    pub draught_meters: Option<f32>,
    pub destination: String,
    pub dte: bool,
    pub ais_class: AisClass,
}

impl StaticVoyageData {
    /// Decode a Type 5 static and voyage data message.
    pub fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 424 {
            return None;
        }

        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let ais_version = extract_u32(bits, 38, 2)? as u8;
        let imo = extract_u32(bits, 40, 30)?;
        let callsign = extract_string(bits, 70, 7)?;
        let vessel_name = extract_string(bits, 112, 20)?;
        let ship_type = extract_u32(bits, 232, 8)? as u8;
        let dimension_to_bow = extract_u32(bits, 240, 9)? as u16;
        let dimension_to_stern = extract_u32(bits, 249, 9)? as u16;
        let dimension_to_port = extract_u32(bits, 258, 6)? as u8;
        let dimension_to_starboard = extract_u32(bits, 264, 6)? as u8;
        let position_fixing_device = extract_u32(bits, 270, 4)? as u8;
        let eta_month = optional_range(extract_u32(bits, 274, 4)? as u8, 1, 12);
        let eta_day = optional_range(extract_u32(bits, 278, 5)? as u8, 1, 31);
        let eta_hour = optional_range(extract_u32(bits, 283, 5)? as u8, 0, 23);
        let eta_minute = optional_range(extract_u32(bits, 288, 6)? as u8, 0, 59);
        let draught_raw = extract_u32(bits, 294, 8)? as u8;
        let destination = extract_string(bits, 302, 20)?;
        let dte = extract_u32(bits, 422, 1)? == 1;

        Some(Self {
            repeat_indicator,
            mmsi,
            ais_version,
            imo,
            callsign,
            vessel_name,
            ship_type,
            dimension_to_bow,
            dimension_to_stern,
            dimension_to_port,
            dimension_to_starboard,
            position_fixing_device,
            eta_month,
            eta_day,
            eta_hour,
            eta_minute,
            draught_meters: if draught_raw == 0 {
                None
            } else {
                Some(f32::from(draught_raw) / 10.0)
            },
            destination,
            dte,
            ais_class: AisClass::A,
        })
    }
}

fn optional_range(value: u8, minimum: u8, maximum: u8) -> Option<u8> {
    if (minimum..=maximum).contains(&value) {
        Some(value)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn type5_retains_voyage_fields() {
        let mut bits = vec![0u8; 424];
        set_bits(&mut bits, 0, 6, 5);
        set_bits(&mut bits, 6, 2, 2);
        set_bits(&mut bits, 38, 2, 3);
        set_bits(&mut bits, 240, 9, 100);
        set_bits(&mut bits, 249, 9, 30);
        set_bits(&mut bits, 258, 6, 10);
        set_bits(&mut bits, 264, 6, 11);
        set_bits(&mut bits, 270, 4, 1);
        set_bits(&mut bits, 274, 4, 7);
        set_bits(&mut bits, 278, 5, 28);
        set_bits(&mut bits, 283, 5, 12);
        set_bits(&mut bits, 288, 6, 30);
        set_bits(&mut bits, 294, 8, 65);
        set_bits(&mut bits, 302, 6, 12); // L
        set_bits(&mut bits, 308, 6, 5); // E
        set_bits(&mut bits, 314, 6, 8); // H
        set_bits(&mut bits, 422, 1, 1);

        let data = StaticVoyageData::decode(&bits).expect("decodes");
        assert_eq!(data.repeat_indicator, 2);
        assert_eq!(data.ais_version, 3);
        assert_eq!(data.dimension_to_bow, 100);
        assert_eq!(data.dimension_to_stern, 30);
        assert_eq!(data.dimension_to_port, 10);
        assert_eq!(data.dimension_to_starboard, 11);
        assert_eq!(data.position_fixing_device, 1);
        assert_eq!(data.eta_month, Some(7));
        assert_eq!(data.eta_day, Some(28));
        assert_eq!(data.eta_hour, Some(12));
        assert_eq!(data.eta_minute, Some(30));
        assert_eq!(data.draught_meters, Some(6.5));
        assert_eq!(data.destination, "LEH");
        assert!(data.dte);
    }
}
