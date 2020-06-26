mod hasher;

pub use hasher::hash;
use std::borrow::Borrow;
use std::hash::Hash;

const BUCKET_SIZE: usize = 8;
// const BUCKET_GROW: usize = 8;

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> BucketList<K, V> {
    fn new() -> Self {
        BucketList {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }

    // usize returned how big chosen bucket is
    // tell caller if its too full
    fn push(&mut self, k: K, v: V) -> usize {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        self.buckets[h].push((k, v));
        self.len += 1;
        self.buckets[h].len()
    }

    // Key Borrow KB
    fn get<KB>(&self, k: &KB) -> Option<&V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &self.buckets[h] {
            if k == ik.borrow() {
                return Some(iv);
            }
        }
        None
    }

    fn get_mut<KB>(&mut self, k: &KB) -> Option<&mut V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &mut self.buckets[h] {
            if k == (ik as &K).borrow() {
                return Some(iv);
            }
        }
        None
    }

    fn bucket(&mut self, n: usize) -> Option<Vec<(K, V)>> {
        if n >= self.buckets.len() {
            return None;
        }
        let mut res = Vec::new();
        std::mem::swap(&mut res, &mut self.buckets[n]);
        self.len -= res.len();
        Some(res)
    }

    fn set_buckets(&mut self, n: usize) {
        for _ in self.buckets.len()..n {
            self.buckets.push(Vec::new())
        }
    }
}

#[derive(Debug)]
pub struct HMap<K, V> {
    n_moved: usize,
    main: BucketList<K, V>,
    grow: BucketList<K, V>,
}

impl<K: Hash + Eq, V> HMap<K, V> {
    pub fn new() -> Self {
        HMap {
            n_moved: 0,
            main: BucketList::new(),
            grow: BucketList::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some(iv) = self.main.get_mut(&k) {
            *iv = v;
            return;
        }
        if let Some(iv) = self.grow.get_mut(&k) {
            *iv = v;
            return;
        }

        if self.n_moved > 0 {
            // we have started move to bigger bucket
            self.grow.push(k, v);
            self.move_bucket();
            return;
        }

        if self.main.push(k, v) > BUCKET_SIZE / 2 {
            // grow buckets
            self.move_bucket();
        }
    }

    pub fn get<KR>(&self, kr: &KR) -> Option<&V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        self.main.get(kr).or_else(|| self.grow.get(kr))
    }

    pub fn get_mut<KR>(&mut self, kr: &KR) -> Option<&mut V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        if let Some(b) = self.main.get_mut(kr) {
            return Some(b);
        }
        self.grow.get_mut(kr)
    }

    pub fn len(&self) -> usize {
        self.main.len + self.grow.len
    }

    pub fn move_bucket(&mut self) {
        if self.n_moved == 0 {
            self.grow.set_buckets(self.main.buckets.len() * 2);
        }
        if let Some(b) = self.main.bucket(self.n_moved) {
            for (k, v) in b {
                self.grow.push(k, v);
            }
            self.n_moved += 1;
            return;
        }

        // if all data out of main and into grow
        // then grow is main
        std::mem::swap(&mut self.main, &mut self.grow);
        self.n_moved = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_right_values() {
        // cargo test test_get_right_values -- --nocapture
        // cargo test test_get_right_values -- --show-output
        let mut hm = HMap::new();
        hm.insert("james".to_string(), 18);
        hm.insert("dave".to_string(), 45);
        hm.insert("andy".to_string(), 23);
        hm.insert("pete".to_string(), 14);
        hm.insert("steve".to_string(), 90);
        hm.insert("jane".to_string(), 105);
        hm.insert("grander".to_string(), 23);
        hm.insert("irene".to_string(), 65);
        hm.insert("sam".to_string(), 66);
        hm.insert("andrex".to_string(), 77);
        hm.insert("andrew".to_string(), 89);
        hm.insert("geralt".to_string(), 99);
        // repeat
        hm.insert("dave".to_string(), 83);

        assert_eq!(hm.get("geralt"), Some(&99));
        assert_eq!(hm.get("sam"), Some(&66));
        assert_eq!(hm.get("dave"), Some(&83));

        assert_eq!(hm.len(), 12);

        println!("hm: {:?}", hm);
    }

    #[test]
    fn test_lots_of_numbers() {
        // cargo test test_lots_of_numbers -- --nocapture
        let mut hm = HMap::new();
        for x in 0..10_000 {
            hm.insert(x, x + 250);
        }

        assert_eq!(hm.len(), 10_000);
        assert_eq!(hm.get(&500), Some(&750));

        for (n, x) in hm.main.buckets.iter().enumerate() {
            assert!(
                x.len() < 10,
                format!("main buckets too big {}:{}", n, x.len())
            )
        }

        for (n, x) in hm.grow.buckets.iter().enumerate() {
            assert!(
                x.len() < 10,
                format!("grow buckets too big {}:{}", n, x.len())
            )
        }
    }
}
