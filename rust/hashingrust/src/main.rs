
use sha2::{Sha256,Digest}
use hex;

fn main(){
    let data  = b"Hello Harish how about learning solana";
    let mut hasher = Sha256::new();
    hasher.update(data);

    let result = hasher.finalize();

    println!("Sha256:{}", hex::encode(result));
}