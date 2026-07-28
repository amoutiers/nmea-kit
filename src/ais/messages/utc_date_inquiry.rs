//! AIS Type 10 — UTC and date inquiry.

use crate::ais::armor::extract_u32;

/// AIS Type 10 UTC and date inquiry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtcDateInquiry {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub destination_mmsi: u32,
}

impl UtcDateInquiry {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 72 {
            return None;
        }
        Some(Self {
            repeat_indicator: extract_u32(bits, 6, 2)? as u8,
            mmsi: extract_u32(bits, 8, 30)?,
            destination_mmsi: extract_u32(bits, 40, 30)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn decodes_utc_date_inquiry() {
        let mut bits = vec![0; 72];
        set_bits(&mut bits, 0, 6, 10);
        set_bits(&mut bits, 6, 2, 3);
        set_bits(&mut bits, 8, 30, 123_456_789);
        set_bits(&mut bits, 40, 30, 987_654_321);

        assert_eq!(
            UtcDateInquiry::decode(&bits),
            Some(UtcDateInquiry {
                repeat_indicator: 3,
                mmsi: 123_456_789,
                destination_mmsi: 987_654_321,
            })
        );
    }
}
