use std::{fs, path::PathBuf};

use serde_json::Value;

use crate::{console_utils::ConsoleUtils, subcommands::csharp::init::CsharpLspOption};

/// Updates the C# LSP option in the `.vscode/settings.json` file.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `csharp_lsp` - The C# language server to use.
/// * `console_utils` - The console utilities.
pub fn update_csharp_lsp(output_directory: &PathBuf, csharp_lsp: CsharpLspOption, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn std::error::Error>> {
    let vscode_settings_path = output_directory.join(".vscode/settings.json");

    console_utils.write_info(format!("- 📄 Updating C# LSP option in tasks.json... "))?;

    let vscode_settings_json = fs::read_to_string(&vscode_settings_path)?;

    let mut vscode_settings: Value = serde_json::from_str(&vscode_settings_json.as_str())?;

    vscode_settings["dotnet.server.useOmnisharp"] = match csharp_lsp {
        CsharpLspOption::CsharpLsp => Value::Bool(false),
        CsharpLspOption::OmniSharp => Value::Bool(true)
    };

    vscode_settings["dotnet.server.path"] = match csharp_lsp {
        CsharpLspOption::CsharpLsp => Value::String("".to_string()),
        CsharpLspOption::OmniSharp => Value::String("omnisharp".to_string())
    };

    let updated_vscode_settings_json = serde_json::to_string_pretty(&vscode_settings)?;

    fs::write(&vscode_settings_path, updated_vscode_settings_json)?;

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}
