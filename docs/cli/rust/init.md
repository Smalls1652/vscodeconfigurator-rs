# `vscode-configurator rust init`

## Table of Contents

- [`vscode-configurator`](../README.md)
  - [`csharp`](../csharp/README.md)
    - [`init`](../csharp/init.md)
    - [`add`](../csharp/add.md)
  - [`rust`](./README.md)
    - `init`
    - [`add`](./add.md)
  - [`completions`](../completions.md)

## Description

Initialize a new Rust project.

## Usage

```bash
vscode-configurator rust init [options]
```

### Options

#### `-o`, `--output-directory`

The output directory for the new project.

**Default value**: The current working directory.

#### `-n`, `--base-package-name`

The name for the base package.

#### `--base-package-template`

The template for the base package.

**Default value**: `Library`

**Allowed values**: `Binary`, `Library`

#### `-f`, `--force`

Force the command to run without prompting for confirmation.

#### `-h`, `--help`

Show help and usage information

#### `--version`

Show version information
