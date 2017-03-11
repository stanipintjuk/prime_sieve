use std::sync::mpsc::{SendError, RecvError};
use std::sync::Arc;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result;
use std::vec::Vec;
use sieve::worker::{new_worker, MsgToWorker, MsgFromWorker};
use sieve::thread::{Thread, Send, Receive};
use sieve::math;
use sieve::math::{Partition, MathError};

pub struct ThreadPool {
    threads: Vec<Thread>,
    max_ppt: usize, // ppt = prime per thread
}

impl ThreadPool {
    pub fn new(no_threads: usize, max_ppt: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(no_threads);
        for _ in 0..no_threads {
            threads.push(new_worker());
        }

        ThreadPool {
            threads: threads,
            max_ppt: max_ppt,
        }
    }

    pub fn find_candidates(&self, init_primes: Vec<u64>) -> Result<Vec<u64>, ThreadPoolError> {
        let &last_prime = init_primes.last().unwrap();
        let max = try!(math::best_max_for_sieve(last_prime, self.max_ppt as u64));
        let partitions =
            math::best_partitioning(last_prime as usize, max as usize, self.threads.len());

        self.send_find_candidates_instruction(init_primes, partitions)
            .and_then(|running_threads| self.recv_candidate_results(running_threads))
            .map_err(ThreadPoolError::Thread)
    }

    fn send_find_candidates_instruction(&self,
                                        init_primes: Vec<u64>,
                                        partitions: Vec<Option<Partition>>)
                                        -> Result<Vec<&Thread>, Vec<ThreadError>> {
        let primes = Arc::new(init_primes);
        let mut running_threads = Vec::with_capacity(self.threads.len());
        let mut errors = vec![];
        for (thread, partition) in self.threads.iter().zip(partitions) {
            if let Some(partition) = partition.clone() {
                let result = thread.send(MsgToWorker::FindCandidates(primes.clone(), partition));
                if let Err(err) = result {
                    errors.push(ThreadError::SendError(err));
                }
                running_threads.push(thread);
            }
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(running_threads)
        }
    }

    fn recv_candidate_results(&self,
                              running_threads: Vec<&Thread>)
                              -> Result<Vec<u64>, Vec<ThreadError>> {
        let mut found_candidates = Vec::with_capacity(running_threads.len());
        let mut errors = vec![];
        for thread in running_threads.iter() {
            match thread.recv() {
                Ok(MsgFromWorker::CandidatesResult(mut candidates)) => {
                    found_candidates.append(&mut candidates);
                }
                Ok(resp) => {
                    errors.push(ThreadError::UnexpectedResponse("Unexpected response from thread \
                                                                 while finding candidat primes"
                                                                    .to_string(),
                                                                resp))
                }
                Err(err) => errors.push(ThreadError::RecvError(err)),
            }
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(found_candidates)
        }

    }


    pub fn sieve(&self,
                 prime_page: Vec<u64>,
                 candidates: Vec<u64>)
                 -> Result<Vec<u64>, ThreadPoolError> {
        let partitions = math::best_partitioning(0, candidates.len(), self.threads.len());

        let threadparts = self.threads.iter().zip(partitions);
        for (part, thread) in threadparts {}

        unimplemented!();
    }

    pub fn stop(&self) -> Result<(), ThreadPoolError> {
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
            Err(ThreadPoolError::Thread(errors))
        } else {
            Ok(())
        }
    }
}

pub enum ThreadError {
    RecvError(RecvError),
    SendError(SendError<MsgToWorker>),
    UnexpectedResponse(String, MsgFromWorker),
}

pub enum ThreadPoolError {
    Math(MathError),
    Thread(Vec<ThreadError>),
}

impl From<MathError> for ThreadPoolError {
    fn from(err: MathError) -> Self {
        ThreadPoolError::Math(err)
    }
}

impl Display for ThreadPoolError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &ThreadPoolError::Thread(ref errors) => {
                let _ = write!(f, "One or more errors occured in the thread pool \n\t");
                for err in errors {
                    let _ = match err {
                        &ThreadError::RecvError(err) => {
                            write!(f, "Failed to read from a thread. Err: {}", err)
                        }
                        &ThreadError::SendError(ref err) => {
                            write!(f, "Failed to send to a thread. Err: {}", err)
                        }
                        &ThreadError::UnexpectedResponse(ref req, ref resp) => {
                            write!(f,
                                   "Unexpected response! Thread answered '{}' on request '{}' ",
                                   req,
                                   resp)
                        }
                    };
                }
                write!(f, "\n\t")
            }

            &ThreadPoolError::Math(MathError::Limit(ref msg)) => {
                write!(f, "Math limit reached: {}", msg)
            }
        }

    }
}
