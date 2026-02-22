fn main() {
    let mut cfg = cmake::Config::new("vendor/ZenKitCAPI");

    if std::env::var("CARGO_FEATURE_STATIC").is_ok() {
        cfg.define("BUILD_SHARED_LIBS", "OFF");
    }

    let dst = cfg.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=ZenKitCAPI");
    // println!("cargo:rustc-link-lib=static=ZenKitCAPI");
}
