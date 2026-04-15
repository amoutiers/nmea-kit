use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// Per-satellite information block within a GSV sentence.
#[derive(Debug, Clone, PartialEq)]
pub struct SatInfo {
    /// Satellite PRN number (up to ~400 for multi-constellation).
    pub prn: Option<u32>,
    /// Elevation in degrees (0–90).
    pub elevation: Option<i8>,
    /// Azimuth in degrees (0–359).
    pub azimuth: Option<u32>,
    /// Signal-to-noise ratio in dB-Hz (0–99, `None` when not tracking).
    pub snr: Option<u8>,
}

/// GSV — Satellites in View.
///
/// Wire: `total_msgs,msg_num,sats_in_view,[prn,elevation,azimuth,snr]×n[,signal_id]`
#[derive(Debug, Clone, PartialEq)]
pub struct Gsv {
    /// Total number of GSV messages in this cycle.
    pub total_msgs: Option<u8>,
    /// Message number within the cycle (1-based).
    pub msg_num: Option<u8>,
    /// Total number of satellites in view.
    pub sats_in_view: Option<u8>,
    /// Satellite information blocks (up to 4 per message).
    pub sats: Vec<SatInfo>,
    /// Signal ID (NMEA 4.11, optional last field).
    pub signal_id: Option<u8>,
}

impl Gsv {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let total_msgs = r.u8();
        let msg_num = r.u8();
        let sats_in_view = r.u8();

        // Satellite groups are 4 fields each; remainder of 1 means signal_id is present.
        let remaining = fields.len().saturating_sub(3);
        let num_groups = remaining / 4;
        let has_signal_id = remaining % 4 == 1;

        let mut sats = Vec::with_capacity(num_groups);
        for _ in 0..num_groups {
            sats.push(SatInfo {
                prn: r.u32(),
                elevation: r.i8(),
                azimuth: r.u32(),
                snr: r.u8(),
            });
        }

        let signal_id = if has_signal_id { r.u8() } else { None };

        Some(Self {
            total_msgs,
            msg_num,
            sats_in_view,
            sats,
            signal_id,
        })
    }
}

impl NmeaEncodable for Gsv {
    const SENTENCE_TYPE: &str = "GSV";

    fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.u8(self.total_msgs);
        w.u8(self.msg_num);
        w.u8(self.sats_in_view);
        for sat in &self.sats {
            w.u32(sat.prn);
            w.i8(sat.elevation);
            w.u32(sat.azimuth);
            w.u8(sat.snr);
        }
        if self.signal_id.is_some() {
            w.u8(self.signal_id);
        }
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn gsv_empty() {
        let f = Gsv {
            total_msgs: None,
            msg_num: None,
            sats_in_view: None,
            sats: vec![],
            signal_id: None,
        }
        .to_sentence("GP");
        let frame = parse_frame(f.trim()).expect("valid");
        let g = Gsv::parse(&frame.fields).expect("parse");
        assert!(g.total_msgs.is_none());
        assert!(g.msg_num.is_none());
        assert!(g.sats_in_view.is_none());
        assert!(g.sats.is_empty());
    }

    #[test]
    fn gsv_encode_roundtrip() {
        let original = Gsv {
            total_msgs: Some(3),
            msg_num: Some(1),
            sats_in_view: Some(9),
            sats: vec![
                SatInfo {
                    prn: Some(9),
                    elevation: Some(73),
                    azimuth: Some(246),
                    snr: Some(35),
                },
                SatInfo {
                    prn: Some(2),
                    elevation: Some(51),
                    azimuth: Some(60),
                    snr: Some(40),
                },
                SatInfo {
                    prn: Some(6),
                    elevation: Some(16),
                    azimuth: Some(58),
                    snr: Some(37),
                },
                SatInfo {
                    prn: Some(7),
                    elevation: Some(16),
                    azimuth: Some(35),
                    snr: Some(25),
                },
            ],
            signal_id: None,
        };
        let sentence = original.to_sentence("GP");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Gsv::parse(&frame.fields).expect("re-parse GSV");
        assert_eq!(original, parsed);
    }

    #[test]
    fn gsv_glonass_gonmea() {
        let frame =
            parse_frame("$GLGSV,3,1,11,03,03,111,00,04,15,270,00,06,01,010,12,13,06,292,00*6B")
                .expect("valid go-nmea GLONASS GSV frame");
        let gsv = Gsv::parse(&frame.fields).expect("parse GSV");
        assert_eq!(gsv.total_msgs, Some(3));
        assert_eq!(gsv.msg_num, Some(1));
        assert_eq!(gsv.sats_in_view, Some(11));
        assert_eq!(gsv.sats.len(), 4);
        assert_eq!(gsv.sats[0].prn, Some(3));
        assert_eq!(gsv.sats[0].elevation, Some(3));
        assert_eq!(gsv.sats[0].azimuth, Some(111));
        assert_eq!(gsv.sats[0].snr, Some(0));
        assert_eq!(gsv.sats[1].prn, Some(4));
        assert_eq!(gsv.sats[3].prn, Some(13));
        assert!(gsv.signal_id.is_none());
    }

    #[test]
    fn gsv_gpsd() {
        let frame =
            parse_frame("$GPGSV,3,1,09,09,73,246,35,02,51,060,40,06,16,058,37,07,16,291,25*78")
                .expect("valid gpsd GSV frame");
        let gsv = Gsv::parse(&frame.fields).expect("parse GSV");
        assert_eq!(gsv.total_msgs, Some(3));
        assert_eq!(gsv.msg_num, Some(1));
        assert_eq!(gsv.sats_in_view, Some(9));
        assert_eq!(gsv.sats.len(), 4);
        assert_eq!(gsv.sats[0].prn, Some(9));
        assert_eq!(gsv.sats[0].elevation, Some(73));
        assert_eq!(gsv.sats[0].azimuth, Some(246));
        assert_eq!(gsv.sats[0].snr, Some(35));
        assert_eq!(gsv.sats[1].prn, Some(2));
        assert_eq!(gsv.sats[2].prn, Some(6));
        assert_eq!(gsv.sats[3].prn, Some(7));
        assert!(gsv.signal_id.is_none());
    }
}
