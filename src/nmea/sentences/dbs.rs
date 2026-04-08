use crate::nmea::field::{FieldReader, FieldWriter};

/// DBS — Depth Below Surface.
///
/// Wire: `depth_f,f,depth_m,M,depth_F,F`
#[derive(Debug, Clone, PartialEq)]
pub struct Dbs {
    /// Depth below surface in feet.
    pub depth_feet: Option<f32>,
    /// Depth below surface in meters.
    pub depth_meters: Option<f32>,
    /// Depth below surface in fathoms.
    pub depth_fathoms: Option<f32>,
}

impl Dbs {
    pub const SENTENCE_TYPE: &str = "DBS";

    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let depth_feet = r.f32();
        r.skip();
        let depth_meters = r.f32();
        r.skip();
        let depth_fathoms = r.f32();
        Some(Self {
            depth_feet,
            depth_meters,
            depth_fathoms,
        })
    }

    pub fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.f32(self.depth_feet);
        w.fixed('f');
        w.f32(self.depth_meters);
        w.fixed('M');
        w.f32(self.depth_fathoms);
        w.fixed('F');
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
    fn dbs_empty() {
        let f = parse_frame("$IIDBS,,,,,,*55").expect("empty DBS frame");
        let d = Dbs::parse(&f.fields).expect("parse DBS");
        assert!(d.depth_feet.is_none());
        assert!(d.depth_meters.is_none());
        assert!(d.depth_fathoms.is_none());
    }

    #[test]
    fn dbs_roundtrip() {
        let original = Dbs {
            depth_feet: Some(35.53),
            depth_meters: Some(10.83),
            depth_fathoms: Some(5.85),
        };
        let sentence = original.to_sentence("II");
        assert!(sentence.starts_with("$IIDBS,"));

        let frame = parse_frame(sentence.trim()).expect("re-parse DBS sentence");
        let parsed = Dbs::parse(&frame.fields).expect("parse DBS from re-encoded frame");

        assert_eq!(original.depth_feet, parsed.depth_feet);
        assert_eq!(original.depth_meters, parsed.depth_meters);
        assert_eq!(original.depth_fathoms, parsed.depth_fathoms);
    }

    #[test]
    fn dbs_signalk() {
        let f = parse_frame("$IIDBS,035.53,f,010.83,M,005.85,F*24").expect("valid DBS frame");
        let d = Dbs::parse(&f.fields).expect("parse DBS");
        assert!((d.depth_meters.expect("depth_meters present") - 10.83).abs() < 0.01);
    }
}
