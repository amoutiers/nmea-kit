use super::field::{encode_f32, encode_u8, read_f32, read_string, read_u8};

/// VSD — AIS voyage static data.
///
/// Source: go-nmea fixtures.
///
/// Wire: `type_of_ship,draught,persons,destination,arrival_time,arrival_day,arrival_month,nav_status,regional`
#[derive(Debug, Clone, PartialEq)]
pub struct Vsd {
    /// Type of ship and cargo (AIS type code).
    pub type_of_ship: Option<u8>,
    /// Maximum present static draught in metres.
    pub draught: Option<f32>,
    /// Number of persons on board.
    pub persons: Option<u8>,
    /// Destination (up to 20 characters).
    pub destination: Option<String>,
    /// Estimated time of arrival (UTC, HHMM or HHMMSS depending on sender).
    pub arrival_time: Option<String>,
    /// Estimated arrival day (UTC).
    pub arrival_day: Option<u8>,
    /// Estimated arrival month (UTC).
    pub arrival_month: Option<u8>,
    /// Navigational status code.
    pub nav_status: Option<u8>,
    /// Regional reserved application identifier.
    pub regional: Option<u8>,
}

impl Vsd {
    pub const SENTENCE_TYPE: &'static str = "VSD";

    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut idx = 0;
        let type_of_ship = read_u8(fields, &mut idx);
        let draught = read_f32(fields, &mut idx);
        let persons = read_u8(fields, &mut idx);
        let destination = read_string(fields, &mut idx);
        let arrival_time = read_string(fields, &mut idx);
        let arrival_day = read_u8(fields, &mut idx);
        let arrival_month = read_u8(fields, &mut idx);
        let nav_status = read_u8(fields, &mut idx);
        let regional = read_u8(fields, &mut idx);
        Some(Self {
            type_of_ship,
            draught,
            persons,
            destination,
            arrival_time,
            arrival_day,
            arrival_month,
            nav_status,
            regional,
        })
    }

    pub fn encode(&self) -> Vec<String> {
        vec![
            encode_u8(self.type_of_ship),
            encode_f32(self.draught),
            encode_u8(self.persons),
            self.destination.clone().unwrap_or_default(),
            self.arrival_time.clone().unwrap_or_default(),
            encode_u8(self.arrival_day),
            encode_u8(self.arrival_month),
            encode_u8(self.nav_status),
            encode_u8(self.regional),
        ]
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
    fn vsd_empty() {
        let s = Vsd {
            type_of_ship: None,
            draught: None,
            persons: None,
            destination: None,
            arrival_time: None,
            arrival_day: None,
            arrival_month: None,
            nav_status: None,
            regional: None,
        }
        .to_sentence("RA");
        let f = parse_frame(s.trim()).expect("valid");
        let v = Vsd::parse(&f.fields).expect("parse");
        assert!(v.type_of_ship.is_none());
        assert!(v.destination.is_none());
    }

    #[test]
    fn vsd_encode_roundtrip() {
        let original = Vsd {
            type_of_ship: Some(0),
            draught: Some(4.5),
            persons: Some(6),
            destination: Some("PORT".to_string()),
            arrival_time: Some("220516".to_string()),
            arrival_day: Some(1),
            arrival_month: Some(2),
            nav_status: Some(8),
            regional: None,
        };
        let sentence = original.to_sentence("RA");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Vsd::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn vsd_ravsd_gonmea() {
        let frame =
            parse_frame("$RAVSD,0,4.5,6,@@@@@@@@@@@@@@@@@@@@,220516,01,02,8,*6E").expect("valid");
        let v = Vsd::parse(&frame.fields).expect("parse");
        assert_eq!(v.type_of_ship, Some(0));
        assert!((v.draught.expect("draught") - 4.5).abs() < 0.01);
        assert_eq!(v.persons, Some(6));
        assert_eq!(v.destination.as_deref(), Some("@@@@@@@@@@@@@@@@@@@@"));
        assert_eq!(v.arrival_time.as_deref(), Some("220516"));
        assert_eq!(v.arrival_day, Some(1));
        assert_eq!(v.arrival_month, Some(2));
        assert_eq!(v.nav_status, Some(8));
        assert!(v.regional.is_none());
    }
}
