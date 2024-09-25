use aes::{
    cipher::{
        block_padding::{Pkcs7, UnpadError},
        BlockDecryptMut, BlockEncryptMut, KeyInit,
    },
    Aes256Dec, Aes256Enc,
};
use ecb;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("failed to decrypt")]
#[diagnostic(
    code(gd::crypto::aes::decrypt),
    help("make sure the data is padded correctly")
)]
pub struct Error(#[from] pub UnpadError);

pub type Decryptor = ecb::Decryptor<Aes256Dec>;
pub type Encryptor = ecb::Encryptor<Aes256Enc>;

pub const KEY_LENGTH: usize = 32;

pub const KEY: [u8; KEY_LENGTH] = [
    0x69, 0x70, 0x75, 0x39, 0x54, 0x55, 0x76, 0x35, 0x34, 0x79, 0x76, 0x5d, 0x69, 0x73, 0x46, 0x4d,
    0x68, 0x35, 0x40, 0x3b, 0x74, 0x2e, 0x35, 0x77, 0x33, 0x34, 0x45, 0x32, 0x52, 0x79, 0x40, 0x7b,
];

pub fn decryptor<K: AsRef<[u8]>>(key: K) -> Decryptor {
    Decryptor::new(key.as_ref().into())
}

pub fn encryptor<K: AsRef<[u8]>>(key: K) -> Encryptor {
    Encryptor::new(key.as_ref().into())
}

pub fn decrypt<D: AsRef<[u8]>>(data: D) -> Result<Vec<u8>, Error> {
    fn decrypt_inner(data: &[u8]) -> Result<Vec<u8>, Error> {
        decryptor(KEY)
            .decrypt_padded_vec_mut::<Pkcs7>(data)
            .map_err(Error)
    }

    decrypt_inner(data.as_ref())
}

pub fn encrypt<D: AsRef<[u8]>>(data: D) -> Vec<u8> {
    fn encrypt_inner(data: &[u8]) -> Vec<u8> {
        encryptor(KEY).encrypt_padded_vec_mut::<Pkcs7>(data)
    }

    encrypt_inner(data.as_ref())
}
