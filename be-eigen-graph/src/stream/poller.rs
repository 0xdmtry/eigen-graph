use std::sync::Arc;

use crate::stream::state::StreamState;

pub async fn run(stream_state: Arc<StreamState>) {
    let _ = stream_state;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
