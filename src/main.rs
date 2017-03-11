mod fs;
mod sieve;
mod config;
use config::{FILE, CORES};
use sieve::{math, ThreadPool, ThreadError, ThreadPoolError};
use std::result::Result;
use std::io::{stdin, Error as IOError, ErrorKind};
use std::vec::Vec;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::convert::From;

enum SieveError {
    IO(IOError),
    Thread(ThreadPoolError),
    PrimesFileEmpty,
}

fn sieve_file(thread_pool: &ThreadPool, fname: String) -> Result<Vec<u64>, SieveError> {
    let mut primes_pager = try!(fs::load_primes(fname));

    if let Some(init_primes) = (&mut primes_pager).next() {
        let mut candidates = try!(thread_pool.find_candidates(init_primes));

        for page in primes_pager {
            candidates = try!(thread_pool.sieve(page, candidates));
        }

        Ok(candidates)
    } else {
        Err(SieveError::PrimesFileEmpty)
    }
}

fn sieve(thread_pool: &ThreadPool, fname: String) -> Result<Vec<u64>, SieveError> {
    match sieve_file(thread_pool, fname) {
        Ok(primes) => Ok(primes),
        Err(err) => {
            match err {
                SieveError::IO(ioerr) => {
                    match ioerr.kind() {
                        ErrorKind::NotFound => Ok(math::init_primes()),
                        _ => Err(SieveError::IO(ioerr)),
                    }
                }
                _ => Err(err),
            }
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
                println!("got some kind of error lol {}", err);
                break;
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
            &SieveError::Thread(ref error) => write!(f, "Error in thread pool\n\t{}", error),
        }
    }
}

impl From<IOError> for SieveError {
    fn from(err: IOError) -> SieveError {
        SieveError::IO(err)
    }
}

impl From<ThreadPoolError> for SieveError {
    fn from(err: ThreadPoolError) -> SieveError {
        SieveError::Thread(err)
    }
}

