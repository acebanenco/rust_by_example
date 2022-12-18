extern crate core;

use std::ffi::c_void;
use std::{ptr, slice};

use jni::objects::{JByteBuffer, JClass};
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use openssl::hash::MessageDigest;
use openssl_sys::{EVP_MD, EVP_MD_CTX};

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_NativeMessageDigestSha256Impl_init(
    _env: JNIEnv,
    _cls: JClass,
    md_long: jlong,
) -> jlong {
    let ctx = ctx_new();

    let md_ptr: *const EVP_MD = md_long as *const EVP_MD;
    digest_init_ex(md_ptr, ctx);
    ctx as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_NativeMessageDigestSha256Impl_md_1init(
    _env: JNIEnv,
    _cls: JClass,
) -> jlong {
    let md: MessageDigest = MessageDigest::sha256();
    let md_ptr: *const EVP_MD = md.as_ptr();
    md_ptr as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_NativeMessageDigestSha256Impl_digest_1multi(
    env: JNIEnv,
    _cls: JClass,
    md_long: jlong,
    ctx_long: jlong,
    msg_buf: JByteBuffer,
    msg_len: jint,
    dgst_buf: JByteBuffer,
    len: jint,
) {
    let md_ptr: *const EVP_MD = md_long as *const EVP_MD;
    let ctx: *mut EVP_MD_CTX = ctx_long as *mut EVP_MD_CTX;

    let batch_size: usize = len as usize;
    let msg_size: usize = msg_len as usize;
    let digest_size: usize = 32;

    let msg_ptr: *const u8 = env.get_direct_buffer_address(msg_buf).unwrap() as *const u8;
    let digest_ptr: *mut u8 = env.get_direct_buffer_address(dgst_buf).unwrap();

    digest_multi(
        md_ptr,
        ctx,
        msg_ptr,
        msg_size,
        digest_ptr,
        digest_size,
        batch_size,
    );
}

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_NativeMessageDigestSha256Impl_free(
    _env: JNIEnv,
    _cls: JClass,
    ctx_long: jlong,
) {
    let ctx: *mut EVP_MD_CTX = ctx_long as *mut EVP_MD_CTX;
    md_ctx_free(ctx);
}

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_NativeMessageDigestSha256Impl_md_1free(
    _env: JNIEnv,
    _cls: JClass,
    md_long: jlong,
) {
    let md_ptr: *mut EVP_MD = md_long as *mut EVP_MD;
    md_free(md_ptr);
}

pub fn digest_multi(
    md_ptr: *const EVP_MD,
    ctx: *mut EVP_MD_CTX,
    msg_ptr: *const u8,
    msg_size: usize,
    digest_ptr: *mut u8,
    digest_size: usize,
    batch_size: usize,
) {
    let msg_bytes: &[u8] = unsafe { slice::from_raw_parts(msg_ptr, batch_size * msg_size) };
    let digest_bytes: &mut [u8] =
        unsafe { slice::from_raw_parts_mut(digest_ptr, batch_size * digest_size) };

    for index in 0..batch_size {
        if index % 1000 == 0 {
            println!("Calclated {} digests", index);
        }

        let from: usize = index * msg_size;
        let chunk_bytes: &[u8] = &msg_bytes[from..from + msg_size];
        let data = chunk_bytes.as_ptr() as *mut _;

        digest_update(ctx, msg_size, data);

        let from: usize = index * digest_size;
        let digest: *mut u8 = digest_bytes[from..from + digest_size].as_mut_ptr();
        digest_final(ctx, digest);

        digest_init_ex(md_ptr, ctx);
    }
}

pub fn digest_update(ctx: *mut EVP_MD_CTX, msg_size: usize, data: *const c_void) {
    unsafe {
        // TODO check result
        openssl_sys::EVP_DigestUpdate(ctx, data, msg_size);
    }
}

pub fn digest_final(ctx: *mut EVP_MD_CTX, digest: *mut u8) {
    unsafe {
        // TODO check result
        openssl_sys::EVP_DigestFinal(ctx, digest, ptr::null_mut());
    }
}

pub fn md_ctx_free(ctx: *mut EVP_MD_CTX) {
    unsafe {
        openssl_sys::EVP_MD_CTX_free(ctx);
    }
}

pub fn md_free(md_ptr: *mut EVP_MD) {
    unsafe {
        openssl_sys::EVP_MD_free(md_ptr);
    }
}

pub fn digest_init_ex(md_ptr: *const EVP_MD, ctx: *mut EVP_MD_CTX) {
    unsafe {
        // TODO check result
        openssl_sys::EVP_DigestInit_ex(ctx, md_ptr, ptr::null_mut());
    }
}

pub extern "C" fn ctx_new() -> *mut EVP_MD_CTX {
    let ctx: *mut EVP_MD_CTX = unsafe { openssl_sys::EVP_MD_CTX_new() };
    ctx
}
