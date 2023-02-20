fn main() {
    println!("cargo:rerun-if-env-changed=LANG");
    if let Ok(lang) = std::env::var("LANG") {
        let locale = lang.split('.').next().unwrap();
        if locale != "C" {
            println!(
                "cargo:rustc-env=MAOMI_I18N_LOCALE={}",
                locale,
            );
        }
    }
}
