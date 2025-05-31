# ��ȡ��ǰ�ű�����Ŀ¼
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path

# ����update�ļ��е�����·��
$updateFolderPath = Join-Path -Path $scriptPath -ChildPath "update"

# ���update�ļ����Ƿ����
if (-not (Test-Path -Path $updateFolderPath -PathType Container)) {
    Write-Host "����: update�ļ��в����ڣ���ȷ��·���Ƿ���ȷ: $updateFolderPath" -ForegroundColor Red
    exit 1
}

# ����ļ�·��
$outputFilePath = Join-Path -Path $scriptPath -ChildPath "update_files_list.txt"

# ����һ��StringBuilder�������洢���
$output = New-Object System.Text.StringBuilder

# ��ӱ����ʱ���
[void]$output.AppendLine("Update�ļ��������嵥")
[void]$output.AppendLine("����ʱ��: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')")
[void]$output.AppendLine("=" * 50)

# ��ȡ������Ŀ¼
$subDirectories = Get-ChildItem -Path $updateFolderPath -Directory

# ���û����Ŀ¼�������ʾ��Ϣ
if ($subDirectories.Count -eq 0) {
    [void]$output.AppendLine("δ�ҵ���Ŀ¼")
} else {
    # ����ÿ����Ŀ¼
    foreach ($dir in $subDirectories) {
        # �����Ŀ¼����
        [void]$output.AppendLine("")
        [void]$output.AppendLine("Ŀ¼: $($dir.Name)")
        [void]$output.AppendLine("-" * 30)
        
        # ��ȡ��Ŀ¼�е������ļ�
        $files = Get-ChildItem -Path $dir.FullName -File
        
        # �����Ŀ¼��û���ļ��������ʾ��Ϣ
        if ($files.Count -eq 0) {
            [void]$output.AppendLine("    (���ļ�)")
        } else {
            # ����ÿ���ļ�����������ƺʹ�С
            foreach ($file in $files) {
                # ��ʽ���ļ���С
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
                
                # ����ļ����ʹ�С
                [void]$output.AppendLine("    $($file.Name) - $formattedSize")
            }
        }
    }
}

# �����д�뵽�ļ�
$output.ToString() | Out-File -FilePath $outputFilePath -Encoding utf8

Write-Host "�ļ��б������ɵ�: $outputFilePath" -ForegroundColor Green