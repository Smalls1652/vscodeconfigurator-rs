use std::{env, ffi::OsString, fmt, io, path::{absolute, Path, PathBuf}};

use crate::error::{CliError, CliErrorKind};

/// Represents an output directory.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputDirectory {
    pub path: String
}

impl OutputDirectory {
    /// Creates a new `OutputDirectory` from the current directory.
    pub fn from_current_dir() -> Self {
        let current_dir = env::current_dir().unwrap().into_os_string();

        Self {
            path: current_dir
                .to_string_lossy()
                .to_string()
        }
    }

    /// Creates a new `OutputDirectory` from an `OsString`.
    /// 
    /// ### Arguments
    /// 
    /// * `path` - The path to create the `OutputDirectory` from.
    pub fn from_os_string(path: OsString) -> Result<Self, io::Error> {
        let input_path = PathBuf::from(&path)
            .canonicalize();

        let input_path_string = match input_path.is_err() {
            true => {
                absolute(PathBuf::from(&path))
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            }

            false => {
                input_path
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            }
        };

        Ok(
            Self {
                path: input_path_string
            }
        )
    }

    /// Converts the `OutputDirectory` to a `PathBuf`.
    pub fn as_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }

    /// Resolves the home directory in the path if it starts with `~`.
    pub fn resolve_home_dir(&mut self) -> Result<OutputDirectory, CliError> {
        if self.path.starts_with("~") {
            let home_dir_env_var_key = match env::consts::OS {
                "windows" => "USERPROFILE",
                "unix" | "macos" => "HOME",
                _ => return Err(CliError::new("The operating system is not supported.", CliErrorKind::UnsupportedOperatingSystem).into()),
            };
            let home_dir_env_var = env::var(home_dir_env_var_key).unwrap();
            let home_dir = Path::new(&home_dir_env_var);

            self.path =
                PathBuf::from(&home_dir)
                    .join(self.path
                        .strip_prefix("~")
                        .unwrap()
                    )
                    .to_string_lossy()
                    .to_string();
        }

        Ok(self.clone())
    }

    /// Trims trailing slashes from the path.
    pub fn trim_trailing_slashes(&mut self) -> Result<OutputDirectory, CliError> {
        self.path = self.path
            .trim_end_matches('/')
            .trim_end_matches('.')
            .to_string();

        Ok(self.clone())
    }

    /// Creates the directory if it does not exist.
    pub fn create_if_not_exists(&self) -> Result<(), io::Error> {
        if !Path::new(&self.path).exists() {
            std::fs::create_dir(&self.path)?;
        }

        Ok(())
    }

    /// Converts the path to an absolute path.
    pub fn to_absolute(&self) -> PathBuf {
        return absolute(self.as_pathbuf()).unwrap();
    }
}

impl fmt::Display for OutputDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl Into<clap::builder::OsStr> for OutputDirectory {
    fn into(self) -> clap::builder::OsStr {
        self.path.into()
    }
}

impl Into<PathBuf> for OutputDirectory {
    fn into(self) -> PathBuf {
        PathBuf::from(self.path)
    }
}
