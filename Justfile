set shell := ["pwsh.exe", "-c"]

build:
    cargo build --release -j 12
    @New-Item -ItemType Directory -Force -Path G:\Dx\bin | Out-Null
    @Copy-Item target\release\icon.exe G:\Dx\bin\dx-icon.exe -Force
    @$cacheBin = Join-Path $env:LOCALAPPDATA "dx\bin"
    @New-Item -ItemType Directory -Force -Path $cacheBin | Out-Null
    @Copy-Item target\release\icon.exe (Join-Path $cacheBin "dx-icon.exe") -Force






