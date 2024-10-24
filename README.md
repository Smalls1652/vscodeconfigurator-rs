# VSCode Configurator-rs [![Build status](https://github.com/Smalls1652/vscodeconfigurator-rs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/Smalls1652/vscodeconfigurator-rs/actions/workflows/build.yml) [![MIT license](https://badgen.net/static/License/MIT/blue)](./LICENSE)

This is a CLI tool to quickly bootstrap a new project for VSCode. More specifically, it's to reduce the amount of common tasks I perform.

> [!NOTE]
> This is mainly just a tool I'm writing for myself.

## 😵‍💫 **Getting a little deja vu?**

That's because this is actually a _"rewrite"_ of the original [VSCode Configurator](https://github.com/Smalls1652/SmallsOnline.VSCode.Configurator) project that I wrote in `C#`, but this time in `Rust`. I've been wanting to learn how to write Rust for a while and I needed a bit of a starting point to get me acclimated with the language. So why not translate a CLI tool I've written before into Rust?

Will this replace the original project? Don't know yet! We'll see how this goes and if it makes sense once I finish porting over all of the current features.

## 📄 Docs

You can view the docs for installing and using the CLI here:

- [Docs](./docs/README.md)

## 🏗️ Building from source

### 🧰 Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install)
  - The `nightly` toolchain is preferred, but `stable` should work as well.
  - Make sure to have the toolchains for the [target platforms](https://doc.rust-lang.org/nightly/rustc/platform-support.html) you want to build for.

### 🧱 Building

#### Command-line

1. Navigate to the project directory in your terminal.
2. Run the following command to build the project:

```bash
cargo build --package vscodeconfigurator --release --target <TARGET>
```

Replace `<TARGET>` with the desired [target platform](https://doc.rust-lang.org/nightly/rustc/platform-support.html).

#### Command-line (PowerShell)

If you have PowerShell 7 (or higher) installed on Linux, macOS, or Windows, you can do the following:

1. Navigate to the project directory in your terminal.
2. Run the following command to build the project:

```powershell
./tools/Compile-VSCodeConfigurator.ps1 -Platform <PLATFORM> -Architechture <ARCHITECTURE>
```

Make sure to replace the following:

- `<PLATFORM>` with the desired platform (e.g. `Windows`, `Linux`, or `macOS`).
- `<ARCHITECTURE>` with the desired architecture (e.g. `x64` or `arm64`).

## 🗂️ Dependencies used

- [`clap`](https://crates.io/crates/clap)
- [`clap_complete`](https://crates.io/crates/clap_complete)
- [`crossterm`](https://crates.io/crates/crossterm)
- [`serde_json`](https://crates.io/crates/serde_json)

### Build dependencies

- [`git-version`](https://crates.io/crates/git-version)
- [`toml_edit`](https://crates.io/crates/toml_edit)

## 🤝 License

The source code for this project is licensed with the [MIT License](LICENSE).
