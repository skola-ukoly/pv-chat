use cipher::*;
use std::{fs::File, io::{Write, Read} };

#[test]
fn key_retrieval_test() {
    let mut buf = vec![0u8; 256];
    let mut one_time_pad = File::open("./OneTimePad.dat").expect("one time pad");

    one_time_pad.read(&mut buf).unwrap();

    let key1 = buf[0..128].to_vec();
    let key2 = buf[128..256].to_vec();

    let mut cipher = Cipher::new(one_time_pad);

    assert_eq!(&cipher.get_key().unwrap(), &key1[..128]);
    assert_eq!(&cipher.get_key().unwrap(), &key2[..128]);
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