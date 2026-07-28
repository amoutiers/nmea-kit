use crate::EncodeError;
use crate::ais::encode::BitWriter;

use super::{AisEncodable, AisTransmitOptions, encode_payload};

/// AIS Type 12 addressed safety-related message.
#[derive(Debug, Clone, PartialEq)]
pub struct SafetyAddressed {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub sequence: u8,
    pub destination_mmsi: u32,
    pub retransmit: bool,
    pub text: String,
}

/// AIS Type 14 safety-related broadcast message.
#[derive(Debug, Clone, PartialEq)]
pub struct SafetyBroadcast {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub text: String,
}

impl AisEncodable for SafetyAddressed {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(12, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_u32(u32::from(self.sequence), 2, "sequence")?;
        writer.push_u32(self.destination_mmsi, 30, "destination_mmsi")?;
        writer.push_bool(self.retransmit);
        writer.push_spare(1);
        push_variable_text(&mut writer, &self.text, 156, "text")?;
        encode_payload(&writer.finish(), options)
    }
}

impl AisEncodable for SafetyBroadcast {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(14, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_spare(2);
        push_variable_text(&mut writer, &self.text, 161, "text")?;
        encode_payload(&writer.finish(), options)
    }
}

fn push_variable_text(
    writer: &mut BitWriter,
    value: &str,
    max_chars: usize,
    field: &'static str,
) -> Result<(), EncodeError> {
    let actual_chars = value.chars().count();
    if actual_chars > max_chars {
        return Err(EncodeError::AisTextTooLong {
            field,
            max_chars,
            actual_chars,
        });
    }
    writer.push_text(value, actual_chars, field)
}
