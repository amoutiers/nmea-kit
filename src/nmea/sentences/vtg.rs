use crate::nmea::field::{FieldReader, FieldWriter};

/// VTG — Track Made Good and Ground Speed.
#[derive(Debug, Clone, PartialEq)]
pub struct Vtg {
    /// Course over ground true in degrees.
    pub course_true: Option<f32>,
    /// Course over ground magnetic in degrees.
    pub course_mag: Option<f32>,
    /// Speed over ground in knots.
    pub speed_kts: Option<f32>,
    /// Speed over ground in km/h.
    pub speed_kmh: Option<f32>,
    /// Mode indicator ('A' = autonomous, 'D' = differential, etc.).
    pub mode: Option<char>,
}

impl Vtg {
    pub const SENTENCE_TYPE: &str = "VTG";

    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let course_true = r.f32();
        r.skip();
        let course_mag = r.f32();
        r.skip();
        let speed_kts = r.f32();
        r.skip();
        let speed_kmh = r.f32();
        r.skip();
        let mode = r.char();
        Some(Self { course_true, course_mag, speed_kts, speed_kmh, mode })
    }

    pub fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.f32(self.course_true);
        w.fixed('T');
        w.f32(self.course_mag);
        w.fixed('M');
        w.f32(self.speed_kts);
        w.fixed('N');
        w.f32(self.speed_kmh);
        w.fixed('K');
        w.char(self.mode);
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
    fn vtg_empty_signalk() {
        let f = parse_frame("$IIVTG,,,,,,,,,*69").expect("valid");
        let v = Vtg::parse(&f.fields).expect("parse");
        assert!(v.course_true.is_none());
        assert!(v.course_mag.is_none());
        assert!(v.speed_kts.is_none());
        assert!(v.speed_kmh.is_none());
        assert!(v.mode.is_none());
    }

    #[test]
    fn vtg_full_signalk() {
        let f = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid VTG frame");
        let v = Vtg::parse(&f.fields).expect("parse VTG");
        assert!((v.course_mag.expect("course_mag present") - 359.3).abs() < 0.01);
        assert_eq!(v.mode, Some('A'));
    }

    #[test]
    fn vtg_missing_course_signalk() {
        let f = parse_frame("$GPVTG,,T,,M,0.102,N,0.190,K,A*28").expect("valid VTG frame");
        let v = Vtg::parse(&f.fields).expect("parse VTG");
        assert!(v.course_true.is_none());
        assert!((v.speed_kts.expect("speed_kts present") - 0.102).abs() < 0.001);
    }

    #[test]
    fn vtg_roundtrip() {
        let v = Vtg { course_true: Some(0.0), course_mag: Some(359.3), speed_kts: Some(5.0), speed_kmh: Some(9.26), mode: Some('A') };
        let s = v.to_sentence("GP");
        let f = parse_frame(s.trim()).expect("re-parse VTG frame");
        let v2 = Vtg::parse(&f.fields).expect("re-parse VTG");
        assert_eq!(v.course_true, v2.course_true);
    }
}
