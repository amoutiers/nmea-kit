//! AIS Type 26 — Multiple slot binary message with communication state.

use crate::ais::armor::extract_u32;

/// AIS Type 26 multiple-slot binary message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryMultiSlot {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub destination_mmsi: Option<u32>,
    /// `true` when an application identifier prefixes the binary data.
    pub binary_data_flag: bool,
    /// Raw application data, one bit per byte.
    pub data: Vec<u8>,
    pub communication_state_selector: bool,
    pub communication_state: u32,
}

impl BinaryMultiSlot {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 64 {
            return None;
        }
        let addressed = extract_u32(bits, 38, 1)? == 1;
        let data_start = if addressed { 72 } else { 40 };
        let data_end = bits.len().checked_sub(24)?;
        let communication_state_start = bits.len().checked_sub(20)?;
        if data_end < data_start {
            return None;
        }
        Some(Self {
            repeat_indicator: extract_u32(bits, 6, 2)? as u8,
            mmsi: extract_u32(bits, 8, 30)?,
            destination_mmsi: if addressed {
                Some(extract_u32(bits, 40, 30)?)
            } else {
                None
            },
            binary_data_flag: extract_u32(bits, 39, 1)? == 1,
            data: bits[data_start..data_end].to_vec(),
            communication_state_selector: extract_u32(bits, communication_state_start, 1)? == 1,
            communication_state: extract_u32(bits, communication_state_start + 1, 19)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn preserves_data_before_communication_state() {
        let mut bits = vec![0; 72];
        set_bits(&mut bits, 0, 6, 26);
        set_bits(&mut bits, 38, 1, 0);
        bits[40..48].copy_from_slice(&[1, 0, 1, 1, 0, 0, 1, 0]);
        set_bits(&mut bits, 52, 1, 1);
        set_bits(&mut bits, 53, 19, 0x5_4321);

        let message = BinaryMultiSlot::decode(&bits).expect("decode");
        assert_eq!(message.data, vec![1, 0, 1, 1, 0, 0, 1, 0]);
        assert!(message.communication_state_selector);
        assert_eq!(message.communication_state, 0x5_4321);
    }
}
