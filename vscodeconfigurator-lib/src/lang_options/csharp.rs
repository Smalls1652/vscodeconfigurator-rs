use clap::ValueEnum;

/// The type of C# language server to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CsharpLspOption {
    /// The C# language server provided by the C# extension for Visual Studio
    /// Code.
    #[value(name = "CsharpLsp")]
    CsharpLsp,

    /// The original C# language server provided by OmniSharp.
    #[value(name = "OmniSharp")]
    OmniSharp
}
