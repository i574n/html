param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. deps/spiral/scripts/core.ps1
. deps/spiral/lib/spiral/lib.ps1

$notebooks = @(Get-ChildItem src -Filter *.spi | ForEach-Object { '--path'; $_.FullName })
$env:SPIRAL_CELLS_MEMO = '0'
Push-Location (Join-Path 'deps' 'spiral' 'apps' 'cells')
try { $cells = @(& pwsh -NoProfile -File (Join-Path 'spi' 'run_notebook.ps1') --gate @notebooks 2>&1 | ForEach-Object { "$_" }); $cellsExit = $LASTEXITCODE }
finally { Pop-Location }
$cells | ForEach-Object { Write-Output "build.ps1 / cells / $_" }
if ($cellsExit -ne 0) { throw "build.ps1 / Spiral test cells exited $cellsExit" }
if (!($cells -match '^spiral-tests total')) { throw "build.ps1 / Spiral test cells printed no spiral-tests total line" }

if (!(BuildSpiral markdown/entry.spi markdown/entry.rs "html/markdown")) {
    throw "RUST-FAILED html/markdown / compile"
}
if (!(BuildSpiral cli/html.spi cli/html.rs "html/cli")) {
    throw "RUST-FAILED html/cli / compile"
}
{ cargo +nightly-2026-10-18 build --release } | Invoke-Block
$cargoTarget = (cargo +nightly-2026-10-18 metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory
if (!$cargoTarget) { throw "RUST-FAILED html / cargo metadata found no target directory" }

Remove-Item dist -Recurse -Force -ErrorAction Ignore
New-Item -ItemType Directory -Force -Path dist | Out-Null
Copy-Item -Force (Join-Path $cargoTarget 'release' "html$(_exe)") (Join-Path 'dist' "html$(_exe)")
{ & (Join-Path 'dist' "html$(_exe)") --self-test } | Invoke-Block

$generated = @(Get-ChildItem cli, markdown -Filter *.rs | ForEach-Object { [pscustomobject]@{ Path = $_.FullName; Lines = @(Get-Content $_.FullName).Count } } | Sort-Object Lines -Descending)
Write-Output "build.ps1 / largest generated file / $($generated[0].Lines) lines / $($generated[0].Path)"
if ($generated[0].Lines -gt 1000) { throw "build.ps1 / 1000-line gate: $($generated[0].Path) has $($generated[0].Lines) lines" }
Write-Output "HTML-OK"
