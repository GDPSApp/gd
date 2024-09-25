use sha1::{Digest, Sha1};

macro_rules! hex {
    ($item: expr) => {
        format!("{:x}", $item)
    };
}

pub fn sha1<D: AsRef<[u8]>>(data: D) -> String {
    hex!(Sha1::digest(data))
}

pub fn sha1_with_salt<D: AsRef<[u8]>, S: AsRef<[u8]>>(data: D, salt: S) -> String {
    let mut hasher = Sha1::new();

    hasher.update(data.as_ref());
    hasher.update(salt.as_ref());

    hex!(hasher.finalize())
}
