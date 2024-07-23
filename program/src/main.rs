#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};

pub fn main() {
    let mut hodgepodge = [0u8; 32];

    let hash_count: u32 = sp1_zkvm::io::read();
    for _ in 0..hash_count {
        let mut h = Sha256::new();
        h.update(sp1_zkvm::io::read_vec());
        let hash = h.finalize().to_vec();
        for i in 0..32 {
            hodgepodge[i] += hash[i];
        }
    }

    sp1_zkvm::io::commit_slice(&hodgepodge);
}
