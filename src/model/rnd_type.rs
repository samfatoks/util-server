use super::Response;
use crate::error::Error;
use crate::util::rand_util;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
pub enum RndType {
    ALPHA,
    NUM,
    ALPHANUM,
    PASSWORD,
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

impl RndType {
    pub fn generate(&self, length: u32) -> Result<Response, Error> {
        let start_exec = Instant::now();
        let characters: &[u8] = match self {
            RndType::ALPHA => "ABCDEFGHIJKLMNOPQRSTUUVWZYZ",
            RndType::NUM => "0123456789",
            RndType::ALPHANUM => "ABCDEFGHIJKLMNOPQRSTUUVWZYZ0123456789",
            RndType::PASSWORD => "abcdefghijklmnopqrstvuwxyzABCDEFGHIJKLMNOPQRSTUUVWZYZ0123456789!@#$%^&*()[]{},.;:~=+-_",
        }
        .as_bytes();

        let response_message = rand_util::generate(characters, length)?;

        let response = Response::new_successful(response_message, start_exec.elapsed().as_micros());
        Ok(response)
    }
}
