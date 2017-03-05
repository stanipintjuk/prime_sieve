use std::result::Result;
use std::u32::MAX as u32MAX;
use sieve::math::errors::MathError;

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

pub fn find_next_prime(n: u64, prime_list: &Vec<u64>) -> Result<u64, MathError> {
    let mut counter = 0u64;
    while !try!(is_coprime_to_list(counter + n, prime_list)) {
        counter += 1;
    }
    Ok(counter + n)
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
    #[test]
    fn find_next_prime_works() {
        let primes = vec![2, 3, 5];
        assert_eq!(trytest!(find_next_prime(6, &primes)), 7);
        assert_eq!(trytest!(find_next_prime(5, &primes)), 5);
    }
}
