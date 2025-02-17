use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use crate::blob::{read_u64, write_u64, Blob};
use crate::error::BlobError;

// size of the initial data
const CONTROL_DATA_SIZE: u64 = 32;

/// This blob store will act as one half of the hashmap
/// as with the hashmap wrap this in something to make growing work
pub struct BlobStore {
    file: File,
    hseed: u64,
    block_size: u64,
    nblocks: u64,
    elems: u64,
}

impl BlobStore {
    pub fn new(fname: &str, block_size: u64, nblocks: u64) -> Result<Self, BlobError> {
        let hseed = rand::random::<u64>();
        // create file
        let mut ff = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(fname)?;

        let f = &mut ff;
        f.set_len(CONTROL_DATA_SIZE + block_size * nblocks)?;
        f.seek(SeekFrom::Start(0))?;
        write_u64(f, hseed)?;
        write_u64(f, block_size)?;
        write_u64(f, nblocks)?;
        // zero elements in new store
        write_u64(f, 0)?;

        // mark beginnings of each block to show empty
        for x in 0..nblocks {
            f.seek(SeekFrom::Start(CONTROL_DATA_SIZE + x * block_size))?;
            // Key length of 0 means no item
            write_u64(f, 0)?;
            write_u64(f, block_size - 16)?;
        }

        Ok(BlobStore {
            hseed,
            file: ff,
            block_size,
            nblocks,
            elems: 0,
        })
    }

    pub fn open(fname: &str) -> Result<Self, BlobError> {
        let mut ff = OpenOptions::new().write(true).read(true).open(fname)?;

        let f = &mut ff;
        f.seek(SeekFrom::Start(0))?;

        let hseed = read_u64(f)?;
        let block_size = read_u64(f)?;
        let nblocks = read_u64(f)?;
        let elems = read_u64(f)?;

        Ok(BlobStore {
            hseed,
            file: ff,
            block_size,
            nblocks,
            elems,
        })
    }

    pub fn new_or_open(fname: &str, bsize: u64, nblocks: u64) -> Result<Self, BlobError> {
        Self::new(fname, bsize, nblocks).or_else(|_| Self::open(fname))
    }

    pub fn inc_elems(&mut self, n: i32) -> Result<(), BlobError> {
        if n > 0 {
            self.elems += n as u64;
        } else {
            let n2 = (-n) as u64;
            if self.elems > n2 {
                self.elems -= n2;
            }
        }

        self.file.seek(SeekFrom::Start(24))?;
        write_u64(&mut self.file, self.elems)?;

        Ok(())
    }

    pub fn insert<K: Serialize, V: Serialize>(&mut self, k: K, v: V) -> Result<(), BlobError> {
        self.remove(&k).ok();
        self.insert_only(k, v)
    }

    fn insert_only<K: Serialize, V: Serialize>(&mut self, k: K, v: V) -> Result<(), BlobError> {
        let blob = Blob::from(&k, &v)?;
        if blob.len() > self.block_size {
            // Let the wrapper make a file with a bigger group
            return Err(BlobError::TooBig(blob.len()));
        }
        let bucket = blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(
            CONTROL_DATA_SIZE + self.block_size * bucket,
        ))?;

        // start each loop at the beginning of an elem
        // remember klen == 0 means an empty section
        loop {
            if pos > CONTROL_DATA_SIZE + self.block_size * (bucket + 1) {
                // reached end of the data block
                // consider other handlings but this will tell the
                // wrapper to make space
                // another option is to overflow onto the end of the file
                return Err(BlobError::NoRoom);
            }
            let klen = read_u64(f)?;
            let vlen = read_u64(f)?;
            if klen == 0 && blob.len() < vlen {
                f.seek(SeekFrom::Start(pos))?;
                blob.out(f)?;
                // add pointer immediately after data ends
                write_u64(f, 0)?;
                write_u64(f, (vlen - blob.len()) - 16)?;
                self.inc_elems(1)?;
                return Ok(());
            }
            pos = f.seek(SeekFrom::Start(pos + 16 + klen + vlen))?;
        }
    }

    pub fn b_start(&self, b: u64) -> u64 {
        CONTROL_DATA_SIZE + self.block_size * b
    }

    pub fn get<K: Serialize>(&mut self, k: &K) -> Result<Blob, BlobError> {
        let s_blob = Blob::from(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let b_start = self.b_start(bucket);
        let b_end = self.b_start(bucket + 1);
        let f = &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(b_start))?;
        loop {
            if pos >= b_end {
                // Result<Option> is also possible instead of
                return Err(BlobError::NotFound);
            }

            // for very large blobs optimize by reading until the key vs the whole blob
            let b = Blob::read(f)?;
            if b.key_match(&s_blob) {
                return Ok(b);
            }
            pos += b.len();
        }
    }

    pub fn remove<K: Serialize>(&mut self, k: &K) -> Result<(), BlobError> {
        let s_blob = Blob::from(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let b_start = self.b_start(bucket);
        let b_end = self.b_start(bucket + 1);
        let f = &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(b_start))?;
        loop {
            if pos >= b_end {
                return Ok(());
            }
            let b = Blob::read(f)?;
            if b.key_match(&s_blob) {
                // item found
                let l = b.len();
                // if next block is empty, we should join the two blocks
                if pos + l < b_end {
                    if read_u64(f)? == 0 {
                        let nlen = read_u64(f)?;
                        f.seek(SeekFrom::Start(pos))?;
                        write_u64(f, 0)?;
                        write_u64(f, l + nlen + 16)?;

                        return Ok(());
                    }
                }
                // otherwise, just leave empty
                f.seek(SeekFrom::Start(pos))?;
                write_u64(f, 0)?;
                write_u64(f, l - 16)?;
                self.inc_elems(-1)?;

                return Ok(());
            }
            pos = f.seek(SeekFrom::Start(pos + b.len()))?;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_create_file() {
        let fs = "test-data/create-file";
        std::fs::remove_file(fs).ok();
        let mut bs = BlobStore::new(fs, 1_000, 10).unwrap();
        let block_size = bs.block_size;

        let mut b2 = BlobStore::open(fs).unwrap();
        assert_eq!(b2.block_size, block_size);

        b2.insert_only("fish", "so long and thanks for all the fish")
            .unwrap();
        b2.insert_only(23, "a big number for small counters")
            .unwrap();
        b2.insert_only("green", "is a color I guess").unwrap();
        b2.insert_only("happy", "is friends with sleepy").unwrap();

        drop(b2);

        let mut b3 = BlobStore::open(fs).unwrap();
        assert_eq!(
            b3.get(&"green").unwrap().get_v::<String>().unwrap(),
            "is a color I guess".to_string()
        );

        b3.remove(&"green").ok();
        assert!(b3.get(&"green").is_err());
        assert!(b3.get(&"fish").is_ok());
    }
}
