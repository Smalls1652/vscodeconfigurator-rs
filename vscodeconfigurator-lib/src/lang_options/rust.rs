use clap::ValueEnum;

/// The type of Cargo package template to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CargoPackageTemplateOption {
    /// A binary.
    #[value(name = "Binary")]
    Binary,

    /// A library.
    #[value(name = "Library")]
    Library
}
