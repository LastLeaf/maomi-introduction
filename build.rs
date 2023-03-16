fn main() {
    println!("cargo:rerun-if-env-changed=MAOMI_RUST_ANALYZER");
    println!("cargo:rerun-if-env-changed=LANG");
    if let Ok(lang) = std::env::var("LANG") {
        let locale = lang.split('.').next().unwrap();
        if locale != "C" && std::env::var("MAOMI_I18N_LOCALE").is_err() {
            println!(
                "cargo:rustc-env=MAOMI_I18N_LOCALE={}",
                locale,
            );
        }
    }
}
