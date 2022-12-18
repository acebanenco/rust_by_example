extern crate core;

use std::slice;
use sha2::{Digest, Sha256};

use jni::objects::{JByteBuffer, JClass};
use jni::sys::{jint, jlong};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_NativeMessageDigestSha256Impl_sha256_1hasher_1init
(_env: JNIEnv, _cls: JClass) -> jlong {
    let hasher: Sha256 = Sha256::new();
    let wrapper: Box<Sha256> = Box::from(hasher);
    let raw = Box::into_raw(wrapper);
    raw as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_NativeMessageDigestSha256Impl_free
(_env: JNIEnv, _cls: JClass, hasher_addr: jlong) {
    let hasher_ptr = hasher_addr as *mut Sha256;
    //let hasher: &mut Sha256 = hasher_ptr;
    let _x = unsafe {
        Box::from_raw(hasher_ptr)
    };
}

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_NativeMessageDigestSha256Impl_digestAll
(env: JNIEnv,
 _cls: JClass,
 hasher_addr: jlong,
 input_chunks: JByteBuffer,
 input_chunk_size: jint,
 output_chunks: JByteBuffer,
 number_of_chunks: jint)
{
    let hasher_ptr = hasher_addr as *mut Sha256;
    let mut wrapper = unsafe {
        Box::from_raw(hasher_ptr)
    };
    let hasher = wrapper.as_mut();

    let messages: *const u8 = env.get_direct_buffer_address(input_chunks).unwrap() as *const u8;
    let input_chunks_length = (input_chunk_size * number_of_chunks) as usize;

    let digests = env.get_direct_buffer_address(output_chunks).unwrap();
    let mut digest_offset = 0_isize;
    let mut message_offset = 0_isize;
    for _chunk_id in 0..number_of_chunks {

        let messages_vec = unsafe {
            core::slice::from_raw_parts(messages.offset(message_offset), input_chunks_length)
        };
        hasher.update(messages_vec);

        let output = hasher.finalize_reset();
        let slice = output.as_slice();

        let digest_slice = unsafe {
            slice::from_raw_parts_mut(digests.offset(digest_offset), 32_usize)
        };
        digest_slice.copy_from_slice(slice);

        digest_offset += 32;
        message_offset += input_chunk_size as isize;
    }

    let _raw = Box::into_raw(wrapper);
}

