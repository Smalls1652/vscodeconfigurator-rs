[CmdletBinding(SupportsShouldProcess)]
param(
    [Parameter(Position = 0)]
    [string]$RootDirectory = (Get-Location).Path,
    [Parameter(Position = 1, Mandatory)]
    [ValidateSet(
        "vscodeconfigurator"
    )]
    [string]$PackageName = "vscodeconfigurator",
    [Parameter(Position = 2)]
    [string]$CustomVersion
)

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

$newPackageVersion = $null
switch ([string]::IsNullOrWhiteSpace($CustomVersion)) {
    $true {
        $gitDescribeProcInfo = [System.Diagnostics.ProcessStartInfo]::new(
            "git",
            @(
                "describe",
                "--tags",
                "--abbrev=0"
            )
        )

        $gitDescribeProcInfo.WorkingDirectory = $resolvedRootDirectory
        $gitDescribeProcInfo.RedirectStandardOutput = $true

        $gitDescribeProc = [System.Diagnostics.Process]::Start($gitDescribeProcInfo)
        $gitDescribeProc.WaitForExit()

        $gitDescribeProcStandardOutput = $gitDescribeProc.StandardOutput.ReadToEnd()
        $gitDescribeProcStandardOutput = $gitDescribeProcStandardOutput.Trim()

        $newPackageVersion = $gitDescribeProcStandardOutput
        break
    }

    $false {
        $newPackageVersion = $CustomVersion
        break
    }
}

$newPackageVersionTrimmed = $newPackageVersion.TrimStart("v")

$packageDirectory = Join-Path -Path $resolvedRootDirectory -ChildPath $PackageName
$packageManifestPath = Join-Path -Path $packageDirectory -ChildPath "Cargo.toml"

$manifestVersionRegex = [regex]::new("^version = \`"(?'version'.+?)\`"`$", [System.Text.RegularExpressions.RegexOptions]::Multiline)

$packageManifestContent = Get-Content -Path $packageManifestPath -Raw

$manifestVersionMatch = $manifestVersionRegex.Match($packageManifestContent)

if (!$manifestVersionMatch.Success) {
    $PSCmdlet.ThrowTerminatingError(
        [System.Management.Automation.ErrorRecord]::new(
            [System.Exception]::new("Failed to extract the current package version from the manifest."),
            "FailedToExtractPackageVersion",
            [System.Management.Automation.ErrorCategory]::InvalidData,
            $packageManifestPath
        )
    )
}

if ($PSCmdlet.ShouldProcess($PackageName, "Update version to $($newPackageVersion)")) {
    $manifestVersionRegex.Replace($packageManifestContent, "version = `"$($newPackageVersionTrimmed)`"") | Out-File -FilePath $packageManifestPath -Encoding "UTF8" -NoNewline -Force
}
