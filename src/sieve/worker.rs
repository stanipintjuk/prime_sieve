use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::sync::Arc;
use sieve::math::{find_candidates, sieve_page, MathError, Partition};

pub type ArcVec = Arc<Vec<u64>>;

pub enum MsgToWorker {
    FindCandidates(ArcVec, Partition),
    Sieve(ArcVec, ArcVec),
    Stop,
}

pub enum MsgFromWorker {
    CandidatesResult(Vec<u64>),
    SieveResult(Vec<u64>),
    Error(MathError),
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
        let msg = match rec.recv() {
            Ok(msg) => msg,
            Err(_) => break,
        };

        let ans = match msg {

            MsgToWorker::FindCandidates(init_primes, partition) => {
                match find_candidates(&init_primes, partition) {
                    Ok(candidates) => MsgFromWorker::CandidatesResult(candidates),
                    Err(err) => MsgFromWorker::Error(err),
                }
            }

            MsgToWorker::Sieve(primes_page, candidates) => {
                match sieve_page(&primes_page, &candidates) {
                    Ok(primes) => MsgFromWorker::SieveResult(primes),
                    Err(err) => MsgFromWorker::Error(err),
                }
            }

            MsgToWorker::Stop => break,
        };

        match send.send(ans) {
            Ok(_) => (),
            Err(_) => panic!(),
        }

    }

    cleanup()
}

fn cleanup() {}
