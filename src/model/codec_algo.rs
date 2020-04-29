use super::Response;
use crate::error::Error;
use crate::util::codec_util;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
pub enum CodecAlgorithm {
    BASE64,
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

impl CodecAlgorithm {
    pub fn encode(&self, text: String) -> Result<Response, Error> {
        let execution_time = Instant::now();
        let encoded_string = match self {
            CodecAlgorithm::BASE64 => codec_util::base64_encode(&text)?,
        };
        Ok(Response::new_successful(
            encoded_string,
            execution_time.elapsed().as_micros(),
        ))
    }
    pub fn decode(&self, text: String) -> Result<Response, Error> {
        let execution_time = Instant::now();
        let decoded_string = match self {
            CodecAlgorithm::BASE64 => codec_util::base64_decode(&text)?,
        };
        Ok(Response::new_successful(
            decoded_string,
            execution_time.elapsed().as_micros(),
        ))
    }
}
