use std::{
    env,
    fs,
    path::{Path, PathBuf}
};

use git_version::git_version;
use toml_edit::{value, DocumentMut};

/// Sets the version of the package to the latest git tag.
fn set_version() {
    let build_profile_env_var = env::var("PROFILE").clone().unwrap();

    let build_profile = build_profile_env_var.as_str();

    let current_git_version: &str = match build_profile {
        "release" => git_version!(args = ["--tags", "--abbrev=0"], fallback = "v0.0.0"),

        _ => git_version!(args = ["--tags"], fallback = "v0.0.0")
    };

    println!("cargo::rustc-env=CARGO_PKG_VERSION={}", current_git_version);

    // Check if the package version should be updated in the manifest file.
    // The 'VSCODECONFIGURATOR_UPDATE_MANIFEST_VERSION' environment variable should
    // be set to 'true' for this to happen.
    let should_update_cargo_manifest = env::var("VSCODECONFIGURATOR_UPDATE_MANIFEST_VERSION")
        .unwrap_or("false".to_string())
        .parse::<bool>()
        .unwrap();

    if should_update_cargo_manifest {
        println!(
            "cargo::warning=Package version will be set to '{}' in the manifest file.",
            current_git_version
        );

        let manifest_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let cargo_toml_path = manifest_dir_path.join("Cargo.toml");

        let mut cargo_toml = fs::read_to_string(&cargo_toml_path)
            .expect("Failed to read Cargo.toml.")
            .parse::<DocumentMut>()
            .expect("Failed to parse Cargo.toml.");

        cargo_toml["package"]["version"] =
            value(&current_git_version.trim_start_matches("v").to_string());

        fs::write(&cargo_toml_path, cargo_toml.to_string()).expect("Failed to write Cargo.toml.");
    }
}

/// Copies the templates directory to the build directory.
fn copy_templates_to_build_dir() {
    let manifest_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let output_dir_path = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("../../../")
        .canonicalize()
        .unwrap();

    let src_dir_path = manifest_dir_path.clone().join("src");

    let directories_to_copy = vec!["templates"];

    for directory_item in directories_to_copy {
        let src_copy_path = src_dir_path.join(directory_item);

        let output_copy_path = output_dir_path.clone().join(directory_item);

        println!(
            "Copying directory '{:?}' to '{:?}'...",
            src_copy_path, output_copy_path
        );

        copy_source_dir(&src_copy_path, &output_copy_path);
    }
}

/// Copies the source directory to the output directory.
///
/// ## Arguments
/// * `source_dir` - The source directory to copy.
/// * `output_dir` - The output directory to copy to.
fn copy_source_dir<I, O>(
    source_dir: I,
    output_dir: O
) where
    I: AsRef<Path>,
    O: AsRef<Path>
{
    let source_dir_path = source_dir.as_ref().to_path_buf();

    let output_directory_copy_path = output_dir.as_ref().to_path_buf();

    if output_directory_copy_path.exists() {
        fs::remove_dir_all(&output_directory_copy_path)
            .expect("Failed to remove existing directory.");
    }

    fs::create_dir(&output_directory_copy_path).expect("Failed to create directory.");

    for file_copy_item in fs::read_dir(source_dir_path).unwrap() {
        let file_copy_path = file_copy_item.unwrap().path();

        if file_copy_path.file_name().unwrap().to_str().unwrap() == ".DS_Store" {
            continue;
        }

        let output_file_copy_path = output_directory_copy_path
            .clone()
            .join(file_copy_path.file_name().unwrap());

        println!(
            "Copying file '{:?}' to '{:?}'...",
            file_copy_path, output_file_copy_path
        );

        if file_copy_path.is_file() {
            fs::copy(&file_copy_path, &output_file_copy_path).expect("Failed to copy file.");
        } else if file_copy_path.is_dir() {
            fs::create_dir(&output_file_copy_path).expect("Failed to create directory.");

            copy_source_dir(&file_copy_path, output_file_copy_path);
        } else {
            // Do nothing.
        }
    }
}

fn main() {
    copy_templates_to_build_dir();

    set_version();
}
