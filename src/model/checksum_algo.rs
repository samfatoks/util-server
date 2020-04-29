use crate::error::Error;
use crate::util::checksum_util;

#[derive(Debug, PartialEq, Clone)]
pub enum ChecksumAlgorithm {
    CRC16,
    CRC32,
}

impl ChecksumAlgorithm {
    pub fn calculate_checksum(&self, text: String) -> Result<String, Error> {
        let result = match self {
            ChecksumAlgorithm::CRC16 => checksum_util::crc16(&text),
            ChecksumAlgorithm::CRC32 => checksum_util::crc32(&text),
        };
        Ok(result)
    }
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
