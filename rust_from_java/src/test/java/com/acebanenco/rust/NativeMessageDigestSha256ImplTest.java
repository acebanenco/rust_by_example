package com.acebanenco.rust;

import org.junit.jupiter.api.Test;

import java.nio.ByteBuffer;

import static org.junit.jupiter.api.Assertions.*;

class NativeMessageDigestSha256ImplTest {

    @Test
    void testNewInstance() {
        int count = 0;
        try (NativeMessageDigestSha256Impl md = new NativeMessageDigestSha256Impl()) {
            ByteBuffer messages = ByteBuffer.allocateDirect(4 * 2);
            messages.put(0, (byte)1)
                    .put(0, (byte)2)
                    .put(0, (byte)3)
                    .put(0, (byte)4);
            ByteBuffer digests = ByteBuffer.allocateDirect(32 * 2);

            messages.put(0, (byte)2)
                    .put(0, (byte)3)
                    .put(0, (byte)4)
                    .put(0, (byte)5);
            md.digestAll(messages, 4, digests, 1);
            if (digests.get(0) == 0 ) {
                count++;
            }

            md.digestAll(messages, 4, digests, 2);
            if (digests.get(0) == 0 ) {
                count++;
            }
        }
        assertEquals(0, count);
    }

}