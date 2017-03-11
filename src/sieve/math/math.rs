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
    let mut partitions = Vec::with_capacity(parts);

    let mut distance = to - from;
    let delta = (distance as f64 / parts as f64).ceil() as usize;

    for part in 0..parts {
            let from = from + delta * part;

        if distance == 0 {
            partitions.push(None);
        } else if distance < delta {
            partitions.push(Some(Partition {
                from: from,
                delta: distance,
            }));
            distance = 0;
        } else {
            partitions.push(Some(Partition {
                from: from,
                delta: delta,
            }));
            distance -= delta;
        }
    }

    partitions
}

pub fn best_max_for_sieve(last_prime: u64, max: u64) -> Result<u64, MathError> {
    if let Some(theo_limit) = last_prime.checked_square() {
        if theo_limit <= max {
            Ok(theo_limit)
        } else {
            Ok(max)
        }
    } else {
        Err(MathError::Limit("Could not calculate square of a prime!".to_string()))
    }
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
    use super::{best_partitioning, best_max_for_sieve};
    use super::super::Partition;
    use std::fmt::Debug;

    #[test]
    fn best_partitioning_works_for_even() {
        let ans = best_partitioning(2, 8, 2);
        let expected = vec![Some(Partition {
                                from: 2,
                                delta: 3,
                            }),
                            Some(Partition {
                                from: 5,
                                delta: 3,
                            })];

        assert_vec_eq(ans, expected);
    }

    #[test]
    fn best_partitioning_works_for_odd()  {
        let ans = best_partitioning(7, 24, 3);
        let expected = vec![Some(Partition {
            from: 7,
            delta: 6,
        }), Some(Partition {
            from: 13,
            delta: 6,
        }), Some(Partition {
            from: 19,
            delta: 5,
        }),];
        
        assert_vec_eq(ans, expected);
    }

    #[test]
    fn best_partitioning_works_for_overflow() {
        let ans = best_partitioning(2, 7, 4);
        let expected = vec![Some(Partition {
                                from: 2,
                                delta: 2,
                            }),
                            Some(Partition {
                                from: 4,
                                delta: 2,
                            }),
                            Some(Partition {
                                from: 6,
                                delta: 1,
                            }),
                            None];

        assert_vec_eq(ans, expected);
    }

    fn assert_vec_eq<T>(vec1: Vec<T>, vec2: Vec<T>)
        where T: Clone + Debug + PartialEq
    {
        let zip = vec1.iter().cloned().zip(vec2.clone());
        for (e1, e2) in zip {
            assert_eq!(e1, e2);
        }
    }

    #[test]
    fn best_max_for_sieve_works_for_2() {
        let ans = best_max_for_sieve(2, 100).unwrap();
        assert_eq!(ans, 4)
    }

    #[test]
    fn best_max_for_sieve_works_for_trivial() {
        let ans = best_max_for_sieve(11, 500).unwrap();
        assert_eq!(ans, 121)
    }

    #[test]
    fn best_max_for_sieve_works_for_max() {
        let ans = best_max_for_sieve(11, 100).unwrap();
        assert_eq!(ans, 100)
    }
}
