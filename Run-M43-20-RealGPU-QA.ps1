# Runs only a disposable QA instance. NEVER touch a user's live Horizon game.
# The images are REAL Vulkan frames captured by horizon-ctl (NOT AI art).
param(
    [string]$OutputRoot = "",
    [switch]$KeepGameOpen,
    [int]$Samples = 20
)
$ErrorActionPreference = 'Stop'
$Samples = [Math]::Max(1, [Math]::Min(20, $Samples))
$ctl = Join-Path $env:APPDATA 'horizon-launcher\versions\nightly-2026-11-20-0a875da\horizon-ctl.exe'
if (-not (Test-Path -LiteralPath $ctl -PathType Leaf)) { throw 'Pinned official Horizon control CLI not installed' }
if (Get-Process horizon-client -ErrorAction SilentlyContinue) {
    throw 'Existing user Horizon session detected. QA canceled without intervention.'
}
$source = Join-Path $PSScriptRoot 'aetheria-horizon\shaderpacks\aetheria_spell20'
if (-not (Test-Path -LiteralPath (Join-Path $source 'horizon-shaders.toml'))) {
    throw 'M43 spellpack sources missing'
}
if (-not $OutputRoot) { $OutputRoot = Join-Path $PSScriptRoot 'aetheria-horizon\shaderpacks\m43-real-gpu-qa' }
New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null
$testPack = Join-Path $OutputRoot 'aetheria_spell20_qa'
if (Test-Path -LiteralPath $testPack) { Remove-Item -LiteralPath $testPack -Recurse -Force }
Copy-Item -LiteralPath $source -Destination $testPack -Recurse -Force
# Never edit the tracked production pack. The QA copy alone cycles profiles.
$toml = Join-Path $testPack 'horizon-shaders.toml'
$manifest = [IO.File]::ReadAllText($toml)
$manifest = $manifest.Replace('id = "aetheria_spell20"','id = "aetheria_spell20_qa"')
$before = $manifest
$pattern = '(\[\[options\]\]\s+id = "aetheria_cycle"\s+name = "QA: Cycle all 20 spell profiles"\s+type = "bool"\s+default = )false'
$manifest = [regex]::Replace($manifest,$pattern,'${1}true')
if ($manifest -eq $before) {
    throw 'M43 QA cycle option not found; refusing to run a stale shaderpack'
}
[IO.File]::WriteAllText($toml,$manifest)
$isolated = Join-Path $OutputRoot 'isolated-game'
New-Item -ItemType Directory -Force -Path $isolated | Out-Null
$env:HORIZON_SHADER_PACK = $testPack
$script:userActive = $false
$script:launched = $false
function Test-UserInput {
    $state = & $ctl status | ConvertFrom-Json
    if ($state.error) { throw ("Horizon controller error: "+$state.error) }
    if ($null -ne $state.input.last_user_age_s -and $state.input.last_user_age_s -lt 6) {
        $script:userActive = $true
        throw 'User input detected: leave QA game untouched and preserve session.'
    }
    return $state
}
try {
    & $ctl launch --world 'Aetheria-M43-20-PostFX-QA' --creative --wait --test-terrain --render-distance 5 --width 1280 --height 720 --game-dir $isolated --name VFX_QA | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Actual Horizon game failed to start' }
    $script:launched = $true
    $first = Test-UserInput
    Write-Output ('Actual renderer '+$first.renderer.backend+' / GPU '+$first.renderer.gpu)
    $shots = @()
    for ($i=1; $i -le $Samples; $i++) {
        $state = Test-UserInput
        $dest = Join-Path $OutputRoot ('M43_cycle_frame_{0:D2}_actual_Horizon.png' -f $i)
        & $ctl screenshot $dest | Out-Null
        $shots += [ordered]@{
            sample = $i
            file = Split-Path $dest -Leaf
            capture_time = (Get-Date).ToString('o')
            fps = $state.fps
            renderer = $state.renderer.backend
            gpu = $state.renderer.gpu
            # Frame.time isn't exported by horizon-ctl. Never falsely claim
            # that screenshot #i necessarily corresponds to VFX profile #i.
            verified_profile_id = $null
        }
        Start-Sleep -Seconds 4
    }
    [ordered]@{
        engine='Actual Horizon 2026-11-20, MC 26.3'
        api='Horizon WGSL hshader 0.2'
        shader_dev_path=$testPack
        screenshots=$shots
        verdict='ART_NOT_APPROVED_REQUIRES_VISUAL_REVIEW'
    } | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $OutputRoot 'M43_real_gpu_qa.json') -Encoding UTF8
    Get-ChildItem -LiteralPath $OutputRoot -Filter 'M43_cycle_frame_*_actual_Horizon.png' |
        Select-Object Name,Length
} finally {
    Remove-Item Env:HORIZON_SHADER_PACK -ErrorAction SilentlyContinue
    if ($script:launched -and -not $KeepGameOpen -and -not $script:userActive) {
        & $ctl quit | Out-Null
    }
}
