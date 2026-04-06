use crate::FrameError;

/// A parsed NMEA 0183 frame with references into the original input.
///
/// The frame layer handles:
/// - `$` (NMEA) and `!` (AIS) prefix detection
/// - IEC 61162-450 tag block stripping
/// - XOR checksum validation
/// - Talker ID + sentence type extraction
/// - Field splitting by `,`
#[derive(Debug, Clone, PartialEq)]
pub struct NmeaFrame<'a> {
    /// Sentence prefix: `$` for NMEA, `!` for AIS.
    pub prefix: char,
    /// Talker identifier (typically 2 characters, e.g. "GP", "WI", "AI").
    pub talker: &'a str,
    /// Sentence type (3 characters, e.g. "RMC", "MWD", "VDM").
    pub sentence_type: &'a str,
    /// Comma-separated payload fields (after talker+type, before checksum).
    pub fields: Vec<&'a str>,
    /// IEC 61162-450 tag block content, if present.
    pub tag_block: Option<&'a str>,
}

/// Parse a raw NMEA 0183 line into a validated frame.
///
/// Handles both `$` (instrument) and `!` (AIS) sentences.
/// Strips optional IEC 61162-450 tag blocks (`\...\` prefix).
/// Validates XOR checksum when present.
///
/// # Examples
///
/// ```
/// use nmea_kit::parse_frame;
///
/// let frame = parse_frame("$WIMWD,270.0,T,268.5,M,12.4,N,6.4,M*63").unwrap();
/// assert_eq!(frame.prefix, '$');
/// assert_eq!(frame.talker, "WI");
/// assert_eq!(frame.sentence_type, "MWD");
/// assert_eq!(frame.fields.len(), 8);
/// ```
pub fn parse_frame(line: &str) -> Result<NmeaFrame<'_>, FrameError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(FrameError::Empty);
    }

    // Strip IEC 61162-450 tag block: \tag:val,...*xx\SENTENCE
    let (tag_block, line) = strip_tag_block(line)?;

    // Extract prefix
    let prefix = line.chars().next().ok_or(FrameError::Empty)?;
    if prefix != '$' && prefix != '!' {
        return Err(FrameError::InvalidPrefix(prefix));
    }

    let after_prefix = &line[1..];

    // Split at checksum delimiter
    let (body, checksum_str) = match after_prefix.rfind('*') {
        Some(pos) => {
            let body = &after_prefix[..pos];
            let cs_str = after_prefix[pos + 1..].trim_end_matches(['\r', '\n']);
            (body, Some(cs_str))
        }
        None => (after_prefix.trim_end_matches(['\r', '\n']), None),
    };

    // Validate checksum if present
    if let Some(cs_str) = checksum_str {
        let expected =
            u8::from_str_radix(cs_str, 16).map_err(|_| FrameError::MalformedChecksum)?;
        let computed = body.bytes().fold(0u8, |acc, b| acc ^ b);
        if expected != computed {
            return Err(FrameError::BadChecksum { expected, computed });
        }
    }

    // Extract talker (2 chars) + sentence type (3 chars)
    if body.len() < 5 {
        return Err(FrameError::TooShort);
    }

    // Find the first comma to determine where the address field ends
    let addr_end = body.find(',').unwrap_or(body.len());
    let addr = &body[..addr_end];

    // Talker = first 2 chars, sentence type = remaining (usually 3 chars)
    if addr.len() < 3 {
        return Err(FrameError::TooShort);
    }
    let talker = &addr[..addr.len() - 3];
    let sentence_type = &addr[addr.len() - 3..];

    // Split remaining fields by comma
    let fields_str = if addr_end < body.len() {
        &body[addr_end + 1..]
    } else {
        ""
    };

    let fields: Vec<&str> = if fields_str.is_empty() {
        Vec::new()
    } else {
        fields_str.split(',').collect()
    };

    Ok(NmeaFrame {
        prefix,
        talker,
        sentence_type,
        fields,
        tag_block,
    })
}

/// Encode fields into a valid NMEA 0183 sentence string.
///
/// Computes XOR checksum and appends `*XX\r\n`.
///
/// # Examples
///
/// ```
/// use nmea_kit::encode_frame;
///
/// let sentence = encode_frame('$', "WI", "MWD", &["270.0", "T", "268.5", "M", "12.4", "N", "6.4", "M"]);
/// assert!(sentence.starts_with("$WIMWD,"));
/// assert!(sentence.ends_with("\r\n"));
/// ```
pub fn encode_frame(prefix: char, talker: &str, sentence_type: &str, fields: &[&str]) -> String {
    let body = if fields.is_empty() {
        format!("{talker}{sentence_type}")
    } else {
        format!("{talker}{sentence_type},{}", fields.join(","))
    };

    let checksum = body.bytes().fold(0u8, |acc, b| acc ^ b);
    format!("{prefix}{body}*{checksum:02X}\r\n")
}

/// Strip an optional IEC 61162-450 tag block from the beginning of the line.
/// Returns `(Option<tag_block_content>, remaining_line)`.
fn strip_tag_block(line: &str) -> Result<(Option<&str>, &str), FrameError> {
    if let Some(rest) = line.strip_prefix('\\') {
        match rest.find('\\') {
            Some(close) => {
                let tag = &rest[..close];
                let remaining = &rest[close + 1..];
                Ok((Some(tag), remaining))
            }
            None => Err(FrameError::MalformedTagBlock),
        }
    } else {
        Ok((None, line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ais_multi_fragment_signalk() {
        let frame1 = parse_frame("!AIVDM,2,1,0,A,53brRt4000010SG700iE@LE8@Tp4000000000153P615t0Ht0SCkjH4jC1C,0*25")
            .expect("AIS fragment 1");
        assert_eq!(frame1.prefix, '!');
        assert_eq!(frame1.sentence_type, "VDM");
        assert_eq!(frame1.fields[1], "1"); // fragment number
    }

    #[test]
    fn apb_fixture_signalk() {
        let frame = parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C")
            .expect("valid APB");
        assert_eq!(frame.sentence_type, "APB");
        assert_eq!(frame.fields[9], "DEST");
    }

    #[test]
    fn dbt_sounder_gpsd() {
        let frame =
            parse_frame("$SDDBT,7.7,f,2.3,M,1.3,F*05").expect("valid DBT from GPSD sounder.log");
        assert_eq!(frame.sentence_type, "DBT");
        assert_eq!(frame.fields[2], "2.3"); // meters
    }

    #[test]
    fn dpt_fixtures_signalk() {
        let fixtures = [
            ("$IIDPT,4.1,0.0*45", "4.1", "0.0"),
            ("$IIDPT,4.1,1.0*44", "4.1", "1.0"),
            ("$IIDPT,4.1,-1.0*69", "4.1", "-1.0"),
        ];
        for (fix, depth, offset) in &fixtures {
            let frame = parse_frame(fix).unwrap_or_else(|e| panic!("failed to parse {fix}: {e}"));
            assert_eq!(frame.sentence_type, "DPT");
            assert_eq!(frame.fields[0], *depth);
            assert_eq!(frame.fields[1], *offset);
        }
    }

    #[test]
    fn dpt_humminbird_gpsd() {
        let frame = parse_frame("$INDPT,2.2,0.0*47").expect("valid DPT from GPSD humminbird");
        assert_eq!(frame.talker, "IN");
        assert_eq!(frame.sentence_type, "DPT");
    }

    #[test]
    fn encode_no_fields() {
        let result = encode_frame('$', "GP", "RMC", &[]);
        assert!(result.starts_with("$GPRMC*"));
    }

    #[test]
    fn encode_simple_sentence() {
        let result = encode_frame('$', "WI", "MWD", &["270.0", "T", "268.5", "M", "12.4", "N", "6.4", "M"]);
        assert!(result.starts_with("$WIMWD,270.0,T,268.5,M,12.4,N,6.4,M*"));
        assert!(result.ends_with("\r\n"));
        // Verify checksum is valid by re-parsing
        let frame = parse_frame(result.trim()).expect("encoded sentence should be parseable");
        assert_eq!(frame.sentence_type, "MWD");
    }

    #[test]
    fn encode_with_empty_fields() {
        let result = encode_frame('$', "GP", "APB", &["", "", "", "", "", "", "", "", "", "", "", "", "", ""]);
        let frame = parse_frame(result.trim()).expect("should re-parse");
        assert_eq!(frame.sentence_type, "APB");
        assert!(frame.fields.iter().all(|f| f.is_empty()));
    }

    #[test]
    fn error_bad_checksum() {
        assert!(matches!(
            parse_frame("$GPRMC,175957.917,A*FF"),
            Err(FrameError::BadChecksum { .. })
        ));
    }

    #[test]
    fn error_empty_input() {
        assert_eq!(parse_frame(""), Err(FrameError::Empty));
        assert_eq!(parse_frame("   "), Err(FrameError::Empty));
    }

    #[test]
    fn error_invalid_prefix() {
        assert!(matches!(
            parse_frame("GPRMC,175957.917,A*00"),
            Err(FrameError::InvalidPrefix('G'))
        ));
    }

    #[test]
    fn error_malformed_tag_block() {
        assert_eq!(
            parse_frame("\\s:FooBar$GPRMC,175957.917,A*00"),
            Err(FrameError::MalformedTagBlock)
        );
    }

    #[test]
    fn error_too_short() {
        assert_eq!(parse_frame("$GP*17"), Err(FrameError::TooShort));
    }

    #[test]
    fn hdg_fixtures_signalk() {
        let frame =
            parse_frame("$INHDG,180,5,W,10,W*6D").expect("valid HDG");
        assert_eq!(frame.sentence_type, "HDG");
        assert_eq!(frame.fields[0], "180");
        assert_eq!(frame.fields[1], "5");
        assert_eq!(frame.fields[2], "W");
    }

    #[test]
    fn hdt_saab_gpsd() {
        let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid HDT from GPSD saab-r4");
        assert_eq!(frame.talker, "HE");
        assert_eq!(frame.sentence_type, "HDT");
    }

    #[test]
    fn mtw_humminbird_gpsd() {
        let frame = parse_frame("$INMTW,17.9,C*1B").expect("valid MTW from GPSD humminbird");
        assert_eq!(frame.sentence_type, "MTW");
        assert_eq!(frame.fields[0], "17.9");
    }

    #[test]
    fn mwd_fixtures_signalk() {
        // From SignalK test suite
        let fixtures = [
            "$IIMWD,,,046.,M,10.1,N,05.2,M*0B",
            "$IIMWD,046.,T,046.,M,10.1,N,,*17",
            "$IIMWD,046.,T,,,,,5.2,M*72",
        ];
        for fix in &fixtures {
            let frame = parse_frame(fix).unwrap_or_else(|e| panic!("failed to parse {fix}: {e}"));
            assert_eq!(frame.sentence_type, "MWD");
        }
    }

    #[test]
    fn parse_ais_sentence() {
        let frame =
            parse_frame("!AIVDM,1,1,,A,13u@Dt002s000000000000000000,0*60").expect("valid frame");
        assert_eq!(frame.prefix, '!');
        assert_eq!(frame.talker, "AI");
        assert_eq!(frame.sentence_type, "VDM");
        assert_eq!(frame.fields[0], "1");
    }

    #[test]
    fn parse_depth_sentence() {
        let frame = parse_frame("$SDDBT,7.7,f,2.3,M,1.3,F*05").expect("valid frame");
        assert_eq!(frame.talker, "SD");
        assert_eq!(frame.sentence_type, "DBT");
        assert_eq!(frame.fields[2], "2.3");
    }

    #[test]
    fn parse_empty_fields() {
        let frame = parse_frame("$GPAPB,,,,,,,,,,,,,,*44").expect("valid frame");
        assert_eq!(frame.sentence_type, "APB");
        assert!(frame.fields.iter().all(|f| f.is_empty()));
    }

    #[test]
    fn parse_multi_constellation_talker() {
        // GN = multi-constellation GNSS
        let frame =
            parse_frame("$GNRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*69")
                .expect("valid frame");
        assert_eq!(frame.talker, "GN");
        assert_eq!(frame.sentence_type, "RMC");
    }

    #[test]
    fn parse_no_checksum_accepted() {
        let result = parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A");
        assert!(result.is_ok());
    }

    #[test]
    fn parse_standard_nmea_sentence() {
        let frame = parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77").expect("valid frame");
        assert_eq!(frame.prefix, '$');
        assert_eq!(frame.talker, "GP");
        assert_eq!(frame.sentence_type, "RMC");
        assert_eq!(frame.fields[0], "175957.917");
        assert_eq!(frame.fields[1], "A");
        assert_eq!(frame.tag_block, None);
    }

    #[test]
    fn parse_wind_sentence() {
        let frame = parse_frame("$WIMWD,270.0,T,268.5,M,12.4,N,6.4,M*63").expect("valid frame");
        assert_eq!(frame.talker, "WI");
        assert_eq!(frame.sentence_type, "MWD");
        assert_eq!(frame.fields.len(), 8);
        assert_eq!(frame.fields[0], "270.0");
        assert_eq!(frame.fields[1], "T");
    }

    #[test]
    fn parse_with_tag_block() {
        let frame = parse_frame("\\s:FooBar,c:1234567890*xx\\$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77").expect("valid frame");
        assert!(frame.tag_block.is_some());
        assert_eq!(frame.prefix, '$');
        assert_eq!(frame.sentence_type, "RMC");
    }

    #[test]
    fn rot_saab_gpsd() {
        let frame = parse_frame("$HEROT,0.0,A*2B").expect("valid ROT from GPSD saab-r4");
        assert_eq!(frame.sentence_type, "ROT");
    }

    #[test]
    fn roundtrip_parse_encode_parse() {
        let original = "$WIMWD,270.0,T,268.5,M,12.4,N,6.4,M*63";
        let frame1 = parse_frame(original).expect("parse original");
        let encoded = encode_frame(frame1.prefix, frame1.talker, frame1.sentence_type, &frame1.fields);
        let frame2 = parse_frame(encoded.trim()).expect("parse re-encoded");
        assert_eq!(frame1.talker, frame2.talker);
        assert_eq!(frame1.sentence_type, frame2.sentence_type);
        assert_eq!(frame1.fields, frame2.fields);
    }
}
