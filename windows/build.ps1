$ErrorActionPreference = 'Stop'
Push-Location (Join-Path $PSScriptRoot '..')
try {
    cargo build --locked --release --bins
    if ($LASTEXITCODE -ne 0) { throw 'Rust build failed' }
    $csc = Join-Path $env:WINDIR 'Microsoft.NET/Framework64/v4.0.30319/csc.exe'
    & $csc /nologo /target:winexe /platform:x64 /optimize+ /reference:System.Windows.Forms.dll /reference:System.Drawing.dll /reference:System.Web.Extensions.dll /out:target\release\Mugoat-Launcher.exe .\windows\Launcher.cs
    if ($LASTEXITCODE -ne 0) { throw 'Launcher build failed' }
    Copy-Item -LiteralPath target/release/muguet.exe -Destination target/release/Mugoat-PC.exe
    Write-Host 'Build complete: target/release/Mugoat-Launcher.exe and Mugoat-PC.exe'
} finally { Pop-Location }
