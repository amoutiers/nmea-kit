//! Field parsing and formatting helpers for NMEA sentence fields.
//!
//! [`FieldReader`] reads fields sequentially from a parsed frame.
//! [`FieldWriter`] builds fields sequentially for encoding.

/// Sequential field reader for NMEA sentence parsing.
///
/// Wraps a slice of `&str` fields and reads them in order,
/// advancing an internal index after each read.
pub struct FieldReader<'a> {
    fields: &'a [&'a str],
    idx: usize,
}

impl<'a> FieldReader<'a> {
    pub fn new(fields: &'a [&'a str]) -> Self {
        Self { fields, idx: 0 }
    }

    /// Read an optional f32 and advance.
    pub fn f32(&mut self) -> Option<f32> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<f32>().ok()
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional f64 and advance.
    pub fn f64(&mut self) -> Option<f64> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<f64>().ok()
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional u8 and advance.
    pub fn u8(&mut self) -> Option<u8> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<u8>().ok()
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional u32 and advance.
    pub fn u32(&mut self) -> Option<u32> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<u32>().ok()
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional i8 and advance.
    pub fn i8(&mut self) -> Option<i8> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<i8>().ok()
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional u16 and advance.
    pub fn u16(&mut self) -> Option<u16> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() { None } else { f.parse::<u16>().ok() }
        });
        self.idx += 1;
        val
    }

    /// Read an optional i16 and advance.
    pub fn i16(&mut self) -> Option<i16> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() { None } else { f.parse::<i16>().ok() }
        });
        self.idx += 1;
        val
    }

    /// Read an optional i32 and advance.
    pub fn i32(&mut self) -> Option<i32> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() { None } else { f.parse::<i32>().ok() }
        });
        self.idx += 1;
        val
    }

    /// Read an optional single character and advance.
    pub fn char(&mut self) -> Option<char> {
        let val = self
            .fields
            .get(self.idx)
            .and_then(|f| f.chars().next().filter(|_| !f.is_empty()));
        self.idx += 1;
        val
    }

    /// Read an optional non-empty string and advance.
    pub fn string(&mut self) -> Option<String> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                Some((*f).to_string())
            }
        });
        self.idx += 1;
        val
    }

    /// Skip one field (fixed indicator) and advance.
    pub fn skip(&mut self) {
        self.idx += 1;
    }
}

/// Sequential field writer for NMEA sentence encoding.
///
/// Builds a `Vec<String>` of field values in wire order.
pub struct FieldWriter {
    fields: Vec<String>,
}

impl FieldWriter {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Write an optional f32. `None` → empty field.
    pub fn f32(&mut self, value: Option<f32>) {
        self.fields.push(match value {
            Some(v) => format!("{v}"),
            None => String::new(),
        });
    }

    /// Write an optional f64. `None` → empty field.
    pub fn f64(&mut self, value: Option<f64>) {
        self.fields.push(match value {
            Some(v) => format!("{v}"),
            None => String::new(),
        });
    }

    /// Write an optional u8. `None` → empty field.
    pub fn u8(&mut self, value: Option<u8>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional i8. `None` → empty field.
    pub fn i8(&mut self, value: Option<i8>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional u32. `None` → empty field.
    pub fn u32(&mut self, value: Option<u32>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional u16. `None` → empty field.
    pub fn u16(&mut self, value: Option<u16>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional i16. `None` → empty field.
    pub fn i16(&mut self, value: Option<i16>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional i32. `None` → empty field.
    pub fn i32(&mut self, value: Option<i32>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional char. `None` → empty field.
    pub fn char(&mut self, value: Option<char>) {
        self.fields.push(match value {
            Some(c) => c.to_string(),
            None => String::new(),
        });
    }

    /// Write a fixed indicator character (always emitted).
    pub fn fixed(&mut self, c: char) {
        self.fields.push(c.to_string());
    }

    /// Write an optional string. `None` → empty field.
    pub fn string(&mut self, value: Option<&str>) {
        self.fields.push(value.unwrap_or("").to_string());
    }

    /// Consume and return the built field list.
    pub fn finish(self) -> Vec<String> {
        self.fields
    }
}

impl Default for FieldWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for NMEA sentence types that can be encoded to wire format.
///
/// Provides `SENTENCE_TYPE` and `encode()` — the `to_sentence()` default
/// method combines them with [`encode_frame()`](crate::encode_frame) to
/// produce a complete NMEA sentence with checksum.
///
/// # Standard sentences
///
/// ```
/// use nmea_kit::nmea::{NmeaEncodable, sentences::Dpt};
///
/// let dpt = Dpt { depth: Some(4.1), offset: Some(0.0), rangescale: None };
/// let sentence = dpt.to_sentence("II");
/// assert!(sentence.starts_with("$IIDPT,"));
/// ```
///
/// # Proprietary sentences
///
/// Proprietary types set [`PROPRIETARY_ID`](Self::PROPRIETARY_ID) to the full
/// address (e.g. `"PASHR"`, `"PSKPDPT"`) and use
/// [`to_proprietary_sentence()`](Self::to_proprietary_sentence) instead of
/// `to_sentence()`.
pub trait NmeaEncodable {
    /// The 3-character sentence type identifier (e.g. `"MWD"`, `"RMC"`).
    const SENTENCE_TYPE: &'static str;

    /// Full proprietary address identifier (e.g. `"PASHR"`, `"PSKPDPT"`).
    /// Empty for standard sentences.
    const PROPRIETARY_ID: &'static str = "";

    /// Encode fields into a `Vec` of strings in wire order.
    fn encode(&self) -> Vec<String>;

    /// Encode into a complete standard NMEA 0183 sentence with checksum and `\r\n`.
    fn to_sentence(&self, talker: &str) -> String {
        let fields = self.encode();
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        crate::encode_frame('$', talker, Self::SENTENCE_TYPE, &field_refs)
    }

    /// Encode into a complete proprietary NMEA 0183 sentence with checksum and `\r\n`.
    ///
    /// Uses [`PROPRIETARY_ID`](Self::PROPRIETARY_ID) as the full address
    /// (no separate talker).
    fn to_proprietary_sentence(&self) -> String {
        let fields = self.encode();
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        crate::encode_frame('$', "", Self::PROPRIETARY_ID, &field_refs)
    }
}

/// Convert an NMEA `DDMM.MMMM` coordinate to decimal degrees.
///
/// NMEA sentences encode latitude as `DDMM.MMMM` (degrees + minutes) and
/// longitude as `DDDMM.MMMM`. AIS and most application code use decimal degrees.
/// The sign (N/S, E/W) is not part of `ddmm` — apply it after conversion.
///
/// # Example
///
/// ```
/// use nmea_kit::nmea::ddmm_to_decimal;
///
/// // 4807.038 → 48°07.038′ → 48.1173°
/// let lat = ddmm_to_decimal(4807.038);
/// assert!((lat - 48.1173).abs() < 0.0001);
/// ```
pub fn ddmm_to_decimal(ddmm: f64) -> f64 {
    let degrees = (ddmm / 100.0).floor();
    let minutes = ddmm - degrees * 100.0;
    degrees + minutes / 60.0
}

/// Convert decimal degrees to an NMEA `DDMM.MMMM` coordinate.
///
/// This is the inverse of [`ddmm_to_decimal`]. The sign is not encoded —
/// strip it before calling and re-apply the N/S or E/W indicator separately.
///
/// # Example
///
/// ```
/// use nmea_kit::nmea::decimal_to_ddmm;
///
/// // 48.1173° → 48°07.038′ → 4807.038
/// let ddmm = decimal_to_ddmm(48.1173);
/// assert!((ddmm - 4807.038).abs() < 0.001);
/// ```
pub fn decimal_to_ddmm(decimal: f64) -> f64 {
    let degrees = decimal.floor();
    let minutes = (decimal - degrees) * 60.0;
    degrees * 100.0 + minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_char() {
        let fields = &["T", "", "AB"];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.char(), Some('T'));
        assert_eq!(r.char(), None);
        assert_eq!(r.char(), Some('A')); // takes first char
    }

    #[test]
    fn reader_f32() {
        let fields = &["270.0", "", "abc"];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.f32(), Some(270.0));
        assert_eq!(r.f32(), None);
        assert_eq!(r.f32(), None); // invalid
    }

    #[test]
    fn reader_past_end() {
        let fields: &[&str] = &[];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.f32(), None);
        assert_eq!(r.char(), None);
    }

    #[test]
    fn reader_skip() {
        let fields = &["10.0", "T", "20.0"];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.f32(), Some(10.0));
        r.skip();
        assert_eq!(r.f32(), Some(20.0));
    }

    #[test]
    fn reader_string() {
        let fields = &["DEST", ""];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.string(), Some("DEST".to_string()));
        assert_eq!(r.string(), None);
    }

    #[test]
    fn writer_roundtrip() {
        let mut w = FieldWriter::new();
        w.f32(Some(270.0));
        w.fixed('T');
        w.f32(None);
        w.fixed('M');
        let fields = w.finish();
        assert_eq!(fields, vec!["270", "T", "", "M"]);
    }

    #[test]
    fn ddmm_to_decimal_lat() {
        // 4807.038 → 48°07.038′ → 48.1173°
        let result = ddmm_to_decimal(4807.038);
        assert!((result - 48.1173).abs() < 0.0001);
    }

    #[test]
    fn ddmm_to_decimal_lon() {
        // 01131.000 → 11°31.000′ → 11.5167°
        let result = ddmm_to_decimal(1131.0);
        assert!((result - 11.5167).abs() < 0.0001);
    }

    #[test]
    fn decimal_to_ddmm_lat() {
        // 48.1173° → 4807.038
        let result = decimal_to_ddmm(48.1173);
        assert!((result - 4807.038).abs() < 0.001);
    }

    #[test]
    fn decimal_to_ddmm_roundtrip() {
        let original = 5132.5200_f64;
        let roundtrip = decimal_to_ddmm(ddmm_to_decimal(original));
        assert!((roundtrip - original).abs() < 0.0001);
    }
}
