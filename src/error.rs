use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Custom(String),
    InvalidMessage,
    InvalidCommand,
    InvalidOpCommand,
    InvalidRndType,
    InvalidRndLength,
    InvalidHashingAlgo,
    InvalidCodecAlgo,
    InvalidChecksumAlgo,
    UnimplementedFeature,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO Error: {}", err),
            Error::Custom(message) => write!(f, "Error: {}", message),
            Error::InvalidMessage => write!(
                f,
                "Invalid Message size. Expecing COMMAND MODE ATTRIBUTE/DATA e.g RND ALPHA 20"
            ),
            Error::InvalidCommand => {
                write!(f, "Invalid Command. Expecting RND, HASH, ENCODE, DECODE, CHECKSUM")
            }
            Error::InvalidOpCommand => write!(f, "Invalid Operation Command"),
            Error::InvalidRndType => {
                write!(f, "Invalid Random Mode. Expecting ALPHA, NUM, ALPHANUM, PASSWORD")
            }
            Error::InvalidHashingAlgo => write!(
                f,
                "Invalid Hashing Algorithm. Expecting SHA256 or SHA512"
            ),
            Error::InvalidCodecAlgo => write!(f, "Invalid Codec Algorithm. Expecting BASE64"),
            Error::InvalidChecksumAlgo => {
                write!(f, "Invalid Checksum Algorithm. Expecting CRC32 or CRC32")
            }
            Error::InvalidRndLength => write!(f, "Invalid Random ID Length. Expecting number"),
            Error::UnimplementedFeature => write!(f, "Feature not available"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}
impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::Custom(err.to_string())
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Custom(err.to_string())
    }
}
