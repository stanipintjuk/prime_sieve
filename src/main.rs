mod fs;
mod sieve;
mod config;
use config::{FILE, CORES};
use sieve::{math, ThreadPool, ThreadError, MsgToWorker, MsgFromWorker};
use std::result::Result;
use std::io::{stdin, Error as IOError, ErrorKind};
use std::vec::Vec;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::convert::From;

enum SieveError {
    IO(IOError),
    Thread(Vec<ThreadError>),
    PrimesFileEmpty,
}

fn sieve_file(thread_pool: &ThreadPool, fname: String) -> Result<Vec<u64>, SieveError> {
    let mut primes_pager = try!(fs::load_primes(fname));

    let init_primes = match primes_pager.next() {
        Some(primes) => primes,
        None => return Err(SieveError::PrimesFileEmpty),
    };

    let mut candidates = try!(thread_pool.find_candidates(init_primes));

    for page in primes_pager {
        candidates = try!(thread_pool.sieve(page, candidates));
    }

    Ok(candidates)
}

fn sieve(thread_pool: &ThreadPool, fname: String) -> Result<Vec<u64>, SieveError> {
    match sieve_file(thread_pool, fname) {
        Ok(primes) => Ok(primes),
        Err(err) => match err {
            SieveError::IO(ioerr) =>  {
                match ioerr.kind() {
                    ErrorKind::NotFound => Ok(math::init_primes()),
                    err => Err(SieveError::IO(IOError::new(err, ioerr)))
                }
            }
            err => Err(err)
        }
    }
}

fn main() {
    let thread_pool = sieve::ThreadPool::new(CORES);
    loop {
        let sieve_result = sieve(&thread_pool, FILE.to_string());
        match sieve_result {
            Ok(primes) => {
                println!("Sieve found primes {:?}", primes);
                let _ = fs::save_primes(&primes, FILE.to_string());
                println!("saved");
                read_line();
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

pub fn read_line() -> Option<String> {
    let mut num = String::new();
    match stdin().read_line(&mut num) {
        Ok(_) => Some(num),
        Err(_) => None,
    }
}

impl Display for SieveError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &SieveError::PrimesFileEmpty => {
                write!(f,
                       "The primes file was loaded but didn't contain any numbers.")
            }
            &SieveError::IO(ref err) => write!(f, "IO Error: \n\t{}", err),
            &SieveError::Thread(ref errors) => {
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
        }
    }
}

impl Display for MsgToWorker {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &MsgToWorker::FindCandidates(_, _) => write!(f, "FindCandidates(...)"),
            &MsgToWorker::Sieve(_, _) => write!(f, "Sieve(...)"),
            &MsgToWorker::Stop => write!(f, "Stop"),
        }
    }
}

impl Display for MsgFromWorker {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &MsgFromWorker::CandidatesResult(_) => write!(f, "CandidatesResult(...)"),
            &MsgFromWorker::SieveResult(_) => write!(f, "SieveResult(...)"),
            &MsgFromWorker::Error(_) => write!(f, "Error(...)"),
            &MsgFromWorker::Ok => write!(f, "Ok"),
        }
    }
}

impl From<IOError> for SieveError {
    fn from(err: IOError) -> SieveError {
        SieveError::IO(err)
    }
}

impl From<Vec<ThreadError>> for SieveError {
    fn from(err: Vec<ThreadError>) -> SieveError {
        SieveError::Thread(err)
    }
}
