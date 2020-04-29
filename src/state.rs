use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::Mutex;

pub struct State {
    db: Mutex<HashMap<SocketAddr, Vec<String>>>,
}

impl State {
    pub fn new() -> Self {
        State {
            db: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_request(&self, addr: SocketAddr, request: String) {
        self.db
            .lock()
            .await
            .entry(addr)
            .or_insert(vec![])
            .push(request);
    }
    pub async fn get_history(&self, peer_addr: SocketAddr) -> Value {
        let db = self.db.lock().await;
        let requests: Vec<String> = db.get(&peer_addr).unwrap().to_vec();
        serde_json::to_value(&requests).unwrap()
    }

    // pub async fn get_all_history(&self) -> Value {
    //     let db = self.db.lock().await;
    //     let requests: Vec<&String> = db.values().flatten().collect();
    //     serde_json::to_value(&requests).unwrap()
    // }
}
