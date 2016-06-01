extern crate gcc;

fn main() {
    gcc::Config::new()
                .compiler("g++")
                .file("c/rs_jni_pipe.cpp")
                .flag("-fpermissive")
                .include("C:\\Program Files\\Java\\jdk1.7.0_75\\include")
                .include("C:\\Program Files\\Java\\jdk1.7.0_75\\include\\win32")
                .compile("librs_jni_pipe.a");
                
    println!("cargo:rustc-flags=-l jvm -L .");
}