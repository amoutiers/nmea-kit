use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TXT — Text Transmission.
///
/// Wire: `num_msg,msg_num,msg_type,text`
#[derive(Debug, Clone, PartialEq)]
pub struct Txt {
    /// Total number of messages in this sequence.
    pub num_msg: Option<u8>,
    /// Message number within the sequence (1-based).
    pub msg_num: Option<u8>,
    /// Message type identifier.
    pub msg_type: Option<u8>,
    /// Text message content.
    pub text: Option<String>,
}

impl Txt {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            num_msg: r.u8(),
            msg_num: r.u8(),
            msg_type: r.u8(),
            text: r.string(),
        })
    }
}

impl NmeaEncodable for Txt {
    const SENTENCE_TYPE: &str = "TXT";

    fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.u8(self.num_msg);
        w.u8(self.msg_num);
        w.u8(self.msg_type);
        w.string(self.text.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn txt_empty() {
        let f = Txt {
            num_msg: None,
            msg_num: None,
            msg_type: None,
            text: None,
        }
        .to_sentence("GP");
        let frame = parse_frame(f.trim()).expect("valid");
        let t = Txt::parse(&frame.fields).expect("parse");
        assert!(t.num_msg.is_none());
        assert!(t.msg_num.is_none());
        assert!(t.msg_type.is_none());
        assert!(t.text.is_none());
    }

    #[test]
    fn txt_encode_roundtrip() {
        let original = Txt {
            num_msg: Some(1),
            msg_num: Some(1),
            msg_type: Some(2),
            text: Some("u-blox ag - www.u-blox.com".to_string()),
        };
        let sentence = original.to_sentence("GP");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Txt::parse(&frame.fields).expect("re-parse TXT");
        assert_eq!(original, parsed);
    }

    #[test]
    fn txt_full_gonmea() {
        let frame =
            parse_frame("$GPTXT,01,01,02,u-blox ag - www.u-blox.com*50").expect("valid TXT frame");
        let txt = Txt::parse(&frame.fields).expect("parse TXT");
        assert_eq!(txt.num_msg, Some(1));
        assert_eq!(txt.msg_num, Some(1));
        assert_eq!(txt.msg_type, Some(2));
        assert_eq!(txt.text, Some("u-blox ag - www.u-blox.com".to_string()));
    }
}
