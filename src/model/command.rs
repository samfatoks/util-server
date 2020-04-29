use super::{CodecAlgorithm, HashingAlgorithm, RndType, ChecksumAlgorithm};
use crate::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    RND(RndType),
    HASH(HashingAlgorithm),
    ENCODE(CodecAlgorithm),
    DECODE(CodecAlgorithm),
    CHECKSUM(ChecksumAlgorithm)
}

impl std::str::FromStr for Command {
    type Err = Error;
    fn from_str(cmd: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let cmd = parts.get(0).unwrap();
        let mode = parts.get(1).unwrap();

        match cmd.as_ref() {
            "RND" => Ok(Command::RND(mode.parse()?)),
            "HASH" => Ok(Command::HASH(mode.parse()?)),
            "ENCODE" => Ok(Command::ENCODE(mode.parse()?)),
            "DECODE" => Ok(Command::DECODE(mode.parse()?)),
            "CHECKSUM" => Ok(Command::CHECKSUM(mode.parse()?)),
            _ => return Err(Error::InvalidCommand),
        }
    }
}
