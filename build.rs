fn main() {
    println!("cargo:rerun-if-env-changed=MAOMI_RUST_ANALYZER");
    println!("cargo:rerun-if-env-changed=MAOMI_I18N_LOCALE");
    println!("cargo:rerun-if-env-changed=MAOMI_PATH_PREFIX");
}
