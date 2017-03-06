use std::sync::mpsc::{ SendError, RecvError };
use std::result::Result;
use std::vec::Vec;
use sieve::worker::{new_worker, MsgToWorker};
use sieve::thread::{Thread, Send, Receive};

pub fn create_pool(no_threads: usize) -> ThreadPool {
    let mut threads = Vec::with_capacity(no_threads);
    for _ in 0..no_threads {
        threads.push(new_worker());
    }

    ThreadPool { threads: threads }
}

pub struct ThreadPool {
    threads: Vec<Thread>,
}

trait PageSieve {
    fn find_candidates(self, init_primes: Vec<u64>) -> Vec<u64>;
    fn sieve(self, prime_page: Vec<u64>, candidates: Vec<u64>) -> Vec<u64>;
    fn stop(self) -> Result<(), Vec<ThreadError>>;
}

impl PageSieve for ThreadPool {
    fn find_candidates(self, init_primes: Vec<u64>) -> Vec<u64> {
        vec!()
    }

    fn sieve(self, prime_page: Vec<u64>, candidates: Vec<u64>) -> Vec<u64> {
        vec!()
    }

    fn stop(self) -> Result<(), Vec<ThreadError>> {
        let mut errors = Vec::new();
        for thread in self.threads {

            match thread.send(MsgToWorker::Stop) {
                Err(err) => errors.push(ThreadError::SendError(err)),
                Ok(_) => 
                    match thread.recv() {
                        Err(err) => errors.push(ThreadError::RecvError(err)),
                        Ok(_) => {}
                    }
            }

        }

        if errors.len() == 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

enum ThreadError {
    RecvError(RecvError),
    SendError(SendError<MsgToWorker>),
}
