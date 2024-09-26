use std::{env, path::PathBuf};

pub fn get_core_templates_path() -> PathBuf {
    let binary_path = env::current_exe().unwrap();

    let binary_dir = binary_path.parent().unwrap();

    let core_templates_path = binary_dir.join("Templates");

    return core_templates_path;
}
