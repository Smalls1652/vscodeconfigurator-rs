use std::{env, ffi::OsString, path::PathBuf};

/// Gets the default value for the `output_directory` (`--output-directory`) argument if 
/// it is not provided by the user.
pub fn get_output_directory_default_value() -> OsString {
    env::current_dir().unwrap().into_os_string()
}

/// Gets the path to the core templates directory.
pub fn get_core_templates_path() -> PathBuf {
    let binary_path = env::current_exe().unwrap();

    let binary_dir = binary_path.parent().unwrap();

    let core_templates_path = binary_dir.join("Templates");

    return core_templates_path;
}
