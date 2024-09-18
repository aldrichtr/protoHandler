param(
    [Parameter()]
    [ValidateSet('release', 'debug')]
    [string]$Configuration = 'debug'
)


task Install {
    $destination = "$env:LOCALAPPDATA/protoHandler"
    $outputDir = "$BUILDROOT/artifacts/bin/protoHandler"
    $source = Join-Path -Path $outputDir -ChildPath $Configuration

    Write-Build Cyan "Installing protoHandler to $destination"

    Write-Build Cyan "- First removing current content"

    Remove-Item -Path "$destination/*" -Recurse -Force

    Write-Build Cyan "- Second copying new content"
    Copy-Item -Path "$source/*" -Destination $destination -Recurse

    Write-Build Cyan "- Creating directories"

    New-Item "$destination/logs" -ItemType Directory
    New-Item "$destination/scripts" -ItemType Directory

    Write-Build Cyan "- Copying script"
    Copy-Item "$HOME/.pwsh/scripts/capture.ps1" -Destination "$destination/scripts"

    $settings = @{
        LogFile = "$env:LOCALAPPDATA/protoHandler/logs/protoHandler.log"
        ScriptPath = "$env:LOCALAPPDATA/protoHandler/scripts/capture.ps1"
    }

    Write-Build Cyan "- Creating settings"

    $settings | ConvertTo-Json | Set-Content "$destination/protohandler.json"
}

task Build {
    Write-Build Cyan "Building the Application"
    dotnet build --configuration release --self-contained true
}

task . Build, Install
