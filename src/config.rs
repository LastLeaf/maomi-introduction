pub const fn path_prefix() -> &'static str {
    match option_env!("MAOMI_PATH_PREFIX") {
        Some(x) => x,
        None => "",
    }
}

pub fn res_path(rel_path: &str) -> String {
    format!("{}/res/{}", path_prefix(), rel_path)
}
