//! AIS multi-fragment message reassembly.
//!
//! AIS messages can span multiple NMEA sentences (up to 5 fragments).
//! The reassembler collects fragments by message ID (0-9) and returns
//! the complete payload when the last fragment arrives.

/// Assembled AIS payload ready for decoding.
#[derive(Debug, Clone)]
pub struct AisPayload {
    /// Combined 6-bit armored payload from all fragments.
    pub payload: String,
    /// Fill bits from the last fragment (0-5).
    pub fill_bits: u8,
    /// Channel indicator ('A' or 'B').
    pub channel: char,
}

/// Slot for tracking a partially assembled multi-fragment message.
#[derive(Debug, Default)]
struct FragmentSlot {
    /// Total expected fragments.
    total: u8,
    /// Number of fragments received so far.
    received: u8,
    /// Accumulated payload.
    payload: String,
}

/// Multi-fragment reassembler.
///
/// Maintains 10 slots (message IDs 0-9) for concurrent multi-fragment
/// message assembly.
pub struct FragmentCollector {
    slots: [Option<FragmentSlot>; 10],
}

impl FragmentCollector {
    pub fn new() -> Self {
        Self {
            slots: Default::default(),
        }
    }

    /// Process one AIS NMEA frame's fields.
    ///
    /// Fields expected (from frame.fields after VDM/VDO sentence type):
    /// - `[0]` total_fragments
    /// - `[1]` fragment_number
    /// - `[2]` message_id (may be empty for single-fragment)
    /// - `[3]` channel ('A'/'B')
    /// - `[4]` payload (6-bit armored)
    /// - `[5]` fill_bits
    ///
    /// Returns `Some(AisPayload)` when a complete message is assembled.
    pub fn process(&mut self, fields: &[&str]) -> Option<AisPayload> {
        if fields.len() < 6 {
            return None;
        }

        let total: u8 = fields[0].parse().ok()?;
        let frag_num: u8 = fields[1].parse().ok()?;
        let msg_id_str = fields[2];
        let channel = fields[3].chars().next().unwrap_or('A');
        let payload = fields[4];
        let fill_bits: u8 = fields[5].parse().unwrap_or(0);

        if total == 0 || frag_num == 0 || frag_num > total {
            return None;
        }

        // Single-fragment message — return immediately
        if total == 1 {
            return Some(AisPayload {
                payload: payload.to_string(),
                fill_bits,
                channel,
            });
        }

        // Multi-fragment — need a message ID
        let msg_id: usize = msg_id_str.parse().ok()?;
        if msg_id > 9 {
            return None;
        }

        if frag_num == 1 {
            // Start new assembly
            self.slots[msg_id] = Some(FragmentSlot {
                total,
                received: 1,
                payload: payload.to_string(),
            });
            None
        } else {
            // Continue assembly
            let slot = self.slots[msg_id].as_mut()?;
            if slot.total != total || slot.received + 1 != frag_num {
                // Out of sequence — discard
                self.slots[msg_id] = None;
                return None;
            }

            slot.payload.push_str(payload);
            slot.received = frag_num;

            if frag_num == total {
                // Complete — take the slot
                let completed = self.slots[msg_id].take()?;
                Some(AisPayload {
                    payload: completed.payload,
                    fill_bits,
                    channel,
                })
            } else {
                None
            }
        }
    }
}

impl Default for FragmentCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_fragment() {
        let mut c = FragmentCollector::new();

        // Fragment 1 of 2
        let r1 = c.process(&[
            "2", "1", "0", "A",
            "53brRt4000010SG700iE@LE8@Tp4000000000153P615t0Ht0SCkjH4jC1C", "0",
        ]);
        assert!(r1.is_none());

        // Fragment 2 of 2
        let r2 = c.process(&["2", "2", "0", "A", "`0000000001", "2"]);
        assert!(r2.is_some());
        let p = r2.unwrap();
        assert!(p.payload.starts_with("53brRt"));
        assert!(p.payload.ends_with("`0000000001"));
        assert_eq!(p.fill_bits, 2);
    }

    #[test]
    fn out_of_sequence_discards() {
        let mut c = FragmentCollector::new();

        // Fragment 1 of 3
        let _ = c.process(&["3", "1", "1", "A", "AAAA", "0"]);
        // Fragment 3 of 3 (skipped 2) — should discard
        let r = c.process(&["3", "3", "1", "A", "CCCC", "0"]);
        assert!(r.is_none());
        // Slot should be cleared
        assert!(c.slots[1].is_none());
    }

    #[test]
    fn single_fragment() {
        let mut c = FragmentCollector::new();
        let result = c.process(&["1", "1", "", "A", "13u@Dt002s000000000000000000", "0"]);
        assert!(result.is_some());
        let p = result.unwrap();
        assert_eq!(p.payload, "13u@Dt002s000000000000000000");
        assert_eq!(p.fill_bits, 0);
        assert_eq!(p.channel, 'A');
    }

    #[test]
    fn too_few_fields() {
        let mut c = FragmentCollector::new();
        assert!(c.process(&["1", "1"]).is_none());
    }
}
