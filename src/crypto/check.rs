use crate::crypto::{hash, robtop};

pub fn of<D: AsRef<[u8]>, K: AsRef<[u8]>>(data: D, key: K) -> String {
    fn of_inner(data: &[u8], key: &[u8]) -> String {
        robtop::encode(hash::sha1(data), key)
    }

    of_inner(data.as_ref(), key.as_ref())
}

pub fn of_with_salt<D: AsRef<[u8]>, S: AsRef<[u8]>, K: AsRef<[u8]>>(
    data: D,
    salt: S,
    key: K,
) -> String {
    fn of_with_salt_inner(data: &[u8], salt: &[u8], key: &[u8]) -> String {
        robtop::encode(hash::sha1_with_salt(data, salt), key)
    }

    of_with_salt_inner(data.as_ref(), salt.as_ref(), key.as_ref())
}
