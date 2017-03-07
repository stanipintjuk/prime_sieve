use std::fs::{File, OpenOptions};
use std::io::{Result, Read, Write};
use fs::serializer::{deserialize_buf, serialize_u64};
use config::MAX_MEM_USAGE;

const PAGE_SIZE: usize = MAX_MEM_USAGE / 2;
const BUF_SIZE: usize = 4194304;

pub struct PrimesPagination {
    pub file: File,
    pub position: usize,
}

impl Iterator for PrimesPagination {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; BUF_SIZE];
        let mut primes = Vec::with_capacity(PAGE_SIZE);

        let mut read_total = 0usize;

        while read_total < PAGE_SIZE {
            match self.file.read(&mut buf) {
                Ok(read) => {
                    if read == 0 {
                        break;
                    }
                    println!("Read {}", read);
                    read_total += read;
                    if read_total > PAGE_SIZE {
                        let delta = read_total - PAGE_SIZE;
                        deserialize_to_vec(&buf, read - delta, &mut primes)
                    } else {
                        deserialize_to_vec(&buf, read, &mut primes)
                    }
                }
                Err(_) => return None,
            }
        }

        Some(primes)
    }


}


fn deserialize_to_vec(buf: &[u8], read: usize, out_vec: &mut Vec<u64>) {
    let mut vec = deserialize_buf(&buf, read);
    out_vec.append(&mut vec);
}

pub fn load_primes(file_name: String) -> Result<PrimesPagination> {
    let file = try!(File::open(file_name));
    Ok(PrimesPagination {
        file: file,
        position: 0,
    })
}

pub fn save_primes(primes: &Vec<u64>, fname: String) -> Result<()> {
    let fname = &fname;
    let mut file = match OpenOptions::new()
        .append(true)
        .open(fname) {
        Ok(file) => file,
        Err(_) => try!(File::create(fname)),
    };

    for &p in primes {
        try!(file.write_all(&serialize_u64(p)));
    }

    Ok(())
}
