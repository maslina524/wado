$InstallFolder = "C:\Program Files\wado"
$ExePath = "$InstallFolder\wado.exe"
$Repo = "maslina524/wado"
$MaxLength = 30

function Dot-Print {
    param(
        [string]$Message
    )

    Write-Host "$($Message) " -NoNewLine
    for ($i = 0; $i -le $MaxLength - $Message.Length; $i++) {
        Write-Host "." -NoNewLine
    }
    Write-Host " " -NoNewLine
}

function Download-Source {
    param(
        [string]$Repository,
        [string]$Folder,
        [string]$Destination
    )

    Write-Host "Installing the doc directory from GitHub:"
    Invoke-WebRequest -Uri "https://api.github.com/repos/$($Repository)/zipball/main" -OutFile "$($SourcePath).zip"

    Dot-Print "  Extracting archive"
    Expand-Archive -Path "$($SourcePath).zip" -DestinationPath "$($SourcePath)" -Force
    Write-Host "Successfully"

    $SubfolderName = Get-ChildItem -Path "$($SourcePath)" -Directory | Select-Object -ExpandProperty Name

    Dot-Print "  Copying directory"
    Remove-Item -Path $DocPath -Recurse -Force
    Copy-Item -Path "$($SourcePath)\$($SubfolderName)\$($Folder)" -Destination $DocPath -Recurse -Force
    
    Remove-Item -Path "$($SourcePath).zip" -Recurse -Force
    Remove-Item -Path "$($SourcePath)" -Recurse -Force

    Write-Host "Successfully`n"
}

function Download-Exe {
    param(
        [string]$Repository,
        [string]$Folder,
        [string]$Destination
    )

    $Parent = Split-Path $Destination -Parent
    if (-not (Test-Path $Parent)) {
        New-Item -ItemType Directory -Path $Parent -Force | Out-Null
    }

    Write-Host "Installing wado.exe from the latest release:"
    $Response = Invoke-RestMethod "https://api.github.com/repos/$($Repository)/releases/latest"
    $DownloadUrl = ($Response.assets | Where-Object { $_.name -eq "wado.exe" }).browser_download_url

    Dot-Print "  Downloading"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $Destination

    Write-Host "Successfully`n"
}

function AddTo_Path {
    param([string]$Directory)

    Dot-Print "Adding directory to PATH"
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $PathEntries = $UserPath -split ';' | Where-Object { $_ -ne '' } | ForEach-Object { $_.TrimEnd('\') }

    $normalizedDir = $Directory.TrimEnd('\')

    if ($PathEntries -contains $normalizedDir) {
        Write-Host "Already"
    } else {
        try {
            $newPath = $UserPath + ";" + $Directory
            [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
            Write-Host "Successfully"
            Write-Host "Please restart your terminal to use Wado."
        } catch {
            Write-Host "Failed to update PATH: $($_.Exception.Message)" -ForegroundColor Red
        }
    }
}

Download-Exe -Repository $Repo -Destination $ExePath
AddTo_Path -Directory $InstallFolder

Read-Host -Prompt "Press any key to continue..."