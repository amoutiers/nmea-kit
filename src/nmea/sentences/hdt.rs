use crate::nmea::field::{FieldReader, FieldWriter};

/// HDT — Heading True.
///
/// Wire: `headingT,T`
#[derive(Debug, Clone, PartialEq)]
pub struct Hdt {
    /// True heading in degrees.
    pub heading_true: Option<f32>,
}

impl Hdt {
    pub const SENTENCE_TYPE: &str = "HDT";

    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let heading_true = r.f32();
        r.skip(); // T
        Some(Self { heading_true })
    }

    pub fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.f32(self.heading_true);
        w.fixed('T');
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
    fn hdt_empty() {
        let f = parse_frame("$IIHDT,,*58").expect("valid");
        let t = Hdt::parse(&f.fields).expect("parse");
        assert!(t.heading_true.is_none());
    }

    #[test]
    fn hdt_encode_roundtrip() {
        let hdt = Hdt {
            heading_true: Some(123.456),
        };
        let sentence = hdt.to_sentence("GP");
        assert!(sentence.starts_with("$GPHDT,"));
        assert!(sentence.contains('*'));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let hdt2 = Hdt::parse(&frame.fields).expect("re-parse HDT");
        assert_eq!(hdt.heading_true, hdt2.heading_true);
    }

    #[test]
    fn hdt_full_signalk() {
        let frame = parse_frame("$GPHDT,123.456,T*32").expect("valid");
        let hdt = Hdt::parse(&frame.fields).expect("parse HDT");
        assert!((hdt.heading_true.expect("hdg") - 123.456).abs() < 0.01);
    }

    #[test]
    fn hdt_he_talker_gpsd() {
        let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid");
        let hdt = Hdt::parse(&frame.fields).expect("parse HDT");
        assert!((hdt.heading_true.expect("hdg") - 4.0).abs() < 0.01);
    }

    #[test]
    fn hdt_pynmeagps() {
        let frame = parse_frame("$GPHDT,274.07,T*03").expect("valid pynmeagps HDT");
        let hdt = Hdt::parse(&frame.fields).expect("parse HDT");
        assert!((hdt.heading_true.expect("hdg") - 274.07).abs() < 0.01);
    }
}
