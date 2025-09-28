use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct Event(pub String);

#[derive(Debug)]
pub struct StreamState {
    inner: Mutex<HashMap<String, broadcast::Sender<Event>>>,
    cap: usize,
}

impl StreamState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            cap: 1024,
        }
    }

    pub fn subscribe(&self, token: &str) -> broadcast::Receiver<Event> {
        let mut guard = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        let tx = guard.entry(token.to_string()).or_insert_with(|| {
            let (tx, _rx) = broadcast::channel::<Event>(self.cap);
            tx
        });
        tx.subscribe()
    }

    pub fn publish(&self, token: &str, evt: Event) -> usize {
        let guard = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(tx) = guard.get(token) {
            tx.send(evt).unwrap_or(0)
        } else {
            0
        }
    }

    pub fn subscriber_count(&self, token: &str) -> usize {
        let guard = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        guard.get(token).map(|tx| tx.receiver_count()).unwrap_or(0)
    }
}

impl Default for StreamState {
    fn default() -> Self {
        Self::new()
    }
}

pub type SharedStreamState = Arc<StreamState>;
