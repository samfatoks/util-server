use super::Response;
use crate::error::Error;
use crate::state::State;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, PartialEq)]
pub enum OpCommand {
    Exit,
    History,
}
impl OpCommand {
    pub async fn process(
        self,
        peer_addr: SocketAddr,
        state: &Arc<State>,
    ) -> Result<Response, Error> {
        let response = match self {
            OpCommand::History => OpCommand::get_history(peer_addr, state).await,
            _ => Err(Error::UnimplementedFeature),
        };
        response
        //response.and_then(|r| Ok(serde_json::to_string(&r).unwrap()))
    }
    async fn get_history(peer_addr: SocketAddr, state: &Arc<State>) -> Result<Response, Error> {
        let start_exec = Instant::now();
        let history = state.get_history(peer_addr).await;
        Ok(Response::new_successful_with_extradata(
            history,
            start_exec.elapsed().as_micros(),
        ))
    }
}
impl std::str::FromStr for OpCommand {
    type Err = Error;
    fn from_str(cmd: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match cmd.to_uppercase().as_ref() {
            "/EXIT" => Ok(OpCommand::Exit),
            "/HISTORY" => Ok(OpCommand::History),
            _ => return Err(Error::InvalidOpCommand),
        }
    }
}
