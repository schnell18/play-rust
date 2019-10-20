extern crate crypto;
extern crate rustc_serialize as serialize;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::aes::{self, KeySize};
use serialize::base64::{STANDARD, ToBase64};
use std::iter::repeat;
use rand::{thread_rng, Rng};

fn main() {
    let input = "Hello world!";
    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
    let mut bytes: Vec<u8> = repeat(0u8).take(sha.output_bytes()).collect();
    sha.result(&mut bytes);
    println!("{}", bytes.to_base64(STANDARD));
    println!("{}", Vec::from(input).to_base64(STANDARD));

    let mut rng = thread_rng();
    let mut key = [0u8, 16];
    rng.fill(&mut key);
    // let mut nonce: Vec<u8> = repeat(0u8).take(16).collect();
    let mut nonce = [0u8, 16];
    rng.fill(&mut nonce);
    println!("Key: {}", key.to_base64(STANDARD));
    println!("Nonce: {}", nonce.to_base64(STANDARD));
    let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
    let secret = "I like Nickelback";
    let mut output: Vec<u8> = repeat(0u8).take(secret.len()).collect();
    cipher.process(secret.as_bytes(), &mut output[..]);
    println!("Ciphertext: {}", output.to_base64(STANDARD));
}
