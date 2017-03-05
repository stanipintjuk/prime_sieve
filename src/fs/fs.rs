pub struct PrimesPagination {}

pub fn load_primes(file: &str, max_mem_usage: u64) -> PrimesPagination {
    PrimesPagination {}
}

impl Iterator for PrimesPagination {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(vec!())
    }
}
