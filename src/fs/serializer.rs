use std::vec::Vec;

const SIZE: usize = 8;
pub fn serialize_u64(num: u64) -> [u8; SIZE] {
    let mut arr = [0u8; SIZE];

    let mut num = num;
    for i in 0..SIZE {
        arr[i] = (num & 255u64) as u8;
        num = num >> 8;
    }

    arr
}

pub fn deserialize_u64(arr: [u8; SIZE]) -> u64 {
    let mut num = 0u64;
    for i in (0..SIZE).rev() {
        num = num << 8;
        num = num + (arr[i] as u64);
    }
    num
}

pub fn deserialize_buf(buf: &[u8], read: usize) -> Vec<u64> {
    let no_primes = read / 8;
    let mut vec = Vec::with_capacity(no_primes);

    for i in 0..no_primes {
        vec.push(deserialize_chunk(&buf[i * 8..i * 8 + 8]))
    }

    vec
}

fn deserialize_chunk(vec: &[u8]) -> u64 {
    deserialize_u64([vec[0], vec[1], vec[2], vec[3], vec[4], vec[5], vec[6], vec[7]])
}
