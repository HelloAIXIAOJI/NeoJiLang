# 获取当前脚本所在目录
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path

# 构建update文件夹的完整路径
$updateFolderPath = Join-Path -Path $scriptPath -ChildPath "update"

# 检查update文件夹是否存在
if (-not (Test-Path -Path $updateFolderPath -PathType Container)) {
    Write-Host "错误: update文件夹不存在，请确认路径是否正确: $updateFolderPath" -ForegroundColor Red
    exit 1
}

# 输出文件路径
$outputFilePath = Join-Path -Path $scriptPath -ChildPath "update_files_list.txt"

# 创建一个StringBuilder对象来存储结果
$output = New-Object System.Text.StringBuilder

# 添加标题和时间戳
[void]$output.AppendLine("Update文件夹内容清单")
[void]$output.AppendLine("生成时间: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')")
[void]$output.AppendLine("=" * 50)

# 获取所有子目录
$subDirectories = Get-ChildItem -Path $updateFolderPath -Directory

# 如果没有子目录，输出提示信息
if ($subDirectories.Count -eq 0) {
    [void]$output.AppendLine("未找到子目录")
} else {
    # 遍历每个子目录
    foreach ($dir in $subDirectories) {
        # 添加子目录名称
        [void]$output.AppendLine("")
        [void]$output.AppendLine("目录: $($dir.Name)")
        [void]$output.AppendLine("-" * 30)
        
        # 获取子目录中的所有文件
        $files = Get-ChildItem -Path $dir.FullName -File
        
        # 如果子目录中没有文件，输出提示信息
        if ($files.Count -eq 0) {
            [void]$output.AppendLine("    (无文件)")
        } else {
            # 遍历每个文件并输出其名称和大小
            foreach ($file in $files) {
                # 格式化文件大小
                $sizeInBytes = $file.Length
                $formattedSize = if ($sizeInBytes -ge 1GB) {
                    "{0:N2} GB" -f ($sizeInBytes / 1GB)
                } elseif ($sizeInBytes -ge 1MB) {
                    "{0:N2} MB" -f ($sizeInBytes / 1MB)
                } elseif ($sizeInBytes -ge 1KB) {
                    "{0:N2} KB" -f ($sizeInBytes / 1KB)
                } else {
                    "$sizeInBytes B"
                }
                
                # 添加文件名和大小
                [void]$output.AppendLine("    $($file.Name) - $formattedSize")
            }
        }
    }
}

# 将结果写入到文件
$output.ToString() | Out-File -FilePath $outputFilePath -Encoding utf8

Write-Host "文件列表已生成到: $outputFilePath" -ForegroundColor Green