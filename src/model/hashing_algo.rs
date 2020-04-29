use crate::error::Error;
use crate::util::hash_util;

#[derive(Debug, PartialEq, Clone)]
pub enum HashingAlgorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
}

impl HashingAlgorithm {
    pub fn hash(&self, text: String) -> Result<String, Error> {
        let hash_string = match self {
            HashingAlgorithm::SHA1 => hash_util::sha1_digest(&text)?,
            HashingAlgorithm::SHA256 => hash_util::sha256_digest(&text)?,
            HashingAlgorithm::SHA512 => hash_util::sha512_digest(&text)?,
            _ => return Err(Error::UnimplementedFeature),
        };
        Ok(hash_string)
    }
}

impl std::str::FromStr for HashingAlgorithm {
    type Err = Error;
    fn from_str(algo: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match algo {
            "MD5" => Ok(HashingAlgorithm::MD5),
            "SHA1" => Ok(HashingAlgorithm::SHA1),
            "SHA256" => Ok(HashingAlgorithm::SHA256),
            "SHA512" => Ok(HashingAlgorithm::SHA512),
            _ => Err(Error::InvalidHashingAlgo),
        }
    }
}
