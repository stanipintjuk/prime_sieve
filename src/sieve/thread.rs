use std::sync::mpsc::{Sender, Receiver, SendError, RecvError};
use sieve::worker::{MsgToWorker, MsgFromWorker};
use std::result::Result;

pub type Thread = (Sender<MsgToWorker>, Receiver<MsgFromWorker>);

pub trait Send {
    fn send(&self, MsgToWorker) -> Result<(), SendError<MsgToWorker>>;
}

pub trait Receive {
    fn recv(&self) -> Result<MsgFromWorker, RecvError>;
}

impl Send for Thread {
    fn send(&self, msg: MsgToWorker) -> Result<(), SendError<MsgToWorker>> {
        self.0.send(msg)
    }
}

impl Receive for Thread {
    fn recv(&self) -> Result<MsgFromWorker, RecvError> {
        self.1.recv()
    }
}
