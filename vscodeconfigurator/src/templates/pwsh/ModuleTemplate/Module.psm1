$moduleRoot = $PSScriptRoot

$classesDirPath = Join-Path -Path $moduleRoot -ChildPath "Classes"
$functionsDirPath = Join-Path -Path $moduleRoot -ChildPath "Functions"

# ~ Import classes/types ~
if (Test-Path -Path $classesDirPath) {
    $classesToImport = Get-ChildItem -Path $classesDirPath -Filter "*.ps1" -Recurse

    foreach ($class in $classesToImport) {
        . $class.FullName
    }

    # ~~ Export classes/types ~~
    # Add any classes/types you want to export to this array.
    $typesToExport = @()

    # If there are any classes/types to export, add them to the TypeAccelerators
    if ($typesToExport.Count -gt 0) {
        $typeAcceleratorsType = [psobject].Assembly.GetType("System.Management.Automation.TypeAccelerators")
        $existingTypeAccelerators = $typeAcceleratorsType::Get

        foreach ($typeItem in $typesToExport) {
            # Check if the type accelerator already exists.
            if ($typeItem.FullName -in $existingTypeAccelerators.Keys) {
                throw [System.Management.Automation.ErrorRecord]::new(
                    [System.InvalidOperationException]::new("Type accelerator already exists: $($typeItem.FullName)"),
                    "TypeAcceleratorAlreadyExists",
                    [System.Management.Automation.ErrorCategory]::InvalidOperation,
                    $typeItem.FullName
                )
            }

            # Add the type accelerator.
            $typeAcceleratorsType::Add($typeItem.FullName, $typeItem)
        }

        # Remove the type accelerators when the module is removed.
        $MyInvocation.MyCommand.ScriptBlock.Module.OnRemove = {
            $typeAcceleratorsType = [psobject].Assembly.GetType("System.Management.Automation.TypeAccelerators")

            foreach ($typeItem in $typesToExport) {
                $null = $typeAcceleratorsType::Remove($typeItem.FullName)
            }
        }.GetNewClosure()
    }
}

# ~ Import functions ~
if (Test-Path -Path $functionsDirPath) {
    $functionsToImport = Get-ChildItem -Path $functionsDirPath -Filter "*.ps1" -Recurse

    foreach ($function in $functionsToImport) {
        . $function.FullName
    }
}
