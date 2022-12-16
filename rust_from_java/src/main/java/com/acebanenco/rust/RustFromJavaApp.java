package com.acebanenco.rust;

// from https://blog.frankel.ch/start-rust/7/
public class RustFromJavaApp {

    public static void main(String[] args) {
        RustFromJavaService service = new RustFromJavaService(2);
        int doubleRust = service.doubleRust(2);
        System.out.println("doubleRust(2) = " + doubleRust);

        int timesRust = service.timesRust(3);
        System.out.println("timesRust(3) = " + timesRust);
    }
}
