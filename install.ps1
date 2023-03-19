# Elevate to super user extracted from here: https://superuser.com/a/532109s
param([switch]$Elevated)

function Test-Admin {
    $currentUser = New-Object Security.Principal.WindowsPrincipal $([Security.Principal.WindowsIdentity]::GetCurrent())
    $currentUser.IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator)
}

if ((Test-Admin) -eq $false)  {
    if ($elevated) {
        # tried to elevate, did not work, aborting
    } else {
        Start-Process powershell.exe -Verb RunAs -ArgumentList ('-noprofile -file "{0}" -elevated' -f ($myinvocation.MyCommand.Definition))
    }
    exit
}

$base_url = "https://github.com/ruben69695/hostbeat-cli/releases/download"
$version = "1.0.0"
$file = "x86_64-pc-windows-gnu"
$extension = ".zip"
$from_dir = $PWD.Path

$resource_url = "$base_url/$version/$file$extension"

Write-Host "Hostbeat $version installer"

if (-NOT (Test-Path C:\hostbeat_temp)) {
    New-Item -Path C:\hostbeat_temp -ItemType Directory
}

Set-Location C:\hostbeat_temp

Write-Host "  > ⬇️  Downloading packages..."
Invoke-WebRequest -Uri $resource_url -OutFile $file$extension
Expand-Archive -Path $file$extension -DestinationPath $file

Write-Host "  > ⏳ Installing..."
if (Test-Path C:\Windows\System32\hostbeat.exe) {
    Remove-Item C:\Windows\System32\hostbeat.exe
}
Move-Item "$file\$file\release\hostbeat.exe" C:\Windows\System32

Write-Host "  > 🧹 Cleaning the house..."
Set-Location $from_dir
Remove-Item C:\hostbeat_temp -Recurse

Write-Host "  > 🍺 Installed!"
Write-Host "  > ⚠️  Reopen your terminal before use the hostbeat CLI"