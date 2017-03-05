use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub enum MsgToWorker {
    Stop,
}

pub enum MsgFromWorker {
    Ok,
}

pub fn new_worker() -> (Sender<MsgToWorker>, Receiver<MsgFromWorker>) {
    let (s_tw, r_tw) = channel();
    let (s_fw, r_fw) = channel();

    thread::spawn(move || { worker(s_fw, r_tw); });

    (s_tw, r_fw)
}

fn worker(send: Sender<MsgFromWorker>, rec: Receiver<MsgToWorker>) {
    loop {
        match rec.recv() {
            Ok(msg) => {
                match msg {
                    Stop => break,
                }
            }
            Err(_) => break,

        }
    }

    cleanup()
}

fn cleanup() {}
