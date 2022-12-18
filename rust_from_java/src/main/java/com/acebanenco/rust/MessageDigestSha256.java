package com.acebanenco.rust;

import java.nio.ByteBuffer;

public interface MessageDigestSha256 {
    void digestAll(ByteBuffer messages, int messageSize, ByteBuffer digests, int len);
}
