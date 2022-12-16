package com.acebanenco.rust;

import lombok.Getter;
import lombok.Setter;

public class RustFromJavaService {

    @Getter
    @Setter
    private int state;

    static {
        System.loadLibrary("rust_by_example");
    }

    public RustFromJavaService(int state) {
        this.state = state;
    }

    public native int doubleRust(int input);

    public native int timesRust(int input);

}
