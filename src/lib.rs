//! A more efficient, no_std SHA-256 for the Solana SVM.
//!
//! On `target_os = "solana"`, hashing routes through the `sol_sha256`
//! syscall. Off-Solana, it falls through to the `sha2` crate so the same
//! APIs work in host code (tests, off-chain tooling). The Solana
//! implementation costs ~100 CUs for `hashv(&[b"test"])`, vs ~120 CUs for
//! `solana_program::hash::hashv`.
#![no_std]

use core::mem::MaybeUninit;

#[cfg(not(target_os = "solana"))]
use sha2::{Digest, Sha256};

/// Length of a SHA-256 digest, in bytes.
pub const HASH_LENGTH: usize = 32;

#[cfg(target_os = "solana")]
unsafe extern "C" {
    fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

/// Hash a single byte slice and return the digest by value.
#[cfg_attr(target_os = "solana", inline(always))]
pub fn hash(data: &[u8]) -> [u8; HASH_LENGTH] {
    hashv(&[data])
}

/// Hash any `T: AsRef<[u8]>` (e.g. `&str`, `&[u8; N]`) and return the digest.
#[inline(always)]
pub fn hash_ref<T: AsRef<[u8]>>(data: T) -> [u8; HASH_LENGTH] {
    hashv(&[data.as_ref()])
}

/// Hash a sequence of byte slices as if they were concatenated, and return
/// the digest. Cheaper than concatenating the inputs yourself.
#[cfg_attr(target_os = "solana", inline(always))]
pub fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH] {
    let mut out = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();
    unsafe {
        hash_into(data, out.assume_init_mut());
        out.assume_init()
    }
}

/// Hash `data` directly into the provided 32-byte buffer.
///
/// Use this when you want the digest written into pre-existing storage
/// (e.g. a struct field) without an intermediate move.
#[cfg(not(target_os = "solana"))]
pub fn hash_into(data: &[&[u8]], out: &mut [u8; HASH_LENGTH]) {
    let mut hasher = Sha256::new();
    for item in data {
        hasher.update(item);
    }
    hasher.finalize_into(out.into());
}

#[cfg(target_os = "solana")]
#[inline(always)]
pub fn hash_into(data: &[&[u8]], out: &mut [u8; HASH_LENGTH]) {
    unsafe {
        sol_sha256(
            data as *const _ as *const u8,
            data.len() as u64,
            out.as_mut_ptr(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_hash() {
        let h = hash_ref("test");
        let h2 = hashv(&[b"test".as_ref()]);
        assert_eq!(h, h2);
        assert_eq!(
            h2,
            [
                0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65, 0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a,
                0xd0, 0x15, 0xa3, 0xbf, 0x4f, 0x1b, 0x2b, 0x0b, 0x82, 0x2c, 0xd1, 0x5d, 0x6c, 0x15,
                0xb0, 0xf0, 0x0a, 0x08
            ]
        );
    }
}
