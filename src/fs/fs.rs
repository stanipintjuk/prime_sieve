use std::fs::File;
use std::io::{Result, Read};
use fs::serializer::deserialize_buf;
use config::MAX_MEM_USAGE;

const PAGE_SIZE: usize = MAX_MEM_USAGE;

pub struct PrimesPagination {
    file: File,
    position: usize,
}

impl Iterator for PrimesPagination {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; PAGE_SIZE];
        match self.file.read(&mut buf) {
            Ok(read) => {
                let vec = deserialize_buf(&buf, read);
                self.position += vec.len();
                Some(vec)
            }
            Err(_) => None,
        }
    }
}

pub fn load_primes(file_name: String) -> Result<PrimesPagination> {
    let file = try!(File::open(file_name));
    Ok(PrimesPagination {
        file: file,
        position: 0,
    })
}
