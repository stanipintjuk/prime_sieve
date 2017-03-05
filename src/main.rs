mod fs;
mod sieve;
mod config;
use config::{ FILE, MAX_MEM_USAGE, CORES };

fn main() {
    let primes_pagination = fs::load_primes(FILE, MAX_MEM_USAGE);
    let thread_pool = sieve::create_pool(CORES);

    for primes_page in primes_pagination {

    }
}
