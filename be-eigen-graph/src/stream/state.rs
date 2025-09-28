use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct Event(pub String);

#[derive(Clone, Debug, Default)]
pub struct Cursor {
    pub last_ts: i64,
    pub last_id: String,
    pub since_hint: i64,
}

#[derive(Debug)]
pub struct StreamState {
    inner: Mutex<HashMap<String, broadcast::Sender<Event>>>,
    cap: usize,
    interested: Mutex<HashSet<String>>,
    cursors: Mutex<HashMap<String, Cursor>>,
}

impl StreamState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            cap: 1024,
            interested: Mutex::new(HashSet::new()),
            cursors: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_interest(&self, token: &str, since: Option<i64>) {
        {
            let mut s = self.interested.lock().unwrap_or_else(|p| p.into_inner());
            s.insert(token.to_string());
        }
        if let Some(since_ts) = since {
            let mut c = self.cursors.lock().unwrap_or_else(|p| p.into_inner());
            let e = c.entry(token.to_string()).or_default();
            if e.since_hint < since_ts {
                e.since_hint = since_ts;
            }
        }
    }

    pub fn advance_cursor(&self, token: &str, last_ts: i64, last_id: &str) {
        let mut c = self.cursors.lock().unwrap_or_else(|p| p.into_inner());
        let e = c.entry(token.to_string()).or_default();
        if last_ts > e.last_ts || (last_ts == e.last_ts && *last_id > *e.last_id) {
            e.last_ts = last_ts;
            e.last_id = last_id.to_string();
        }
    }

    pub fn tokens_to_poll(&self) -> Vec<(String, Cursor)> {
        let s = self.interested.lock().unwrap_or_else(|p| p.into_inner());
        let c = self.cursors.lock().unwrap_or_else(|p| p.into_inner());
        s.iter()
            .map(|t| {
                let cur = c.get(t).cloned().unwrap_or_default();
                (t.clone(), cur)
            })
            .collect()
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
