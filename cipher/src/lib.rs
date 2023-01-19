mod error;

use error::*;

use std::{
    fs::File,
    io::{Read, Seek},
};

// lenght of every message and key in bytes
const MESSAGE_LEN: usize = 64;



/// Cipher can encrypt and decrypt fixed length byte arrays
///
/// Cipher holds one time pad file with the encryption/decryption key
/// and a pointer to current location in this key
/// 
/// One time pad should be as long as possible random bytes file
/// 
/// The same key is needed for Encryption and Decryption 
///
/// # Examples
///
/// ```no_run
/// use crate::cipher::*;
/// use std::{fs::File, io::Write};
/// 
/// let mut cipher = Cipher::new(File::open("./OneTimePad.dat").expect("one time pad not found"));
///    
/// let mut buf = vec![0u8; 128];
/// let msg = b"Hello world";
///
/// buf.write(msg).unwrap();
///
/// let plaintext = buf.clone();
///
/// cipher.encrypt(&mut buf[..128]).unwrap();
///
/// assert_ne!(&plaintext, &buf); 
/// ```
#[derive(Debug)]
pub struct Cipher {
    one_time_pad: File,
    pointer: usize,
}

impl Cipher {

    /// Creates a cipher with reference to One time pad
    /// 
    /// Cipher can encrypt and decrypt fixed length byte arrays
    ///
    /// Cipher holds one time pad file with the encryption/decryption key
    /// and a pointer to current location in this key
    /// 
    /// One time pad should be as long as possible random bytes file
    /// 
    /// The same key is needed for Encryption and Decryption 
    /// ```no_run
    /// let path = "./one_time_pad.dat"
    /// let one_time_pad = File::open(path).unwrap();
    ///
    /// let cipher = Cipher::new(one_time_pad);
    /// ```
    pub fn new(one_time_pad: File) -> Self {
        Self {
            one_time_pad,
            pointer: 0,
        }
    }

    /// Creates a cipher with reference to one time pad and already set pointer. 
    /// 
    /// Cipher holds one time pad file with the encryption/decryption key
    /// and a pointer to current location in this key
    /// 
    /// One time pad should be as long as possible random bytes file
    /// 
    /// The same key is needed for Encryption and Decryption 
    pub fn from(one_time_pad: File, pointer: usize) -> Self {
        Self {
            one_time_pad,
            pointer,
        }
    }


    /// Returns a 64 bytes of cipher key from one time pad and
    /// moves the pointer forwards that amount
    pub fn get_key(&mut self) -> Result<[u8; MESSAGE_LEN]> {
        let mut key = [0u8; MESSAGE_LEN];
        self.one_time_pad.seek(std::io::SeekFrom::Start(self.pointer as u64))?;
        let bytes_read = self.one_time_pad.read(&mut key)?;

        if bytes_read != MESSAGE_LEN {
            return Err(CipherError::InvalidKeyLen);
        };

        self.pointer += MESSAGE_LEN;
            
        Ok(key)
    }
    
    /// Moves the pointer 64 bytes forward
    /// 
    /// Useful when connecting to ongoing chat 
    pub fn move_forward(&mut self) {
        self.pointer += MESSAGE_LEN;
    }


    /// Encrypts the provided buffer with 64 bytes of key from one time pad provided at initialization
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use crate::cipher::*;
    /// use std::{fs::File, io::Write};
    /// 
    /// let mut cipher = Cipher::new(File::open("./OneTimePad.dat").expect("one time pad not found"));
    ///    
    /// let mut buf = vec![0u8; 128];
    /// let msg = b"Hello world";
    ///
    /// buf.write(msg).unwrap();
    ///
    /// let plaintext = buf.clone();
    ///
    /// cipher.encrypt(&mut buf[..128]).unwrap();
    ///
    /// assert_ne!(&plaintext, &buf); 
    /// ```
    pub fn encrypt(&mut self, buf: &mut [u8]) -> Result<()> {
        if buf.len() != MESSAGE_LEN {
            return Err(CipherError::InvalidMessageLen);
        };

        let key = self.get_key()?;

        for i in 0..buf.len() {
            let plaintext_byte = buf[i];
            let key_byte = key[i];

            buf[i] = plaintext_byte ^ key_byte;
        };
        
        Ok(())
    }

    /// Decrypts a buffer with 64 bytes of cipherkey
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::cipher::*;
    /// use std::{fs::File, io::Write};
    /// 
    /// let mut cipher = Cipher::new(File::open("./OneTimePad.dat").expect("one time pad not found"));
    ///    
    /// let mut buf = vec![0u8; 128];
    /// let msg = b"Hello world";
    ///
    /// buf.write(msg).unwrap();
    ///
    /// let plaintext = buf.clone();
    ///
    /// cipher.encrypt(&mut buf[..128]).unwrap();
    ///
    /// assert_ne!(&plaintext, &buf); 
    /// ```
    pub fn decrypt(&mut self, buf: &mut [u8]) -> Result<()> {
        if buf.len() != MESSAGE_LEN {
            return Err(CipherError::InvalidMessageLen);
        };

        let key = self.get_key()?;

        for i in 0..buf.len() {
            let plaintext_byte = buf[i];
            let key_byte = key[i];

            buf[i] = plaintext_byte ^ key_byte;
        };

        Ok(())
    }
}
