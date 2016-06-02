extern crate gcc;
use std::env;
use std::io::{self, Write};
use std::fmt;

fn main() {
                
    match env::var("JDK_HOME") {
        Ok(v) => {
            gcc::Config::new()
                        //.compiler("g++")
                        .file("c/rs_jni_pipe.c")
                        .flag("-fpermissive")
                        .include(format!("{jdk}/include", jdk=v))
                        .include(format!("{jdk}/include/win32", jdk=v))
                        .compile("librs_jni_pipe.a");
            println!("cargo:rustc-flags=-l jvm -L {jdk}/jre/bin/server -L {jdk}/jre/bin -L {jdk}/bin -L {jdk}/lib", jdk=v);
        },
        Err(_) => ()
    }
}