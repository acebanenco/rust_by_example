package com.acebanenco.rust;

import java.nio.ByteBuffer;
import java.security.DigestException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public class MessageDigestSha256Impl implements MessageDigestSha256 {

    private final MessageDigest md;

    public MessageDigestSha256Impl() {
        try {
            md = MessageDigest.getInstance("SHA-256");
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public void digestAll(ByteBuffer messages, int messageSize, ByteBuffer digests, int len) {
        int digestLength = md.getDigestLength();
        byte[] msgArray = messages.array();
        byte[] dgsArray = digests.array();

        int msgOffset = 0;
        int dgsOffset = 0;
        for (int id = 0; id < len; id++) {
            md.update(msgArray, msgOffset, messageSize);
            try {
                md.digest(dgsArray, dgsOffset, digestLength);
            } catch (DigestException e) {
                throw new RuntimeException(e);
            }
            msgOffset += messageSize;
            dgsOffset += digestLength;
        }
    }
}
