use crate::nmea::field::{FieldReader, FieldWriter};

/// MWV — Wind Speed and Angle.
///
/// Wire: `angle,reference(R/T),speed,speedUnit,status(A/V)`
#[derive(Debug, Clone, PartialEq)]
pub struct Mwv {
    /// Wind angle in degrees (0-360).
    pub wind_angle: Option<f32>,
    /// Reference type ('R' = relative, 'T' = true).
    pub reference: Option<char>,
    /// Wind speed (unit in `speed_units` field).
    pub wind_speed: Option<f32>,
    /// Speed unit indicator ('N' = knots, 'M' = m/s, 'K' = km/h).
    pub speed_units: Option<char>,
    /// Data validity ('A' = valid, 'V' = invalid).
    pub status: Option<char>,
}

impl Mwv {
    pub const SENTENCE_TYPE: &str = "MWV";

    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            wind_angle: r.f32(),
            reference: r.char(),
            wind_speed: r.f32(),
            speed_units: r.char(),
            status: r.char(),
        })
    }

    pub fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.f32(self.wind_angle);
        w.char(self.reference);
        w.f32(self.wind_speed);
        w.char(self.speed_units);
        w.char(self.status);
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
    fn mwv_empty_signalk() {
        let frame = parse_frame("$IIMWV,,,,*4C").expect("valid");
        let mwv = Mwv::parse(&frame.fields).expect("parse MWV");
        assert!(mwv.wind_angle.is_none());
        assert!(mwv.reference.is_none());
    }

    #[test]
    fn mwv_relative_signalk() {
        let frame = parse_frame("$IIMWV,336,R,13.41,N,A*22").expect("valid");
        let mwv = Mwv::parse(&frame.fields).expect("parse MWV");
        assert!((mwv.wind_angle.expect("angle") - 336.0).abs() < 0.1);
        assert_eq!(mwv.reference, Some('R'));
        assert!((mwv.wind_speed.expect("speed") - 13.41).abs() < 0.01);
        assert_eq!(mwv.speed_units, Some('N'));
        assert_eq!(mwv.status, Some('A'));
    }

    #[test]
    fn mwv_roundtrip() {
        let original = Mwv {
            wind_angle: Some(336.0),
            reference: Some('R'),
            wind_speed: Some(13.41),
            speed_units: Some('N'),
            status: Some('A'),
        };
        let sentence = original.to_sentence("II");
        assert!(sentence.starts_with("$IIMWV,"));

        let frame = parse_frame(sentence.trim()).expect("re-parse MWV sentence");
        let parsed = Mwv::parse(&frame.fields).expect("parse MWV from re-encoded frame");

        assert_eq!(original.wind_angle, parsed.wind_angle);
        assert_eq!(original.reference, parsed.reference);
        assert_eq!(original.wind_speed, parsed.wind_speed);
        assert_eq!(original.speed_units, parsed.speed_units);
        assert_eq!(original.status, parsed.status);
    }

    #[test]
    fn mwv_true_signalk() {
        let frame = parse_frame("$IIMWV,074,T,05.85,N,A*2E").expect("valid");
        let mwv = Mwv::parse(&frame.fields).expect("parse MWV");
        assert_eq!(mwv.reference, Some('T'));
    }
}
