//! AIS Type 16 — Assigned mode command.

use crate::ais::armor::extract_u32;

/// A Type 16 assignment for one destination station.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub destination_mmsi: u32,
    pub offset: u16,
    pub increment: u16,
}

/// AIS Type 16 assigned mode command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentModeCommand {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    /// One or two assigned stations.
    pub assignments: Vec<Assignment>,
}

impl AssignmentModeCommand {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 96 {
            return None;
        }
        let mut assignments = vec![decode_assignment(bits, 40)?];
        if bits.len() >= 144 {
            assignments.push(decode_assignment(bits, 92)?);
        }
        Some(Self {
            repeat_indicator: extract_u32(bits, 6, 2)? as u8,
            mmsi: extract_u32(bits, 8, 30)?,
            assignments,
        })
    }
}

fn decode_assignment(bits: &[u8], offset: usize) -> Option<Assignment> {
    Some(Assignment {
        destination_mmsi: extract_u32(bits, offset, 30)?,
        offset: extract_u32(bits, offset + 30, 12)? as u16,
        increment: extract_u32(bits, offset + 42, 10)? as u16,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn decodes_two_assignments() {
        let mut bits = vec![0; 144];
        set_bits(&mut bits, 0, 6, 16);
        set_bits(&mut bits, 6, 2, 1);
        set_bits(&mut bits, 8, 30, 123_456_789);
        set_bits(&mut bits, 40, 30, 111_111_111);
        set_bits(&mut bits, 70, 12, 100);
        set_bits(&mut bits, 82, 10, 25);
        set_bits(&mut bits, 92, 30, 222_222_222);
        set_bits(&mut bits, 122, 12, 200);
        set_bits(&mut bits, 134, 10, 50);

        let message = AssignmentModeCommand::decode(&bits).expect("decode");
        assert_eq!(message.repeat_indicator, 1);
        assert_eq!(message.mmsi, 123_456_789);
        assert_eq!(
            message.assignments,
            vec![
                Assignment {
                    destination_mmsi: 111_111_111,
                    offset: 100,
                    increment: 25,
                },
                Assignment {
                    destination_mmsi: 222_222_222,
                    offset: 200,
                    increment: 50,
                },
            ]
        );
    }
}
