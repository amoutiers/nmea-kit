/// Errors from frame-level parsing (checksum, delimiters, tag blocks).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameError {
    /// Input is empty or whitespace-only.
    Empty,
    /// First character is not `$` or `!`.
    InvalidPrefix(char),
    /// Checksum field is not valid hexadecimal.
    MalformedChecksum,
    /// Checksum mismatch.
    BadChecksum {
        expected: u8,
        computed: u8,
    },
    /// Tag block opened with `\` but not properly closed.
    MalformedTagBlock,
    /// Sentence too short to contain talker + type (minimum 5 chars: e.g. "GPRMC").
    TooShort,
}

impl core::fmt::Display for FrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::InvalidPrefix(c) => write!(f, "invalid prefix '{c}', expected '$' or '!'"),
            Self::MalformedChecksum => write!(f, "checksum is not valid hexadecimal"),
            Self::BadChecksum { expected, computed } => {
                write!(f, "checksum mismatch: expected {expected:02X}, computed {computed:02X}")
            }
            Self::MalformedTagBlock => write!(f, "malformed IEC 61162-450 tag block"),
            Self::TooShort => write!(f, "sentence too short"),
        }
    }
}

impl std::error::Error for FrameError {}
