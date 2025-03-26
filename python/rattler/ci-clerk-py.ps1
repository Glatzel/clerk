param (
    [ValidateSet("develop","release")]
    $config = "develop"
)
$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot

& "$PSScriptRoot/../scripts/pytest.ps1"
& "$PSScriptRoot/../scripts/build-python-whl.ps1" -config $config

Set-Location $PSScriptRoot
pixi run rattler-build build
Set-Location $ROOT
