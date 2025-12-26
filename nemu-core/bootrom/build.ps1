if (!(Test-Path -Path ".\build")) {
    New-Item -ItemType Directory -Path ".\build" | Out-Null
}

rgbasm -o build\dmg_boot.o dmg_boot.asm
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

rgblink -x -o build\dmg_boot.bin build\dmg_boot.o
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Build succeeded. Output: .\build\dmg_boot.bin"
