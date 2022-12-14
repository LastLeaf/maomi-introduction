use std::path::PathBuf;

fn main() {
    if let Ok(crate_path) = std::env::var("CARGO_MANIFEST_DIR") {
        let crate_path = PathBuf::from(crate_path);

        // enable CSS output and specify the CSS output directory
        println!(
            "cargo:rustc-env=MAOMI_CSS_OUT_DIR={}",
            crate_path.join("pkg").to_str().unwrap(),
        );

        // specify the CSS output mode
        #[cfg(debug_assertions)]
        println!("cargo:rustc-env=MAOMI_CSS_OUT_MODE=debug");

        // specify where to find the CSS @import files (default to `src`)
        println!(
            "cargo:rustc-env=MAOMI_CSS_IMPORT_DIR={}",
            crate_path.join("src").to_str().unwrap(),
        );

        // enable i18n support and specify the current locale
        if let Ok(lang) = std::env::var("LANG") {
            let locale = lang.split('.').next().unwrap();
            if locale != "C" {
                println!(
                    "cargo:rustc-env=MAOMI_I18N_LOCALE={}",
                    locale,
                );
            }
        }
        println!("cargo:rerun-if-env-changed=LANG");

        // specify where to find the locale files
        println!(
            "cargo:rustc-env=MAOMI_I18N_DIR={}",
            crate_path.join("i18n").to_str().unwrap(),
        );
    }
}
