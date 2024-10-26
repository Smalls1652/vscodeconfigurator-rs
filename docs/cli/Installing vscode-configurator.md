# Installing `vscode-configurator`

## Manual install

1. [Download the `.zip` archive of the latest release for your OS and architecture.](https://github.com/Smalls1652/vscodeconfigurator-rs/releases).
2. Create a directory where you want to store it and extract the contents of the `.zip` archive to it.
    * For example, you can create a directory in your user profile/home directory named `.vscode-configurator`.
    * **Ensure that the `templates/` directory and the `vscode-configurator` binary are at the root of the directory you created.**
3. Add the directory you created to your operating system or shell's `PATH` environment variable.

### Additional steps

#### macOS

The pre-compiled binary **is not signed/notarized** by Apple, so, by default, it will be _"quarantined"_. To bypass this you will have to run the following command:

```shell
# Replace /path/to/vscode-configurator to where you extracted the files to.

xattr -r -d com.apple.quarantine /path/to/vscode-configurator
chmod +x /path/to/vscode-configurator
```

#### Linux

You will need to add _"executable"_ permissions to the binary by:

```shell
# Replace /path/to/vscode-configurator to where you extracted the files to.

chmod +x /path/to/vscode-configurator
```

### Adding to `PATH`

> [!WARNING]
> You will need to close and reopen any programs/shells that you're going to run `vscode-configurator` from. **It will persist reboots, so this is just a one-time thing.**

#### macOS/Linux

Add the following line to your shell's profile:

```shell
export PATH="$PATH:/path/to/directory"
```

Be sure to replace `/path/to/directory` with the path to where you extracted the files to. For example, if you put it in a directory named `.vscode-configurator` in your user/home directory:

```shell
export PATH="$PATH:$HOME/.vscodeconfigurator"
```

#### Windows

You run the following command with `powershell.exe` or `pwsh.exe`:

```powershell
[System.Environment]::SetEnvironmentVariable("PATH", "$($env:PATH);C:\path\to\directory", [System.EnvironmentVariableTarget]::User)
```

Be sure to replace `C:\path\to\directory` with the path to where you extracted the files to. For example, if you put it in a directory named `.vscode-configurator` in your user profile directory:

```powershell
[System.Environment]::SetEnvironmentVariable("PATH", "$($env:PATH);$($env:USERPROFILE)\.vscode-configurator", [System.EnvironmentVariableTarget]::User)
```
