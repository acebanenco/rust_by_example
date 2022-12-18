package com.acebanenco.rust;

import java.nio.ByteBuffer;

public class NativeMessageDigestSha256Impl implements MessageDigestSha256 {

    static {
        System.loadLibrary("rust_openssl_lib");
    }

    private final long md;
    private final long context;

    public NativeMessageDigestSha256Impl() {
        this.md = md_init();
        this.context = init(md);
    }

    @Override
    public void digestAll(ByteBuffer messages, int messageSize, ByteBuffer digests, int len) {
        digest_multi(md, context, messages, messageSize, digests, len);
    }

    private static native long init(long md);

    private static native long md_init();

    private static native void digest_multi(long md, long context, ByteBuffer input, int inputSize, ByteBuffer output, int len);

    private static native void free(long context);

    private static native void md_free(long context);

}
