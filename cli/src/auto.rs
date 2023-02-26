use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn start_auto_tick(tick_rate: Duration) -> AutoTick {
    let (tx, rx) = mpsc::channel();

    let join_handle = thread::spawn(move || loop {
        thread::sleep(tick_rate);

        if tx.send(()).is_err() {
            break;
        }
    });

    AutoTick {
        receiver: rx,
        join_handle,
    }
}

pub struct AutoTick {
    pub receiver: Receiver<()>,
    pub join_handle: JoinHandle<()>,
}
