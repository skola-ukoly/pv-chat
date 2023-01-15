use cipher::*;
use std::{fs::File, io::Write};

#[test]
fn key_retrieval_test() {
    let key1: [u8; 128] = [100, 51, 32, 56, 100, 32, 102, 55, 32, 55, 55, 32, 99, 57, 32, 53, 48, 32, 57, 56, 32, 100, 97, 32, 55, 49, 32, 53, 54, 32, 56, 48, 32, 48, 99, 32, 97, 99, 32, 57, 54, 32, 102, 101, 32, 57, 101, 32, 13, 10, 54, 56, 32, 53, 101, 32, 48, 101, 32, 97, 100, 32, 57, 100, 32, 54, 55, 32, 52, 53, 32, 102, 56, 32, 56, 97, 32, 55, 55, 32, 50, 57, 32, 56, 101, 32, 50, 56, 32, 98, 55, 32, 52, 101, 32, 48, 52, 32, 13, 10, 50, 97, 32, 50, 101, 32, 53, 98, 32, 97, 55, 32, 101, 51, 32, 102, 48, 32, 53, 57, 32, 50, 102, 32, 49, 101, 32, 99];
    let key2 : [u8; 128] = [49, 32, 51, 57, 32, 56, 101, 32, 99, 49, 32, 54, 56, 32, 97, 100, 32, 57, 102, 32, 13, 10, 97, 102, 32, 51, 51, 32, 98, 54, 32, 48, 49, 32, 57, 57, 32, 101, 53, 32, 57, 57, 32, 101, 100, 32, 56, 52, 32, 53, 48, 32, 52, 53, 32, 54, 56, 32, 54, 50, 32, 99, 48, 32, 98, 99, 32, 55, 56, 32, 13, 10, 56, 57, 32, 57, 57, 32, 55, 53, 32, 52, 51, 32, 55, 52, 32, 99, 50, 32, 56, 101, 32, 56, 101, 32, 50, 56, 32, 53, 49, 32, 54, 48, 32, 57, 102, 32, 101, 55, 32, 55, 56, 32, 99, 54, 32, 50, 57, 32, 13, 10, 54, 97, 32, 102, 56, 32];
    let mut cipher = Cipher::new(File::open("./OneTimePad.dat").expect("one time pad"));

    assert_eq!(&cipher.get_key().unwrap(), &key1);
    assert_eq!(&cipher.get_key().unwrap(), &key2);
}

#[test]
fn encrypt_test() {
    let mut cipher = Cipher::new(File::open("./OneTimePad.dat").expect("one time pad not found"));

    let mut buf = [0u8; 128];
    let msg = b"Hello world";

    buf.as_mut_slice().write(msg).unwrap();
    let plaintext = buf.clone();

    cipher.encrypt(&mut buf).unwrap();

    assert_ne!(&plaintext, &buf); 
}

#[test]
fn encrypt_decrypt_test() {
    let mut c1 = Cipher::new(File::open("./OneTimePad.dat").expect("one time pad not found"));
    let mut c2 = Cipher::new(File::open("./OneTimePad.dat").expect("one time pad not found"));

    let mut buf = [0u8; 128];
    let msg = b"Hello world";

    buf.as_mut_slice().write(msg).unwrap();
    let plaintext = buf.clone();

    c1.encrypt(&mut buf).unwrap();
    let ciphertext1 = buf.clone();
    c2.decrypt(&mut buf).unwrap();

    assert_eq!(&plaintext, &buf);

    c1.encrypt(&mut buf).unwrap();
    let ciphertext2 = buf.clone();
    c2.decrypt(&mut buf).unwrap();

    assert_eq!(&plaintext, &buf);
    assert_ne!(ciphertext1, ciphertext2);
}