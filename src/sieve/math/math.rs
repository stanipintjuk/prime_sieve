use std::result::Result;
use std::u32::MAX as u32MAX;
use std::option::Option;
use sieve::math::errors::MathError;
use sieve::math::Partition;

pub fn init_primes() -> Vec<u64> {
    vec![2, 3, 5, 7, 11]
}

fn is_coprime_to_list(n: u64, list: &Vec<u64>) -> Result<bool, MathError> {
    if n > (u32MAX as u64) {
        let msg = format!("Number '{}' is to big for checking co-primality in an \
                          effective way.",
                          n);
        return Err(MathError::Limit(msg.to_string()));
    };
    let upper_limit = (n as f64).sqrt() as u64 + 1;

    for p in list {
        if *p > upper_limit {
            return Ok(true);
        } else if !is_coprime(n, *p) {
            return Ok(false);
        };
    }
    return Ok(true);
}

fn is_coprime(n: u64, c: u64) -> bool {
    !(n % c == 0)
}

pub fn find_candidates(init_primes: &Vec<u64>, part: Partition) -> Result<Vec<u64>, MathError> {
    let mut candidates = Vec::new();
    let to = part.from + part.delta;

    for p in part.from..to {
        if try!(is_coprime_to_list(p as u64, init_primes)) {
            candidates.push(p as u64)
        }
    }

    Ok(candidates)
}

pub fn sieve_page(primes_page: &Vec<u64>, candidates: &Vec<u64>) -> Result<Vec<u64>, MathError> {
    let mut sieved = Vec::new();
    for &p in candidates {
        if try!(is_coprime_to_list(p, primes_page)) {
            sieved.push(p);
        }
    }
    Ok(sieved)
}

pub fn best_partitioning(from: usize, to: usize, parts: usize) -> Vec<Option<Partition>> {
    let mut partition = Vec::with_capacity(parts);
    let delta = to - from;
    let size = delta / parts;
    for part in 0..parts {
        let from = from + part * size;
        partition.push(Some(Partition {
            from: from,
            delta: size,
        }));
    }
    partition
}

pub fn best_max_for_sieve(prime: u64, threads: u64) -> u64 {
    unimplemented!();
}

pub trait CheckedSquare {
    fn checked_square(self) -> Option<u64>;
}

impl CheckedSquare for u64 {
    fn checked_square(self) -> Option<u64> {
        const MAX: u64 = u32MAX as u64;
        if self < MAX { Some(self.pow(2)) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use sieve::math::{ Partition, best_partitioning };

    #[test]
    fn best_partitioning_works_for_even() {
        let ans = best_partitioning(2, 8, 2);
        let exp = vec![
            Some(Partition{from:2, delta:3}),
            Some(Partition{from:5, delta:3})
        ];

        let ansexp = ans.iter().zip(exp);
        for (ref ans, ref expected) in ansexp {
            assert_eq!(ans.unwrap(), expected.unwrap());
        }
    }

    #[test]
    fn best_partitioning_works_for_overflow() {
        let ans = best_partitioning(2, 7, 4);
        let expected = vec![
            Some(Partition{from:2, delta:2}),
            Some(Partition{from:3, delta:2}),
            Some(Partition{from:3, delta:1}),
            None
        ];

        let ansexp = ans.iter().zip(expected);
        for (ref ans, ref expected) in ansexp {
            assert_eq!(ans.unwrap(), expected.unwrap());
        }
    }

}
