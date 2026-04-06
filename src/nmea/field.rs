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

    /// Write an optional u32. `None` → empty field.
    pub fn u32(&mut self, value: Option<u32>) {
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
}
