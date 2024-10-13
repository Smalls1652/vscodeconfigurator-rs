use std::{fs, path::PathBuf, process};

use vscodeconfigurator_lib::logging::{ConsoleLogger, OutputEmoji};

/// Initializes a new .NET solution.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the new solution.
/// - `solution_name` - The name of the solution file.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Initializes a new solution, named `MySolution`, in the `MySolution`
/// directory in the temp directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("MySolution");
/// let solution_name = "MySolution";
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// initalize_dotnet_solution(&output_directory, &solution_name, force, logger);
/// ```
pub fn initalize_dotnet_solution(
    output_directory: &PathBuf,
    solution_name: &String,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    logger.write_operation_log(
        format!("Initializing .NET solution '{:}.sln'...", solution_name).as_str(),
        OutputEmoji::Package
    )?;
    logger.save_cursor_position()?;

    let output_file_name = format!("{:}.sln", &solution_name);
    let output_file_path = PathBuf::from(output_directory).join(&output_file_name);

    if output_file_path.exists() {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", &output_file_name).as_str());
    }

    let dotnet_proc_args = vec!["new", "sln", "--name", solution_name];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Add a `global.json` file to the project root.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the project.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Add a `global.json` file to the `MySolution` directory in the temp
/// directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("MySolution");
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// add_dotnet_globaljson(&output_directory, force, logger);
/// ```
pub fn add_dotnet_globaljson(
    output_directory: &PathBuf,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    logger.write_operation_log(
        "Adding 'global.json' to project root...",
        OutputEmoji::Document
    )?;
    logger.save_cursor_position()?;

    let output_file_name = "global.json";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());
    }

    let dotnet_proc_args = vec!["new", "globaljson", "--roll-forward", "latestMinor"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Add a `.gitignore` file to the project root.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the project.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Add a `.gitignore` file to the `MySolution` directory in the temp directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("MySolution");
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// add_dotnet_gitignore(&output_directory, force, logger);
/// ```
pub fn add_dotnet_gitignore(
    output_directory: &PathBuf,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    logger.write_operation_log(
        "Adding '.gitignore' to project root...",
        OutputEmoji::Document
    )?;
    logger.save_cursor_position()?;

    let output_file_name = ".gitignore";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());
    }

    let dotnet_proc_args = vec!["new", "gitignore"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Add a `Directory.Build.props` file to the project root.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the project.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Add a `Directory.Build.props` file to the `MySolution` directory in the temp
/// directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("MySolution");
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// add_dotnet_buildprops(&output_directory, force, logger);
/// ```
pub fn add_dotnet_buildprops(
    output_directory: &PathBuf,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    logger.write_operation_log(
        "Adding 'Directory.Build.props' to project root...",
        OutputEmoji::Document
    )?;
    logger.save_cursor_position()?;

    let output_file_name = "Directory.Build.props";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());
    }

    let dotnet_proc_args = vec!["new", "buildprops", "--use-artifacts"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Add a `NuGet.Config` file to the project root.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the project.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Add a `NuGet.Config` file to the `MySolution` directory in the temp
/// directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("MySolution");
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// add_dotnet_nugetconfig(&output_directory, force, logger);
/// ```
pub fn add_dotnet_nugetconfig(
    output_directory: &PathBuf,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    logger.write_operation_log(
        "Adding 'NuGet.Config' to project root...",
        OutputEmoji::Document
    )?;
    logger.save_cursor_position()?;

    let output_file_name = "NuGet.Config";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());
    }

    let dotnet_proc_args = vec!["new", "nugetconfig"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Add a `Directory.Packages.props` file to the project root.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the project.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Add a `Directory.Packages.props` file to the `MySolution` directory in the
/// temp directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("MySolution");
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// add_dotnet_packagesprops(&output_directory, force, logger);
/// ```
pub fn add_dotnet_packagesprops(
    output_directory: &PathBuf,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    logger.write_operation_log(
        "Adding 'Directory.Packages.props' to project root...",
        OutputEmoji::Document
    )?;
    logger.save_cursor_position()?;

    let output_file_name = "Directory.Packages.props";
    let output_file_path = PathBuf::from(output_directory).join(output_file_name);

    if output_file_path.exists() {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", output_file_name).as_str());
    }

    let dotnet_proc_args = vec!["new", "packagesprops"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Add a .NET tool to the project.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the project.
/// - `tool_name` - The name of the tool to add.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Add a .NET tool named `gitversion` to the `MySolution` directory in the temp
/// directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("MySolution");
/// let tool_name = "gitversion";
/// let mut logger = ConsoleLogger::new();
///
/// add_dotnet_tool(&output_directory, tool_name, logger);
/// ```
pub fn add_dotnet_tool(
    output_directory: &PathBuf,
    tool_name: &str,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    initialize_dotnet_tool_manifest(output_directory, logger)?;

    logger.write_info(format!("- 📦 Adding .NET tool '{:}'... ", tool_name))?;
    logger.save_cursor_position()?;

    let dotnet_proc_args = vec!["tool", "install", tool_name, "--tool-manifest"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Add a tool manifest file to the project root.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the project.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Add a tool manifest file to the `MySolution` directory in the temp
/// directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("MySolution");
/// let mut logger = ConsoleLogger::new();
///
/// initialize_dotnet_tool_manifest(&output_directory, logger);
/// ```
fn initialize_dotnet_tool_manifest(
    output_directory: &PathBuf,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    let config_directory = PathBuf::from(output_directory).join(".config");
    let tool_manifest_file_path = PathBuf::from(&config_directory).join("dotnet-tools.json");

    if !config_directory.exists() {
        fs::create_dir(&config_directory)?;
    }

    if tool_manifest_file_path.exists() {
        return Ok(());
    }

    logger
        .write_operation_log("Initializing .NET tool manifiest...", OutputEmoji::Package)?;
    logger.save_cursor_position()?;

    let dotnet_proc_args = vec!["new", "tool-manifest"];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Add a project to a solution.
///
/// # Arguments
///
/// - `solution_file_path` - The path to the solution file.
/// - `project_file_path` - The path to the project file.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Add a project in the `ConsoleApp` directory to the `MySolution.sln` solution
/// file in the `MySolution` directory in the temp directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let solution_file_path = std::env::temp_dir()
///    .join("MySolution")
/// let project_file_path = std::env::temp_dir()
///   .join("MySolution/ConsoleApp");
/// let mut logger = ConsoleLogger::new();
///
/// add_project_to_solution(&solution_file_path, &project_file_path, logger);
/// ```
pub fn add_project_to_solution(
    solution_file_path: &PathBuf,
    project_file_path: &PathBuf,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    let project_file_path = project_file_path.canonicalize().unwrap();
    let project_path_relative =
        project_file_path.strip_prefix(solution_file_path.parent().unwrap())?;

    logger.write_info(format!(
        "- 📄 Adding '{:}' to solution... ",
        project_path_relative.to_str().unwrap()
    ))?;
    logger.save_cursor_position()?;

    let dotnet_proc_args = vec![
        "sln",
        solution_file_path.to_str().unwrap(),
        "add",
        project_file_path.to_str().unwrap(),
    ];

    process::Command::new("dotnet")
        .args(dotnet_proc_args)
        .output()?;

    logger.write_operation_success_log()?;

    Ok(())
}
