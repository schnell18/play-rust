extern crate crypto;
extern crate rand;
extern crate rustc_serialize as serialize;

// AES128 CBC、encrypt decrypt demo
use std::str;
use std::env;
use std::process;
use crypto::{symmetriccipher,buffer,aes,blockmodes};
use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use serialize::base64::{STANDARD, ToBase64, FromBase64};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("USAGE: {} key text1 text2 ...", args[0]);
        process::exit(1);
    }

    let key = &args[1];
    if args[0].contains("aes-decrypt") {
        for msg in args.iter().skip(2) {
            println!("{}", decrypt(msg, key));
        }
    }
    else {
        for msg in args.iter().skip(2) {
            println!("{}", encrypt(msg, key));
        }
    }
}

fn encrypt(message: &str, key: &str) -> String {
    let iv:[u8;16] = *b"aabbccddeeffgghh";
    let key = key.as_bytes();

    let encrypted_data = aes128_cbc_encrypt(
        message.as_bytes(), &key, &iv).ok().unwrap();

    format!("sea{}",  encrypted_data.to_base64(STANDARD))
}

fn decrypt(message: &str, key: &str) -> String {
    let iv:[u8;16] = *b"aabbccddeeffgghh";
    let key = key.as_bytes();
    let encrypted_data = &(message[3..].from_base64().unwrap());

    let decrypted_data = aes128_cbc_decrypt(
        encrypted_data, &key, &iv).ok().unwrap();

    String::from_utf8(decrypted_data).unwrap()
}

// Encrypt a buffer with the given key and iv using AES-128/CBC/Pkcs
// encryption.
fn aes128_cbc_encrypt(data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
    let mut encryptor=aes::cbc_encryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0;4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(
            &mut read_buffer,&mut write_buffer,true)?;

        final_result.extend(
            write_buffer
            .take_read_buffer()
            .take_remaining()
            .iter()
            .map(|&i| i)
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {},
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using AES-128/CBC/Pkcs
// encryption.
fn aes128_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(
            &mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
            .take_read_buffer()
            .take_remaining()
            .iter()
            .map(|&i| i)
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

#[test]
fn test_encrypt() {
    let key = "aaaaaaaaaaaaaaaa";
    let cipher = "seaLqseBJjOz/x7PnitA2Ezbg==";
    let msg = "hello";
    assert_eq!(encrypt(msg, key), cipher);
}

#[test]
fn test_decrypt() {
    let key = "aaaaaaaaaaaaaaaa";
    let cipher = "seaLqseBJjOz/x7PnitA2Ezbg==";
    let msg = "hello";
    assert_eq!(decrypt(cipher, key), msg);
}
