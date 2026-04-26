use tokio::sync::broadcast;

pub struct EventBus {
    sender: broadcast::Sender<Vec<u8>>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { sender: tx }
    }
}
