use hex;
use sha2::{Digest, Sha256};

fn main() {
    let data = b"Hey";
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    println!("Sha256 {} ", hex::encode(result));
}
