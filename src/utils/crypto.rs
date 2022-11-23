use nanos_sdk::bindings::*;

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

pub fn secure_hash(msg: &mut [u8], msg_len: u32, hash: &mut [u8; 32]) {
    blake2b_256(msg, msg_len, hash);
    keccak_256(&mut hash.clone(), 32, hash);
}
