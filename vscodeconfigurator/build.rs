use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn copy_source_dir<I, O>(source_dir: I, output_dir: O)
where I: AsRef<Path>, O: AsRef<Path> {
    let source_dir_path = source_dir
        .as_ref()
        .to_path_buf();

    let output_directory_copy_path = output_dir
        .as_ref()
        .to_path_buf();

    if output_directory_copy_path.exists() {
        fs::remove_dir_all(&output_directory_copy_path)
            .expect("Failed to remove existing directory.");
    }

    fs::create_dir(&output_directory_copy_path)
        .expect("Failed to create directory.");

    for file_copy_item in fs::read_dir(source_dir_path).unwrap() {
        let file_copy_path = file_copy_item.unwrap().path();

        if file_copy_path.file_name().unwrap().to_str().unwrap() == ".DS_Store" {
            continue;
        }

        let output_file_copy_path = output_directory_copy_path
            .clone()
            .join(file_copy_path.file_name().unwrap());

        println!("Copying file '{:?}' to '{:?}'...", file_copy_path, output_file_copy_path);

        if file_copy_path.is_file() {
            fs::copy(&file_copy_path, &output_file_copy_path)
                .expect("Failed to copy file.");
        }
        else if file_copy_path.is_dir() {
            fs::create_dir(&output_file_copy_path)
                .expect("Failed to create directory.");

            copy_source_dir(&file_copy_path, output_file_copy_path);
        }
        else {
            // Do nothing.
        }

    }
}

fn main() {
    let manifest_dir_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .unwrap()
    );

    let output_dir_path = manifest_dir_path
        .clone()
        .parent()
        .unwrap()
        .join("target")
        .join(
            env::var("PROFILE")
                .unwrap()
        );

    let src_dir_path = manifest_dir_path
        .clone()
        .join("src");

    let directories_to_copy = vec![
        "templates"
    ];

    for directory_item in directories_to_copy {
        let src_copy_path = src_dir_path
            .join(directory_item);

        let output_copy_path = output_dir_path
            .clone()
            .join(directory_item);

        println!("Copying directory '{:?}' to '{:?}'...", src_copy_path, output_copy_path);

        copy_source_dir(&src_copy_path, &output_copy_path);
    }
}
