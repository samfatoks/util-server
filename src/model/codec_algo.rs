use crate::error::Error;
use crate::util::codec_util;

#[derive(Debug, PartialEq, Clone)]
pub enum CodecAlgorithm {
    BASE64,
}

impl CodecAlgorithm {
    pub fn encode(&self, text: String) -> Result<String, Error> {
        let result = match self {
            CodecAlgorithm::BASE64 => codec_util::base64_encode(&text)?,
        };
        Ok(result)
    }
    pub fn decode(&self, text: String) -> Result<String, Error> {
        let result = match self {
            CodecAlgorithm::BASE64 => codec_util::base64_decode(&text)?,
        };
        Ok(result)
    }
}

impl std::str::FromStr for CodecAlgorithm {
    type Err = Error;
    fn from_str(algo: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match algo {
            "BASE64" => Ok(CodecAlgorithm::BASE64),
            _ => Err(Error::InvalidCodecAlgo),
        }
    }
}
