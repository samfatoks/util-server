mod error;
mod model;
mod server;
mod state;
mod util;
use server::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let server = Server::new(addr);
    if let Err(err) = server.start().await {
        eprintln!("{}", err);
    }
}
