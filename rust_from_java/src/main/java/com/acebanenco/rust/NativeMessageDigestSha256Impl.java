package com.acebanenco.rust;

import java.nio.ByteBuffer;

public class NativeMessageDigestSha256Impl implements MessageDigestSha256, AutoCloseable {

    static {
        System.loadLibrary("rust_sha256_lib");
    }

    private final long hasher;

    public NativeMessageDigestSha256Impl() {
        this.hasher = sha256_hasher_init();
    }

    @Override
    public void close() {
        free(hasher);
    }

    @Override
    public void digestAll(ByteBuffer messages, int messageSize, ByteBuffer digests, int len) {
        digestAll(hasher, messages, messageSize, digests, len);
    }

    private static native long sha256_hasher_init();

    private static native void free(long hasher);

    private static native void digestAll(
            long hasher,
            ByteBuffer inputChunks,
            int inputChunkSize,
            ByteBuffer outputChunks,
            int numberOfChunks);

}
