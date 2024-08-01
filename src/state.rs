use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use tokio::sync::broadcast;

pub struct AppState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>
}

impl AppState{
    pub fn new() -> Arc<AppState> {
        let user_set: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
        let (tx, _rx) = broadcast::channel(100);
        Arc::new(AppState{ user_set, tx })
    }
}

