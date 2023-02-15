use gb_rs_asm::operations::Operation;
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Default)]
pub struct Inspector {
    sender: Option<Sender<Message>>,
}

impl Inspector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self) -> Receiver<Message> {
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);

        rx
    }

    pub fn send(&mut self, message: Message) {
        let Some(sender) = &self.sender else {
            return;
        };

        if let Err(_) = sender.send(message) {
            self.sender = None;
        }
    }

    pub fn send_fn<F>(&mut self, content_fn: F)
    where
        F: FnOnce() -> Message,
    {
        self.send(content_fn());
    }
}

pub enum Message {
    Operation(Operation),
    Step,
}
