use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};
use rand::RngCore;

fn main() {
    //generate a random 32 byte key
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);

    // generate a 12 byte nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plain_text = b"Hello i love pussy";
    println!("PlainText:{:?}", String::from_utf8_lossy(plain_text));

    //encrypt
    let cipher_text = cipher
        .encrypt(nonce, plain_text.as_ref())
        .expect("encryption Failure");
    println!("Ciphertext:{:?}", cipher_text);

    //encrypt
    let decrypted = cipher
        .decrypt(nonce, cipher_text.as_ref())
        .expect("decryption Failure");
    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted));
}
