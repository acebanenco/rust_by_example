package com.acebanenco.rust;

import java.nio.ByteBuffer;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.LongAdder;
import java.util.stream.IntStream;

// from https://blog.frankel.ch/start-rust/7/
public class RustFromJavaApp {

    private static final int batchSize = 100_000;
    private static final ConcurrentLinkedQueue<MdContext> initializedContexts = new ConcurrentLinkedQueue<>();

    private static MdContext newMdContext() {
        MdContext context = initializedContexts.poll();
        if (context != null) {
            return context;
        }
        return getMdContext();
    }

    private static MdContext getMdContext() {
        return MdContext.create();
//        return MdContext.createNative();
    }

    public static void main(String[] args) {
        int numOfProcessors = Runtime.getRuntime().availableProcessors();
        for (int i = 0; i < numOfProcessors; i++) {
            initializedContexts.add(getMdContext());
        }

        long time = System.currentTimeMillis();

        LongAdder longAdder = new LongAdder();

        IntStream.range(0, 100_000_000 / batchSize)
                .parallel()
                .forEach(batch -> {
                    int from = batch * batchSize;
                    longAdder.add(getMatchCount(from, from + batchSize));
                });

        int count = longAdder.intValue();

        time = System.currentTimeMillis() - time;

        System.out.printf("Found %d matches in %d seconds%n", count, time / 1000);
    }

    private static int getMatchCount(int from, int to) {
        MdContext ctx = getMd();

        int len = to - from;

        ByteBuffer message = ctx.message;
        ByteBuffer digest = ctx.digest;

        int msgSize = 4;
//        writeMessages(from, len, message, msgSize);

        ctx.md.digestAll(message, msgSize, digest, len);

        int dgstSize = 32;
        int count = 0;
        for (int i = 0; i < len; i++) {
//            if (digest.get(i * dgstSize) == 0) {
//                count++;
//            }
        }
        return count;
    }

    private static void writeMessages(int from, int len, ByteBuffer message, int msgSize) {
        assert len == message.capacity() / msgSize;
        for (int i = 0; i < len; i++) {
            writeInt(message, i * msgSize, from + i);
        }
    }



    private static class MdContext {
//        private final MessageDigestSha256 md = new NativeMessageDigestSha256Impl();
        private final MessageDigestSha256 md;
        private final ByteBuffer message;
        private final ByteBuffer digest;

        MdContext(MessageDigestSha256 md, ByteBuffer message, ByteBuffer digest) {
            this.md = md;
            this.message = message;
            this.digest = digest;
        }

        static MdContext create() {
            return new MdContext(
                    new MessageDigestSha256Impl(),
                    ByteBuffer.allocate(batchSize * 4),
                    ByteBuffer.allocate(batchSize * 32)
            );
        }

        static MdContext createNative() {
            return new MdContext(
                    new NativeMessageDigestSha256Impl(),
                    ByteBuffer.allocateDirect(batchSize * 4),
                    ByteBuffer.allocateDirect(batchSize * 32)
            );
        }
    }

    private static final ThreadLocal<MdContext> mdLocal
            = ThreadLocal.withInitial(RustFromJavaApp::newMdContext);



    private static MdContext getMd() {
        return mdLocal.get();
    }

    private static void writeInt(ByteBuffer message, int from, int value) {
        message.put(from, (byte) (value >>> 24))
                .put(from + 1, (byte) (value >>> 16))
                .put(from + 2, (byte) (value >>> 8))
                .put(from + 3, (byte) value);
    }
}
