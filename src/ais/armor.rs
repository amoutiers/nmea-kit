//! AIS 6-bit ASCII armor encoding/decoding.
//!
//! AIS payloads are encoded as 6-bit ASCII: each character in the payload
//! represents 6 bits of data. Characters are in the range 0x30-0x77
//! (ASCII '0' to 'w'), with a gap at 0x58-0x5F (skipped).

/// Decode a 6-bit ASCII armored AIS payload into a bit vector.
///
/// Each character in the input maps to 6 bits. Returns a `Vec<u8>` where
/// each element is a single bit (0 or 1), MSB first within each 6-bit group.
///
/// The `fill_bits` parameter indicates how many of the rightmost bits in
/// the last 6-bit group should be ignored (0-5).
pub fn decode_armor(payload: &str, fill_bits: u8) -> Option<Vec<u8>> {
    let mut bits = Vec::with_capacity(payload.len() * 6);

    for ch in payload.bytes() {
        let mut val = ch - 48; // subtract ASCII '0'
        if val > 40 {
            val -= 8; // skip the gap 0x58-0x5F
        }
        if val > 63 {
            return None; // invalid character
        }

        // Extract 6 bits, MSB first
        for i in (0..6).rev() {
            bits.push((val >> i) & 1);
        }
    }

    // Remove fill bits from the end
    let fill = fill_bits as usize;
    if fill > 0 && bits.len() >= fill {
        bits.truncate(bits.len() - fill);
    }

    Some(bits)
}

/// Extract an unsigned integer from a bit slice at the given offset and length.
pub fn extract_u32(bits: &[u8], offset: usize, len: usize) -> Option<u32> {
    if offset + len > bits.len() || len > 32 {
        return None;
    }
    let mut val: u32 = 0;
    for i in 0..len {
        val = (val << 1) | u32::from(bits[offset + i]);
    }
    Some(val)
}

/// Extract a signed integer from a bit slice (two's complement).
pub fn extract_i32(bits: &[u8], offset: usize, len: usize) -> Option<i32> {
    let raw = extract_u32(bits, offset, len)?;
    // Check sign bit
    if len > 0 && (raw >> (len - 1)) & 1 == 1 {
        // Negative: sign-extend
        let mask = u32::MAX << len;
        Some((raw | mask) as i32)
    } else {
        Some(raw as i32)
    }
}

/// Extract a 6-bit ASCII string from a bit slice.
/// AIS text uses a custom 6-bit encoding: 0=@ (space), 1-26=A-Z, etc.
pub fn extract_string(bits: &[u8], offset: usize, num_chars: usize) -> Option<String> {
    let mut s = String::with_capacity(num_chars);
    for i in 0..num_chars {
        let char_offset = offset + i * 6;
        let val = extract_u32(bits, char_offset, 6)? as u8;
        let ch = match val {
            0 => '@',
            1..=26 => (b'A' + val - 1) as char,
            27..=31 => (b'[' + val - 27) as char, // [\]^_
            32 => ' ',
            33..=57 => (b'!' + val - 33) as char,  // !"#$%&'()*+,-./0123456789
            58..=63 => (b':' + val - 58) as char,  // :;<=>?
            _ => '?',
        };
        s.push(ch);
    }
    // Trim trailing @ and spaces
    let trimmed = s.trim_end_matches(['@', ' ']);
    Some(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_simple() {
        // '0' = ASCII 48, val = 0 → 000000
        // '1' = ASCII 49, val = 1 → 000001
        let bits = decode_armor("01", 0).unwrap();
        assert_eq!(bits.len(), 12);
        assert_eq!(&bits[..6], &[0, 0, 0, 0, 0, 0]);
        assert_eq!(&bits[6..12], &[0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn decode_with_fill_bits() {
        let bits = decode_armor("0", 2).unwrap();
        assert_eq!(bits.len(), 4); // 6 - 2 = 4
    }

    #[test]
    fn extract_signed_negative() {
        // All ones = -1 in two's complement
        let bits = vec![1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(extract_i32(&bits, 0, 8), Some(-1));
    }

    #[test]
    fn extract_signed_positive() {
        // All zeros except last bit = 1 → value = 1
        let bits = vec![0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(extract_i32(&bits, 0, 8), Some(1));
    }

    #[test]
    fn extract_text() {
        // "@@@@" in AIS 6-bit = all zeros = "@" trimmed to empty
        let bits = vec![0; 24];
        assert_eq!(extract_string(&bits, 0, 4), Some(String::new()));
    }

    #[test]
    fn extract_unsigned() {
        let bits = decode_armor("15RTgt0PAso;90TKcjM8h6g208CQ", 0).unwrap();
        // Message type is first 6 bits
        let msg_type = extract_u32(&bits, 0, 6).unwrap();
        assert_eq!(msg_type, 1); // Type 1 position report

        // MMSI is bits 8-37 (30 bits)
        let mmsi = extract_u32(&bits, 8, 30).unwrap();
        assert!(mmsi > 0);
    }

    #[test]
    fn extract_unsigned_out_of_bounds() {
        let bits = vec![0, 1, 0];
        assert_eq!(extract_u32(&bits, 0, 4), None); // only 3 bits available
    }

    #[test]
    fn decode_empty_payload() {
        let bits = decode_armor("", 0).unwrap();
        assert!(bits.is_empty());
    }

    #[test]
    fn decode_invalid_character() {
        // '~' (0x7E) is outside the valid AIS character range (0x30-0x77)
        assert!(decode_armor("~", 0).is_none());
    }

    #[test]
    fn decode_single_character() {
        let bits = decode_armor("0", 0).unwrap();
        assert_eq!(bits.len(), 6);
        assert_eq!(&bits, &[0, 0, 0, 0, 0, 0]);
    }
}
