use crate::error::Error;
use crate::model::{OpCommand, Request, Response};
use crate::state::State;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};

pub struct Server {
    addr: SocketAddr,
    state: Arc<State>,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        let state = Arc::new(State::new());
        Server { addr, state }
    }
}
impl Server {
    pub async fn start(&self) -> Result<(), Error> {
        let mut listener = TcpListener::bind(self.addr).await?;
        println!(
            "Utility Server started on {}:{}",
            self.addr.ip(),
            self.addr.port()
        );

        while let Some(stream) = listener.next().await {
            match stream {
                Ok(stream) => {
                    let peer_addr = stream.peer_addr().unwrap();
                    println!(
                        "Accepted connection from {}:{}",
                        peer_addr.ip(),
                        peer_addr.port()
                    );
                    let state = self.state.clone();
                    tokio::spawn(async move { handle_stream(stream, &state, peer_addr).await });
                }
                Err(e) => eprintln!("error accepting socket; error = {:?}", e),
            }
        }

        println!("Shutting down.");
        Ok(())
    }
}

async fn handle_stream(stream: TcpStream, state: &Arc<State>, peer_addr: SocketAddr) {
    let mut lines = Framed::new(stream, LinesCodec::new());

    lines
        .send(String::from("Please enter your command. e.g RND ALPHA 20"))
        .await
        .unwrap();
    while let Some(result) = lines.next().await {
        match result {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                state.add_request(peer_addr, line.clone()).await;

                let response = if line.starts_with("/") {
                    match line.parse::<OpCommand>() {
                        Ok(command) => match command {
                            OpCommand::Exit => {
                                println!(
                                    "Client exiting server - {}:{}",
                                    peer_addr.ip(),
                                    peer_addr.port()
                                );
                                return;
                            }
                            _ => command
                                .process(peer_addr, state)
                                .await
                                .unwrap_or_else(|e| Response::from(e)),
                        },
                        Err(err) => Response::from(err),
                    }
                } else {
                    match line.parse::<Request>() {
                        Ok(request) => request
                            .process(peer_addr, state)
                            .await
                            .unwrap_or_else(|e| Response::from(e)),
                        Err(err) => Response::from(err),
                    }
                };
                let response_str = serde_json::to_string(&response).unwrap();
                if let Err(e) = lines.send(response_str).await {
                    eprintln!("error on sending response; error = {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("error on decoding from socket; error = {:?}", e);
            }
        }
    }
}
