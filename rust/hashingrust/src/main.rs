use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn main() {
    // secret key and message
    let secret_key = b"hey-there_Dicks";
    let message = b"Solana feels hotter";

    // hmac instance
    let mut mac =
        HmacSha256::new_from_slice(secret_key).expect("Hmac can take any key of any size");

    // mesage to update
    mac.update(message);

    // finalise the hmac computation to get the authentication tag
    let tag = mac.finalize().into_bytes();

    println!("Senders Side:");
    println!("Message :'{}'", String::from_utf8_lossy(message));
    // tag is just bytes so we print in hexadecimel format
    println!("Geenerated Hmac :{:x}\n", tag);

    // reciever side

    println!("Rechievers Side:");
    println!(" Rechiever Message :'{}'", String::from_utf8_lossy(message));
    // tag is just bytes so we print in hexadecimel format
    println!("Generated Hmac :{:x}\n", tag);

    let mut verifier = HmacSha256::new_from_slice(secret_key).expect("Hmac can be of any size");

    verifier.update(message);

    // match if the tag recieved is same
    match verifier.verify_slice(&tag) {
        Ok(_) => println!("\n 💦💦💦💦Verification successfull:message is authetic"),
        Err(_) => println!("\n Verification Failed:Do not trust"),
    }

    // --- Let's demonstrate a failed verification ---
    println!("\n--- Simulating a tampered message ---");
    let tampered_message = b"This is a message that has been tampered with.";
    let mut failed_verifier = HmacSha256::new_from_slice(secret_key).unwrap();
    failed_verifier.update(tampered_message);

    match failed_verifier.verify_slice(&tag) {
        Ok(_) => println!("This should not happen!"),
        Err(_) => println!(
            "❌ Verification FAILED as expected. The tampered message was correctly rejected."
        ),
    }
}
