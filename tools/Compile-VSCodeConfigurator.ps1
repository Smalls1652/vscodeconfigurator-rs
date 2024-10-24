[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [string]$RootDirectory = (Get-Location).Path,
    [Parameter(Position = 1, Mandatory)]
    [ValidateSet(
        "Linux",
        "macOS",
        "Windows"
    )]
    [string]$Platform,
    [Parameter(Position = 2, Mandatory)]
    [ValidateSet(
        "x64",
        "arm64"
    )]
    [string]$Architecture
)

class RustCompilationTarget {
    [string]$Platform
    [string]$Architecture
    [string]$TargetName
    [bool]$UseCross

    RustCompilationTarget([string]$platform, [string]$architecture, [string]$targetName, [bool]$useCross) {
        $this.Platform = $platform
        $this.Architecture = $architecture
        $this.TargetName = $targetName
        $this.UseCross = $useCross
    }
}

$resolvedRootDirectory = (Resolve-Path -Path $RootDirectory -ErrorAction "Stop").Path

if ([System.IO.FileAttributes]::Directory -notin (Get-Item -Path $resolvedRootDirectory).Attributes) {
    $PSCmdlet.ThrowTerminatingError(
        [System.Management.Automation.ErrorRecord]::new(
            [System.Exception]::new("The specified root directory is not a directory."),
            "RootDirectoryIsNotDirectory",
            [System.Management.Automation.ErrorCategory]::InvalidArgument,
            $resolvedRootDirectory
        )
    )
}

$supportedCompilationTargets = @(
    [RustCompilationTarget]::new("Linux", "x64", "x86_64-unknown-linux-gnu", $false),
    [RustCompilationTarget]::new("Linux", "arm64", "aarch64-unknown-linux-gnu", $true),
    [RustCompilationTarget]::new("macOS", "x64", "x86_64-apple-darwin", $false),
    [RustCompilationTarget]::new("macOS", "arm64", "aarch64-apple-darwin", $false),
    [RustCompilationTarget]::new("Windows", "x64", "x86_64-pc-windows-gnu", $false),
    [RustCompilationTarget]::new("Windows", "arm64", "aarch64-pc-windows-gnullvm", $false)
)

$selectedCompilationTarget = $supportedCompilationTargets | Where-Object { $PSItem.Platform -eq $Platform -and $PSItem.Architecture -eq $Architecture }

if ($null -eq $selectedCompilationTarget) {
    $PSCmdlet.ThrowTerminatingError(
        [System.Management.Automation.ErrorRecord]::new(
            [System.Exception]::new("Unsupported platform and architecture combination."),
            "UnsupportedPlatformAndArchitecture",
            [System.Management.Automation.ErrorCategory]::InvalidArgument,
            @($Platform, $Architecture)
        )
    )
}

$artifactsDirectoryPath = Join-Path -Path $resolvedRootDirectory -ChildPath "artifacts"

if (!(Test-Path -Path $artifactsDirectoryPath)) {
    $null = New-Item -Path $artifactsDirectoryPath -ItemType "Directory"
}

$targetArtifactsPath = Join-Path -Path $artifactsDirectoryPath -ChildPath "$($selectedCompilationTarget.Platform.ToLower())-$($selectedCompilationTarget.Architecture.ToLower())"

if (Test-Path -Path $targetArtifactsPath) {
    Remove-Item -Path $targetArtifactsPath -Recurse -Force
}

$null = New-Item -Path $targetArtifactsPath -ItemType "Directory"

switch ($selectedCompilationTarget.UseCross) {
    $true {
        cross build --target "$($selectedCompilationTarget.TargetName)" --release 
        break
    }

    Default {
        cargo build --release --package "vscodeconfigurator" --target "$($selectedCompilationTarget.TargetName)"
        break
    }
}

if (Test-Path -Path $targetArtifactsPath) {
    Remove-Item -Path $targetArtifactsPath -Recurse -Force
}
$null = New-Item -Path $targetArtifactsPath -ItemType "Directory"

$compiledOutputPath = Join-Path -Path $resolvedRootDirectory -ChildPath "target/$($selectedCompilationTarget.TargetName)/release"

$compiledTemplatesDirectoryPath = Join-Path -Path $compiledOutputPath -ChildPath "templates"
$compiledBinaryPath = switch ($selectedCompilationTarget.Platform) {
    "Windows" {
        Join-Path -Path $compiledOutputPath -ChildPath "vscode-configurator.exe"
        break
    }

    Default {
        Join-Path -Path $compiledOutputPath -ChildPath "vscode-configurator"
        break
    }
}

Copy-Item -Path $compiledTemplatesDirectoryPath -Destination $targetArtifactsPath -Recurse -Force
Copy-Item -Path $compiledBinaryPath -Destination $targetArtifactsPath -Force

