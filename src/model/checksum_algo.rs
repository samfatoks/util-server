use super::Response;
use crate::error::Error;
use crate::util::checksum_util;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
pub enum ChecksumAlgorithm {
    CRC16,
    CRC32,
}

impl std::str::FromStr for ChecksumAlgorithm {
    type Err = Error;
    fn from_str(algo: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match algo {
            "CRC16" => Ok(ChecksumAlgorithm::CRC16),
            "CRC32" => Ok(ChecksumAlgorithm::CRC32),
            _ => Err(Error::InvalidChecksumAlgo),
        }
    }
}

impl ChecksumAlgorithm {
    pub fn calculate_checksum(&self, text: String) -> Result<Response, Error> {
        let execution_time = Instant::now();
        let value = match self {
            ChecksumAlgorithm::CRC16 => checksum_util::crc16(&text),
            ChecksumAlgorithm::CRC32 => checksum_util::crc32(&text),
        };
        Ok(Response::new_successful(
            value,
            execution_time.elapsed().as_micros(),
        ))
    }
}
