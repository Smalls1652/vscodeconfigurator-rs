# `vscode-configurator csharp init`

## Table of Contents

- [`vscode-configurator`](../README.md)
  - [`csharp`](./README.md)
    - `init`
    - [`add`](./add.md)
  - [`rust`](../rust/README.md)
    - [`init`](../rust/init.md)
    - [`add`](../rust/add.md)
  - [`completions`](../completions.md)

## Description

Initialize a new C# project.

## Usage

```bash
vscode-configurator csharp init [options]
```

### Options

#### `-o`, `--output-directory`

The output directory for the new project.

**Default value**: The current working directory.

#### `-n`, `--solution-name`

The default solution file.

**Default value**: The name of the current working directory.

#### `--add-gitversion`

Whether to add GitVersion to the new project.

#### `--add-nuget-config`

Whether to add a NuGet.Config file to the new project.

#### `--enable-centrally-managed-packages`

Whether to enable centrally managed packages.

#### `--csharp-lsp`

The C# language server to use.

**Default value**: `OmniSharp`

**Allowed values**: `CsharpLsp`, `OmniSharp`

#### `-f`, `--force`

Force the command to run without prompting for confirmation.

#### `-h`, `--help`

Show help and usage information

#### `--version`

Show version information
