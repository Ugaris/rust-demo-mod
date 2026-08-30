fn main() {
    // Windows mods must link the client's import library (lib/moac.lib from
    // the client release's mod-sdk.zip). Without it the produced DLL carries
    // no imports from the client, and every API call is a null pointer.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rustc-link-search=native=lib");
        println!("cargo:rustc-link-lib=dylib=moac");
    }
}
