# `vscode-configurator csharp add`

## Table of Contents

- [`vscode-configurator`](../README.md)
  - [`csharp`](./README.md)
    - [`init`](./init.md)
    - `add`
  - [`rust`](../rust/README.md)
    - [`init`](../rust/init.md)
    - [`add`](../rust/add.md)
  - [`completions`](../completions.md)

## Description

Add a new C# project to the workspace.

## Usage

```bash
vscode-configurator csharp add-project [options]
```

### Options

#### `--solution-file-path`

The solution file to add the project to.

**Default value**: Uses the solution file in the current working directory.

> ⚠️ **Note:**
>
> If multiple solution files are found, this option will have to be provided.

#### `--project-path` **(Required)**

The path to the project.

#### `--project-friendly-name`

The friendly name of the project.

**Default value**: The name of the directory supplied in `--project-path`.

#### `--is-runnable`

Whether the project is runnable.

#### `--is-watchable`

Whether the project is watchable.

#### `-h`, `--help`

Show help and usage information

#### `--version`

Show version information
