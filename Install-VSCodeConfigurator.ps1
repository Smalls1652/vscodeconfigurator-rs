[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet(
        "Release",
        "Debug"
    )]
    [string]$Configuration = "Release"
)

$artifactPath = Join-Path -Path $PSScriptRoot -ChildPath "target/$($Configuration.ToLower())/vscodeconfigurator"
$templatesArtifactPath = Join-Path -Path $PSScriptRoot -ChildPath "target/$($Configuration.ToLower())/templates"
$installPath = Join-Path -Path ([System.Environment]::GetFolderPath([System.Environment+SpecialFolder]::UserProfile)) -ChildPath ".vscodeconfigurator/bin/"

if (!(Test-Path -Path $artifactPath)) {
    $PSCmdlet.ThrowTerminatingError(
        [System.Management.Automation.ErrorRecord]::new(
            [System.IO.FileNotFoundException]::new("Built executable not found at '$($artifactPath)'."),
            "InstallerNotFound",
            [System.Management.Automation.ErrorCategory]::ObjectNotFound,
            $artifactPath
        )
    )
}

if (!(Test-Path -Path $installPath)) {
    Write-Warning "Creating install directory at: $($installPath)"
    $null = New-Item -Path $installPath -ItemType "Directory" -Force
}

Write-Verbose "Copying 'vscode-configurator' to: $($installPath)"
Copy-Item -Path $artifactPath -Destination $installPath -Force -Verbose:$false

Write-Verbose "Copying templates to: $($installPath)"
Copy-Item -Path $templatesArtifactPath -Destination $installPath -Recurse -Force -Verbose:$false
