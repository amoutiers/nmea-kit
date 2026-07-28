//! AIS application-layer NMEA sentence types.

use crate::NmeaFrame;

#[cfg(feature = "abm")]
mod abm;
#[cfg(feature = "bbm")]
mod bbm;
mod field;

#[cfg(feature = "abm")]
pub use abm::*;
#[cfg(feature = "bbm")]
pub use bbm::*;

/// Unified enum covering supported AIS application-layer NMEA sentence types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum AisSentence {
    #[cfg(feature = "abm")]
    Abm(Abm),
    #[cfg(feature = "bbm")]
    Bbm(Bbm),
    Unknown {
        sentence_type: String,
        fields: Vec<String>,
    },
}

impl AisSentence {
    /// Parse an AIS application-layer NMEA frame into a typed sentence variant.
    pub fn parse(frame: &NmeaFrame<'_>) -> Self {
        match (frame.prefix, frame.sentence_type) {
            #[cfg(feature = "abm")]
            ('!', Abm::SENTENCE_TYPE) => match Abm::parse(&frame.fields) {
                Some(v) => return Self::Abm(v),
                None => return Self::from_frame(frame),
            },
            #[cfg(feature = "bbm")]
            ('!', Bbm::SENTENCE_TYPE) => match Bbm::parse(&frame.fields) {
                Some(v) => return Self::Bbm(v),
                None => return Self::from_frame(frame),
            },
            _ => {}
        }

        Self::from_frame(frame)
    }

    fn from_frame(frame: &NmeaFrame<'_>) -> Self {
        Self::Unknown {
            sentence_type: frame.sentence_type.to_string(),
            fields: frame.fields.iter().map(|f| f.to_string()).collect(),
        }
    }
}
