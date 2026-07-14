use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub enum WsMessage {
    Frame(Vec<u8>),
    Status(String),
}

pub struct Broadcaster {
    sender: broadcast::Sender<WsMessage>,
}

impl Broadcaster {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.sender.subscribe()
    }

    pub fn sender(&self) -> broadcast::Sender<WsMessage> {
        self.sender.clone()
    }

    pub fn send_frame(&self, bytes: Vec<u8>) {
        let _ = self.sender.send(WsMessage::Frame(bytes));
    }

    pub fn send_status(&self, text: String) {
        let _ = self.sender.send(WsMessage::Status(text));
    }
}
