use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jint;

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_RustFromJavaService_doubleRust
(_env: JNIEnv, _obj: JObject, x: jint) -> jint {
    x * 2
}

#[no_mangle]
pub extern "system" fn Java_com_acebanenco_rust_RustFromJavaService_timesRust
(_env: JNIEnv, _obj: JObject, x: jint) -> jint {
    let state = _env.get_field(_obj, "state", "I");
    state.unwrap().i().unwrap() * x
}
