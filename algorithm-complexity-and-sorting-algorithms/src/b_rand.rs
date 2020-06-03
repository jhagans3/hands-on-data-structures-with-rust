use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn rand(max: usize) -> usize {
    // note: unwrap is bad
    // if others hold the mutex and panics this can fail
    RG.lock().unwrap().next_v(max)
}

pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 56394237,
            inc: 34642349,
            modulo: 23254544563,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rands_printout() {
        // cargo test test_rands_printout -- --nocapture
        let mut r = RandGen::new(12);
        for _ in 0..100 {
            println!("--{}", r.next_v(100));
        }
    }
}
