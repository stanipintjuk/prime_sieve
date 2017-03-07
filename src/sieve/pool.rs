use std::sync::mpsc::{SendError, RecvError};
use std::result::Result;
use std::vec::Vec;
use sieve::worker::{new_worker, MsgToWorker, MsgFromWorker};
use sieve::thread::{Thread, Send, Receive};
use sieve::math;
use sieve::math::{Partition};

pub struct ThreadPool {
    threads: Vec<Thread>,
}

impl ThreadPool {
    pub fn new(no_threads: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(no_threads);
        for _ in 0..no_threads {
            threads.push(new_worker());
        }

        ThreadPool { threads: threads }
    }

    pub fn find_candidates(&self, init_primes: Vec<u64>) -> Result<Vec<u64>, Vec<ThreadError>> {
        let &last_prime = init_primes.last().unwrap();
        let max = math::best_max_for_sieve(last_prime, self.threads.len() as u64);
        let partitions = math::best_partitioning(last_prime as usize, max as usize, self.threads.len());
        
        unimplemented!();
    }

    pub fn sieve(&self,
                 prime_page: Vec<u64>,
                 candidates: Vec<u64>)
                 -> Result<Vec<u64>, Vec<ThreadError>> {
        let partitions = math::best_partitioning(0, candidates.len(), self.threads.len());

        let threadparts = self.threads.iter().zip(partitions);
        for (part, thread) in threadparts {}

        unimplemented!();
    }

    pub fn stop(&self) -> Result<(), Vec<ThreadError>> {
        let mut errors = Vec::new();
        for thread in &self.threads {

            match thread.send(MsgToWorker::Stop) {
                Err(err) => errors.push(ThreadError::SendError(err)),
                Ok(_) => continue,
            }

            match thread.recv() {
                Err(err) => errors.push(ThreadError::RecvError(err)),
                Ok(_) => {}
            }
        }

        if errors.len() == 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

pub enum ThreadError {
    RecvError(RecvError),
    SendError(SendError<MsgToWorker>),
    UnexpectedResponse(MsgToWorker, MsgFromWorker),
}
