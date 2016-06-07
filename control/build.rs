extern crate gcc;
use std::env;
use std::io::{self, Write};
use std::fmt;

fn main() {
                
    match (env::var("JDK_HOME"), env::var("OPENCV_PATH")) {
        (Ok(jdk_path), Ok(opencv_path)) => {
            //compile JNI code
            gcc::Config::new()
                //.compiler("g++")
                .file("c/rs_jni_pipe.c")
                .flag("-fpermissive")
                .include(format!("{jdk}/include", jdk=jdk_path))
                .include(format!("{jdk}/include/win32", jdk=jdk_path))
                .compile("librs_jni_pipe.a");
            //compile opencv code
            gcc::Config::new()
                .file("c/opencv_cam.c")
                .flag("-fpermissive")
                .include(format!("{opencv}/include", opencv=opencv_path))
                .compile("libopencv_cam.a");
                        
            println!("cargo:rustc-flags=-l jvm -L {jdk}/jre/bin/server -L {jdk}/jre/bin -L {jdk}/bin -L {jdk}/lib", jdk=jdk_path);
        },
        _ => ()
    }
}