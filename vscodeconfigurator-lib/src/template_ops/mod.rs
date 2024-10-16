use std::{env, fs, path::PathBuf};

pub mod csharp;
pub mod rust;
pub mod vscode;

/// Represents a template file.
pub struct TemplateFile {
    /// The path to the template file.
    pub template_file_path: PathBuf,

    /// The path to output the file to.
    pub output_file_path: PathBuf,

    /// The name of the output file.
    pub output_file_name: String,

    /// Whether the output file exists.
    pub output_file_exists: bool
}

impl TemplateFile {
    /// Creates a new instance of `TemplateFile`.
    ///
    /// # Arguments
    ///
    /// - `relative_template_file_path` - The relative path to the template file
    ///   from the binary's directory.
    /// - `output_directory_path` - The path to the directory to output the
    ///   file.
    /// - `output_file_name` - The name of the output file.
    ///
    /// # Examples
    ///
    /// ## Example 01
    ///
    /// Creates a new instance of `TemplateFile` for the `GitVersion.yml`
    /// template file.
    ///
    /// ```rust
    /// use std::{env, path::PathBuf};
    ///
    /// let relative_template_file_path = "csharp/GitVersion/GitVersion.yml";
    /// let output_directory_path = PathBuf::from(env::current_dir().unwrap());
    /// let output_file_name = "GitVersion.yml";
    ///
    /// let template_file = TemplateFile::new(
    ///     relative_template_file_path,
    ///     output_directory_path,
    ///     output_file_name
    /// );
    /// ```
    pub fn new(
        relative_template_file_path: &str,
        output_directory_path: &PathBuf,
        output_file_name: &str
    ) -> Self {
        let binary_path = env::current_exe().unwrap();
        let binary_dir = binary_path.parent().unwrap();

        let template_file_path =
            binary_dir.join(format!("templates/{}", relative_template_file_path));

        let output_file_path = output_directory_path.join(output_file_name);

        let output_file_exists = output_file_path.exists();

        Self {
            template_file_path,
            output_file_path,
            output_file_name: output_file_name.to_string(),
            output_file_exists
        }
    }

    /// Copies the template file to the output directory.
    ///
    /// # Examples
    ///
    /// ## Example 01
    ///
    /// Copies the 'GitVersion.yml' template file to the project root.
    ///
    /// ```rust
    /// use vscodeconfigurator::template_ops::TemplateFile;
    ///
    /// let relative_template_file_path = "csharp/GitVersion/GitVersion.yml";
    /// let output_directory_path = PathBuf::from(env::current_dir().unwrap());
    /// let output_file_name = "GitVersion.yml";
    ///
    /// let template_file = TemplateFile::new(
    ///     relative_template_file_path,
    ///     output_directory_path,
    ///     output_file_name
    /// );
    ///
    /// template_file.copy_file();
    /// ```
    pub fn copy_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.output_file_exists {
            fs::remove_file(&self.output_file_path)?;
        }

        fs::copy(&self.template_file_path, &self.output_file_path)?;

        Ok(())
    }
}
