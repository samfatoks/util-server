use crate::error::Error;
use crate::util::rand_util;

#[derive(Debug, PartialEq, Clone)]
pub enum RndType {
    ALPHA,
    NUM,
    ALPHANUM,
    PASSWORD,
}

impl RndType {
    pub fn generate(&self, length: u32) -> Result<String, Error> {
        let characters: &[u8] = match self {
            RndType::ALPHA => "ABCDEFGHIJKLMNOPQRSTUUVWZYZ",
            RndType::NUM => "0123456789",
            RndType::ALPHANUM => "ABCDEFGHIJKLMNOPQRSTUUVWZYZ0123456789",
            RndType::PASSWORD => "abcdefghijklmnopqrstvuwxyzABCDEFGHIJKLMNOPQRSTUUVWZYZ0123456789!@#$%^&*()[]{},.;:~=+-_",
        }
        .as_bytes();

        let result = rand_util::generate(characters, length)?;
        Ok(result)
    }
}

impl std::str::FromStr for RndType {
    type Err = Error;
    fn from_str(rnd_type: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match rnd_type {
            "ALPHA" => Ok(RndType::ALPHA),
            "NUM" => Ok(RndType::NUM),
            "ALPHANUM" => Ok(RndType::ALPHANUM),
            "PASSWORD" => Ok(RndType::PASSWORD),
            _ => Err(Error::InvalidRndType),
        }
    }
}
