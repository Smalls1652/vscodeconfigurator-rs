use std::{error::Error, fmt};

/// Represents an error that occurred during the execution of the CLI tool.
#[derive(Debug)]
pub struct CliError {
    /// The error message.
    pub message: String,

    /// The kind of error.
    pub kind: CliErrorKind
}

impl CliError {
    /// Creates a new instance of `CliError`.
    pub fn new(
        message: &str,
        kind: CliErrorKind
    ) -> Self {
        Self {
            message: message.to_string(),
            kind
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CliError {}

/// Represents the kind of error that occurred during the execution of the CLI
/// tool.
#[derive(Debug, Clone)]
pub enum CliErrorKind {
    /// No subcommand was provided.
    NoSubcommandProvided,

    /// The solution name could not be parsed.
    UnableToParseSolutionName,

    #[allow(dead_code)]
    /// The file path does not exist.
    FilePathDoesNotExist,

    /// The operating system is not supported.
    UnsupportedOperatingSystem,

    /// The output directory does not exist.
    OutputDirectoryDoesNotExist,

    #[allow(dead_code)]
    /// Unknown error.
    UnknownError
}

impl fmt::Display for CliErrorKind {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
