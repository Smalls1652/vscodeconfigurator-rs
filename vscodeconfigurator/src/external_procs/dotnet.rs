use std::{fs, path::PathBuf, process};

use crate::console_utils::ConsoleUtils;

/// Initializes a new .NET solution.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the new solution.
/// * `solution_name` - The name of the solution file.
pub fn initalize_dotnet_solution(
    output_directory: &PathBuf,
    solution_name: &String,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_info(format!(
        "- 📦 Initializing .NET solution '{:}.sln'... ",
        solution_name
    ))?;
    console_utils.save_cursor_position()?;

    let output_file_name = format!("{:}.sln", &solution_name);
    let output_file_path = PathBuf::from(output_directory).join(&output_file_name);

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            console_utils.restore_cursor_position_and_clear_below()?;
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", &output_file_name).as_str());

        console_utils.restore_cursor_position_and_clear_below()?;
    }

    let dotnet_proc_args = vec!["new", "sln", "--name", solution_name];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()
        .expect("Failed to run 'dotnet new sln' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Add a 'global.json' file to the project root.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the project.
pub fn add_dotnet_globaljson(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_info(format!("- 📄 Adding 'global.json' to project root... "))?;
    console_utils.save_cursor_position()?;

    let output_file_name = "global.json";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            console_utils.restore_cursor_position_and_clear_below()?;
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());

        console_utils.restore_cursor_position_and_clear_below()?;
    }

    let dotnet_proc_args = vec!["new", "globaljson", "--roll-forward", "latestMinor"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()
        .expect("Failed to run 'dotnet new globaljson' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Add a '.gitignore' file to the project root.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the project.
pub fn add_dotnet_gitignore(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_info(format!("- 📄 Adding '.gitignore' to project root... "))?;
    console_utils.save_cursor_position()?;

    let output_file_name = ".gitignore";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            console_utils.restore_cursor_position_and_clear_below()?;
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());

        console_utils.restore_cursor_position_and_clear_below()?;
    }

    let dotnet_proc_args = vec!["new", "gitignore"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()
        .expect("Failed to run 'dotnet new gitignore' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Add a 'Directory.Build.props' file to the project root.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the project.
pub fn add_dotnet_buildprops(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_info(format!("- 📄 Adding 'Directory.Build.props' to project root... "))?;
    console_utils.save_cursor_position()?;

    let output_file_name = "Directory.Build.props";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            console_utils.restore_cursor_position_and_clear_below()?;
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());

        console_utils.restore_cursor_position_and_clear_below()?;
    }

    let dotnet_proc_args = vec!["new", "buildprops", "--use-artifacts"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()
        .expect("Failed to run 'dotnet new buildprops' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Add a 'NuGet.Config' file to the project root.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the project.
pub fn add_dotnet_nugetconfig(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_info(format!("- 📄 Adding 'NuGet.Config' to project root... "))?;
    console_utils.save_cursor_position()?;

    let output_file_name = "NuGet.Config";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            console_utils.restore_cursor_position_and_clear_below()?;
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());

        console_utils.restore_cursor_position_and_clear_below()?;
    }

    let dotnet_proc_args = vec!["new", "nugetconfig"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()
        .expect("Failed to run 'dotnet new nugetconfig' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Add a 'Directory.Packages.props' file to the project root.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the project.
pub fn add_dotnet_packagesprops(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_info(format!("- 📄 Adding 'Directory.Packages.props' to project root... "))?;
    console_utils.save_cursor_position()?;

    let output_file_name = "Directory.Packages.props";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            console_utils.restore_cursor_position_and_clear_below()?;
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());

        console_utils.restore_cursor_position_and_clear_below()?;
    }

    let dotnet_proc_args = vec!["new", "packagesprops"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()
        .expect("Failed to run 'dotnet new packagesprops' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Add a .NET tool to the project.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the project.
/// * `tool_name` - The name of the tool to add.
pub fn add_dotnet_tool(
    output_directory: &PathBuf,
    tool_name: &str,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    initialize_dotnet_tool_manifest(output_directory, console_utils)?;

    console_utils.write_info(format!("- 📦 Adding .NET tool '{:}'... ", tool_name))?;
    console_utils.save_cursor_position()?;

    let dotnet_proc_args = vec!["tool", "install", tool_name, "--tool-manifest"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()
        .expect("Failed to run 'dotnet tool install' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Add a tool manifest file to the project root.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the project.
fn initialize_dotnet_tool_manifest(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_directory = PathBuf::from(output_directory).join(".config");
    let tool_manifest_file_path = PathBuf::from(&config_directory).join("dotnet-tools.json");

    if !config_directory.exists() {
        fs::create_dir(&config_directory)
            .expect("Failed to create '.config' directory.");
    }

    if tool_manifest_file_path.exists() {
        return Ok(());
    }

    console_utils.write_info(format!("- 📦 Initializing .NET tool manifiest... "))?;
    console_utils.save_cursor_position()?;

    let dotnet_proc_args = vec!["new", "tool-manifest"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()
        .expect("Failed to run 'dotnet new tool-manifest' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Add a project to a solution.
/// 
/// ## Arguments
/// 
/// * `solution_file_path` - The path to the solution file.
/// * `project_file_path` - The path to the project file.
/// * `console_utils` - The console utilities.
pub fn add_project_to_solution(
    solution_file_path: &PathBuf,
    project_file_path: &PathBuf,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    let project_file_path = project_file_path.canonicalize().unwrap();
    let project_path_relative = project_file_path.strip_prefix(solution_file_path.parent().unwrap())?;

    console_utils.write_info(format!("- 📄 Adding '{:}' to solution... ", project_path_relative.to_str().unwrap()))?;
    console_utils.save_cursor_position()?;

    let dotnet_proc_args = vec!["sln", solution_file_path.to_str().unwrap(), "add", project_file_path.to_str().unwrap()];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .output()
        .expect("Failed to run 'dotnet sln add' command.");

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}
