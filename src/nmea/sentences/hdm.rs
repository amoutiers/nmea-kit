use crate::nmea::field::{FieldReader, FieldWriter};

/// HDM — Heading Magnetic.
///
/// Wire: `headingM,M`
#[derive(Debug, Clone, PartialEq)]
pub struct Hdm {
    /// Magnetic heading in degrees.
    pub heading_mag: Option<f32>,
}

impl Hdm {
    pub const SENTENCE_TYPE: &str = "HDM";

    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let heading_mag = r.f32();
        r.skip(); // M
        Some(Self { heading_mag })
    }

    pub fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.f32(self.heading_mag);
        w.fixed('M');
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
    fn hdm_empty() {
        let f = parse_frame("$IIHDM,,*41").expect("valid");
        let m = Hdm::parse(&f.fields).expect("parse");
        assert!(m.heading_mag.is_none());
    }

    #[test]
    fn hdm_full_signalk() {
        let frame = parse_frame("$04HDM,186.5,M*2C").expect("valid");
        let hdm = Hdm::parse(&frame.fields).expect("parse HDM");
        assert!((hdm.heading_mag.expect("hdg") - 186.5).abs() < 0.1);
    }

    #[test]
    fn hdm_gp_pynmeagps() {
        let frame = parse_frame("$GPHDM,223.12,M*05").expect("valid pynmeagps GP HDM");
        let hdm = Hdm::parse(&frame.fields).expect("parse HDM");
        assert!((hdm.heading_mag.expect("hdg") - 223.12).abs() < 0.01);
    }

    #[test]
    fn hdm_ii_pynmeagps() {
        let frame = parse_frame("$IIHDM,70.6,M*13").expect("valid pynmeagps II HDM");
        let hdm = Hdm::parse(&frame.fields).expect("parse HDM");
        assert!((hdm.heading_mag.expect("hdg") - 70.6).abs() < 0.1);
    }
    #[test]
    fn hdm_roundtrip() {
        let hdm = Hdm {
            heading_mag: Some(186.5),
        };
        let sentence = hdm.to_sentence("04");
        assert!(sentence.starts_with("$04HDM,"));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let hdm2 = Hdm::parse(&frame.fields).expect("re-parse HDM");
        assert_eq!(hdm.heading_mag, hdm2.heading_mag);
    }
}
