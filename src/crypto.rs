pub mod sodium;

use nanos_sdk::bindings::*;
use nanos_sdk::ecc::Ed25519;
use nanos_sdk::io::{StatusWords, SyscallError};

use crate::transaction::account::PublicKeyAccount;

const PATH_BYTES_LENGTH: usize = 20;
const PATH_PREFIX: u32 = 0x8000002C;

/// Helper function that retrieves public key
pub fn get_pubkey() -> Result<PublicKeyAccount, SyscallError> {
    let private_key = Ed25519::new();

    match private_key.public_key() {
        Ok(public_key) => Ok(PublicKeyAccount::from_ed25519(public_key.as_ref())),
        Err(_) => Err(SyscallError::Unspecified),
    }
}

/// Helper function that converts the derivation path received
#[allow(clippy::needless_range_loop)]
pub fn get_derivation_path(buf: &mut &[u8]) -> Result<[u32; 5], StatusWords> {
    let mut path = [0u32; 5];

    match buf.len() {
        PATH_BYTES_LENGTH => {
            for i in 0..4 {
                let (int_bytes, rest) = buf.split_at(4);
                *buf = rest;
                path[i] = u32::from_be_bytes(int_bytes.try_into().unwrap());
            }

            match path[0] {
                PATH_PREFIX => Ok(path),
                _ => Err(StatusWords::Unknown),
            }
        }
        _ => Err(StatusWords::BadLen),
    }
}

pub fn secure_hash(msg: &mut [u8], msg_len: u32, hash: &mut [u8; 32]) {
    blake2b_256(msg, msg_len, hash);
    keccak_256(&mut hash.clone(), 32, hash);
}

fn blake2b_256(msg: &mut [u8], msg_len: u32, out: &mut [u8]) {
    let mut ctx: cx_blake2b_t = cx_blake2b_s::default();

    unsafe {
        cx_blake2b_init_no_throw(&mut ctx, 256);

        cx_hash_no_throw(
            &mut ctx.header,
            CX_LAST,
            msg.as_mut_ptr(),
            msg_len,
            out.as_mut_ptr(),
            32,
        );
    }
}

fn keccak_256(msg: &mut [u8], msg_len: u32, out: &mut [u8]) {
    let mut ctx: cx_sha3_t = cx_sha3_s::default();

    unsafe {
        cx_keccak_init_no_throw(&mut ctx, 256);

        cx_hash_no_throw(
            &mut ctx.header,
            CX_LAST,
            msg.as_mut_ptr(),
            msg_len,
            out.as_mut_ptr(),
            32,
        );
    }
}
