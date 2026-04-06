use crate::nmea::field::{FieldReader, FieldWriter};

/// DPT — Depth of Water.
#[derive(Debug, Clone, PartialEq)]
pub struct Dpt {
    /// Water depth below transducer in meters.
    pub depth: Option<f32>,
    /// Offset from transducer in meters (positive = keel, negative = surface).
    pub offset: Option<f32>,
    /// Maximum range scale in use in meters.
    pub rangescale: Option<f32>,
}

impl Dpt {
    pub const SENTENCE_TYPE: &str = "DPT";

    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            depth: r.f32(),
            offset: r.f32(),
            rangescale: r.f32(),
        })
    }

    pub fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.f32(self.depth);
        w.f32(self.offset);
        w.f32(self.rangescale);
        w.finish()
    }

    pub fn to_sentence(&self, talker: &str) -> String {
        let fields = self.encode();
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        crate::encode_frame('$', talker, Self::SENTENCE_TYPE, &field_refs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn dpt_empty_signalk() {
        let f = parse_frame("$IIDPT,,,*6C").expect("valid");
        let d = Dpt::parse(&f.fields).expect("parse");
        assert!(d.depth.is_none());
        assert!(d.offset.is_none());
        assert!(d.rangescale.is_none());
    }

    #[test]
    fn dpt_gpsd() {
        let f = parse_frame("$INDPT,2.2,0.0*47").expect("valid DPT frame");
        let d = Dpt::parse(&f.fields).expect("parse DPT");
        assert!((d.depth.expect("depth present") - 2.2).abs() < 0.01);
    }

    #[test]
    fn dpt_negative_offset_signalk() {
        let f = parse_frame("$IIDPT,4.1,-1.0*69").expect("valid DPT frame");
        let d = Dpt::parse(&f.fields).expect("parse DPT");
        assert!((d.offset.expect("offset present") - (-1.0)).abs() < 0.01);
    }

    #[test]
    fn dpt_parse_signalk() {
        let f = parse_frame("$IIDPT,4.1,0.0*45").expect("valid DPT frame");
        let d = Dpt::parse(&f.fields).expect("parse DPT");
        assert!((d.depth.expect("depth present") - 4.1).abs() < 0.01);
    }

    #[test]
    fn dpt_roundtrip() {
        let d = Dpt { depth: Some(4.1), offset: Some(1.0), rangescale: None };
        let s = d.to_sentence("II");
        let f = parse_frame(s.trim()).expect("re-parse DPT frame");
        let d2 = Dpt::parse(&f.fields).expect("re-parse DPT");
        assert_eq!(d.depth, d2.depth);
    }
}
