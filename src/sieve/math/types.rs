use std::fmt::{Debug, Formatter, Error};
use std::cmp::PartialEq;

pub struct Partition {
    pub from: usize,
    pub delta: usize,
}

impl Debug for Partition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Partition(from = {}, Δ = {})", self.from, self.delta)
    }
}

impl PartialEq<Partition> for Partition {
    fn eq(&self, other: &Partition) -> bool {
        self.from == other.from && self.delta == other.delta
    }

    fn ne(&self, other: &Partition) -> bool {
        !(self == other)
    }
}
