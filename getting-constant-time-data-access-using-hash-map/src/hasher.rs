use std::hash::{Hash, Hasher};

pub struct MyHash {
    prev: u8,
    number: u128,
}

impl Hasher for MyHash {
    fn write(&mut self, dt: &[u8]) {
        for d in dt {
            self.number = ((self.number + 11) * (*d as u128 + 13) + ((d ^ self.prev) as u128))
                % (std::u64::MAX as u128);
            self.prev = *d;
        }
    }

    fn finish(&self) -> u64 {
        self.number as u64
    }
}

pub fn hash<T: Hash>(seed: u64, t: T) -> u64 {
    let mut h = MyHash { number: 0, prev: 0 };
    h.write_u64(seed);
    t.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_hasher() {
        let n = hash(55, "cat");
        assert_eq!(n, hash(55, "cat"));

        assert!(hash(55, "abc") != hash(55, "cba"));
    }

    #[test]
    pub fn test_numbers() {
        let mut prev = 0;
        for x in 0..10_000 {
            let curr = hash(55, x);
            assert!(curr != prev);
            prev = curr;
        }
    }
}
