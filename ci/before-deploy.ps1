# This script takes care of packaging the build artifacts that will go in the
# release zipfile

# todo : check to make sure this works
$SRC_DIR = "$PWD.Path\wright"
$STAGE = [System.Guid]::NewGuid().ToString()

Set-Location $ENV:Temp
New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$ZIP = "$SRC_DIR\$($Env:CRATE_NAME)-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET).zip"

Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\wright.exe" '.\'
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\kittyhawk.exe" '.\'
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\airport.exe" '.\'
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\liftoff.exe" '.\'

7z a "$ZIP" *

Push-AppveyorArtifact "$ZIP"

Remove-Item *.* -Force
Set-Location ..
Remove-Item $STAGE
Set-Location $SRC_DIR
