# Get the directory of the current script
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path

# Build the full path of the update folder
$updateFolderPath = Join-Path -Path $scriptPath -ChildPath "update"

# Check if the update folder exists
if (-not (Test-Path -Path $updateFolderPath -PathType Container)) {
    Write-Host "Error: update folder does not exist, please confirm the path is correct: $updateFolderPath" -ForegroundColor Red
    exit 1
}

# Output file path
$outputFilePath = Join-Path -Path $scriptPath -ChildPath "update_files_list.txt"

# Create a StringBuilder object to store the result
$output = New-Object System.Text.StringBuilder

    # Add title and timestamp
[void]$output.AppendLine("Update File List")
[void]$output.AppendLine("Make Time: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')")
[void]$output.AppendLine("=" * 50)

# Get all subdirectories
$subDirectories = Get-ChildItem -Path $updateFolderPath -Directory

# If there are no subdirectories, output a prompt message
if ($subDirectories.Count -eq 0) {
    [void]$output.AppendLine("No subdirectories found")
} else {
    # Sort subdirectories by version number
    $sortedDirectories = $subDirectories | ForEach-Object {
        # Extract version from directory name (format: 0.1.0-2025-05-28-12-19-04)
        if ($_.Name -match '^(\d+)\.(\d+)\.(\d+)') {
            $major = [int]$matches[1]
            $minor = [int]$matches[2]
            $patch = [int]$matches[3]
            
            # Create a custom object with version components for sorting
            [PSCustomObject]@{
                Directory = $_
                Major = $major
                Minor = $minor
                Patch = $patch
                FullName = $_.Name
            }
        } else {
            # If directory name doesn't match version pattern, put it at the end
            [PSCustomObject]@{
                Directory = $_
                Major = [int]::MaxValue
                Minor = [int]::MaxValue
                Patch = [int]::MaxValue
                FullName = $_.Name
            }
        }
    } | Sort-Object Major, Minor, Patch
    
    # Iterate through each sorted subdirectory
    foreach ($dirInfo in $sortedDirectories) {
        $dir = $dirInfo.Directory
        # Add subdirectory name
        [void]$output.AppendLine("")
        [void]$output.AppendLine("Directory: $($dir.Name)")
        [void]$output.AppendLine("-" * 30)
        
        # Get all files in the subdirectory
        $files = Get-ChildItem -Path $dir.FullName -File
        
        # If there are no files in the subdirectory, output a prompt message
        if ($files.Count -eq 0) {
            [void]$output.AppendLine("    (No files)")
        } else {
            # Iterate through each file and output its name and size
            foreach ($file in $files) {
                # Format file size
                $sizeInBytes = $file.Length
                $formattedSize = if ($sizeInBytes -ge 1GB) {
                    "{0:N2} GB" -f ($sizeInBytes / 1GB)
                } elseif ($sizeInBytes -ge 1MB) {
                    "{0:N2} MB ({1:N2} KB)" -f ($sizeInBytes / 1MB), ($sizeInBytes / 1KB)
                } elseif ($sizeInBytes -ge 1KB) {
                    "{0:N2} KB" -f ($sizeInBytes / 1KB)
                } else {
                    "$sizeInBytes B"
                }
                
                # Add file name and size
                [void]$output.AppendLine("    $($file.Name) - $formattedSize")
            }
        }
    }
}

# To File
$output.ToString() | Out-File -FilePath $outputFilePath -Encoding utf8

Write-Host "File list has been generated to: $outputFilePath" -ForegroundColor Green