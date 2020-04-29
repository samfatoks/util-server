use super::{Command, Response};
use crate::error::Error;
use crate::state::State;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Request {
    pub command: Command,
    pub rnd_size: Option<u32>,
    pub text: Option<String>,
}

impl Request {
    fn new(command: Command, rnd_size: Option<u32>, text: Option<String>) -> Self {
        Request {
            command,
            rnd_size,
            text,
        }
    }
    pub async fn process(
        self,
        _peer_addr: SocketAddr,
        _state: &Arc<State>,
    ) -> Result<Response, Error> {
        let response = match self.clone().command {
            Command::RND(rnd_type) => rnd_type.generate(self.rnd_size.unwrap()),
            Command::HASH(algo) => algo.hash(self.text.clone().unwrap()),
            Command::ENCODE(algo) => algo.encode(self.text.clone().unwrap()),
            Command::DECODE(algo) => algo.decode(self.text.clone().unwrap()),
            Command::CHECKSUM(algo) => algo.calculate_checksum(self.text.clone().unwrap()),
        };
        response
        //response.and_then(|r| Ok(serde_json::to_string(&r).unwrap()))
    }
}

impl std::str::FromStr for Request {
    type Err = Error;
    fn from_str(msg: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let split_index = get_second_space_index(msg);
        if split_index > 0 {
            let (cmd_mode, data) = msg.split_at(split_index);
            let cmd: Command = cmd_mode.to_uppercase().parse()?;
            let request = match cmd {
                Command::RND(_) => Request::new(
                    cmd,
                    Some(data.trim().parse().map_err(|_| Error::InvalidRndLength)?),
                    None,
                ),
                _ => Request::new(cmd, None, Some(data.trim().to_string())),
            };
            Ok(request)
        } else {
            Err(Error::InvalidMessage)
        }
    }
}

fn get_second_space_index(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut c = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            c += 1;
            if c == 2 {
                return i;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::model::{Command, Request, RndType};
    #[test]
    fn valid_message() {
        let request_str = "RND ALPHA 30";
        let request = request_str.parse::<Request>().unwrap();
        assert_eq!(request.command, Command::RND(RndType::ALPHA));
        assert_eq!(request.rnd_size, Some(30));
    }
    #[test]
    fn invalid_message() {
        let request_str = "AAA BBB";
        if let Err(err) = request_str.parse::<Request>() {
            assert_eq!(
                err.to_string(),
                "Invalid Message size. Expecing COMMAND MODE ATTRIBUTE/DATA e.g RND ALPHA 20"
            );
        }
    }
    #[test]
    fn invalid_command() {
        let request_str = "AAA BBB CCC";
        if let Err(err) = request_str.parse::<Request>() {
            assert_eq!(
                err.to_string(),
                "Invalid Command. Expecting RND, HASH, ENCODE or DECODE"
            );
        }
    }
    #[test]
    fn invalid_rnd_type() {
        let request_str = "RND BBB 20";
        if let Err(err) = request_str.parse::<Request>() {
            assert_eq!(
                err.to_string(),
                "Invalid Random Mode. Expecting ALPHA, NUM or ALPHANUM"
            );
        }
    }
    #[test]
    fn invalid_hashing_algorithm() {
        let request_str = "HASH BBB This is a simple message";
        if let Err(err) = request_str.parse::<Request>() {
            assert_eq!(
                err.to_string(),
                "Invalid Hashing Algorithm. Expecting MD5, SHA1, SHA256 or SHA512"
            );
        }
    }
}
