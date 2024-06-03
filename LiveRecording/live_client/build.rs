fn main() {
    println!("cargo:rustc-link-search=native=.");
    // println!("cargo:rustc-link-lib=dylib=live_push");
    println!("cargo:rustc-link-lib=dylib=avcodec-61");
    println!("cargo:rustc-link-lib=dylib=avdevice-61");
    println!("cargo:rustc-link-lib=dylib=avfilter-10");
    println!("cargo:rustc-link-lib=dylib=avformat-61");
    println!("cargo:rustc-link-lib=dylib=avutil-59");
    println!("cargo:rustc-link-lib=dylib=live_push");
    println!("cargo:rustc-link-lib=dylib=postproc-58");
    println!("cargo:rustc-link-lib=dylib=swresample-5");
    println!("cargo:rustc-link-lib=dylib=swscale-8");
}
