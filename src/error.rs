/// Errors from frame-level parsing (checksum, delimiters, tag blocks).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameError {
    /// Input is empty or whitespace-only.
    Empty,
    /// First character is not `$` or `!`.
    InvalidPrefix(char),
    /// Checksum field is not valid hexadecimal.
    MalformedChecksum,
    /// Checksum mismatch.
    BadChecksum { expected: u8, computed: u8 },
    /// Tag block opened with `\` but not properly closed.
    MalformedTagBlock,
    /// Tag block checksum mismatch.
    BadTagChecksum { expected: u8, computed: u8 },
    /// Sentence too short to contain a valid address (minimum 3 chars, 4 for
    /// proprietary `P` addresses).
    TooShort,
    /// Address (talker + type) contains non-ASCII bytes (NMEA addresses are ASCII).
    NonAsciiAddress,
}

impl core::fmt::Display for FrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::InvalidPrefix(c) => write!(f, "invalid prefix '{c}', expected '$' or '!'"),
            Self::MalformedChecksum => write!(f, "checksum is not valid hexadecimal"),
            Self::BadChecksum { expected, computed } => {
                write!(
                    f,
                    "checksum mismatch: expected {expected:02X}, computed {computed:02X}"
                )
            }
            Self::MalformedTagBlock => write!(f, "malformed IEC 61162-450 tag block"),
            Self::BadTagChecksum { expected, computed } => {
                write!(
                    f,
                    "tag block checksum mismatch: expected {expected:02X}, computed {computed:02X}"
                )
            }
            Self::TooShort => write!(f, "sentence too short"),
            Self::NonAsciiAddress => write!(f, "address field contains non-ASCII bytes"),
        }
    }
}

impl std::error::Error for FrameError {}

/// Errors from encoding (invalid frame parts or field values).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    /// Prefix is not `$` or `!`.
    InvalidPrefix(char),
    /// Talker or sentence type contains non-ASCII characters.
    NonAsciiAddress,
    /// Sentence type is empty.
    InvalidAddress,
    /// A field contains `,`, `*`, `\r`, `\n`, or a non-ASCII character.
    InvalidFieldCharacter(char),
    /// Coordinate magnitude is NaN, infinite, or negative.
    InvalidCoordinate,
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidPrefix(c) => write!(f, "invalid prefix '{c}', expected '$' or '!'"),
            Self::NonAsciiAddress => write!(f, "talker or sentence type is not ASCII"),
            Self::InvalidAddress => write!(f, "sentence type is empty"),
            Self::InvalidFieldCharacter(c) => {
                write!(f, "field contains invalid character {c:?}")
            }
            Self::InvalidCoordinate => {
                write!(f, "coordinate magnitude is NaN, infinite, or negative")
            }
        }
    }
}

impl std::error::Error for EncodeError {}
