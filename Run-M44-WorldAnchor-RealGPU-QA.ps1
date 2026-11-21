# Aetheria M44: REAL Vulkan regression for screen-space sticker bug.
# Uses a throwaway game home and no existing Horizon process. It never
# changes the player's real game options / worlds / installed Photon.
$ErrorActionPreference='Stop'
$root=$PSScriptRoot
$ctl=Join-Path $env:APPDATA 'horizon-launcher\versions\nightly-2026-11-20-0a875da\horizon-ctl.exe'
$source=Join-Path $root 'aetheria-horizon\shaderpacks\aetheria_spell20'
$out=Join-Path $root 'aetheria-horizon\shaderpacks\m44-worldanchor-qa'
if (!(Test-Path $ctl -PathType Leaf)) {throw 'horizon-ctl missing'}
if (Get-Process horizon-client -ErrorAction SilentlyContinue) {
    throw 'Human game session already running. QA aborted before touching it.'
}
New-Item -ItemType Directory -Force $out|Out-Null
$pack=Join-Path $out 'aetheria_spell20_qa'
if (Test-Path $pack) {Remove-Item $pack -Recurse -Force}
Copy-Item $source $pack -Recurse
$toml=Join-Path $pack 'horizon-shaders.toml'
$s=[IO.File]::ReadAllText($toml).Replace('id = "aetheria_spell20"','id = "aetheria_spell20_qa"')
$pattern='(\[\[options\]\]\s+id = "aetheria_demo"\s+name = "QA: Show one test spell \(0=off\)"\s+type = "int"\s+default = )0'
$replaced=[regex]::Replace($s,$pattern,'${1}7')
if($replaced -eq $s){throw 'QA-only fixed spell option missing; abort'}
[IO.File]::WriteAllText($toml,$replaced)
$env:HORIZON_SHADER_PACK=$pack
$isolation=Join-Path $out 'isolated-game'
New-Item -ItemType Directory -Force $isolation|Out-Null
$script:launched=$false
$script:userActed=$false
function Check-HandsOff {
    $s=(& $ctl status|ConvertFrom-Json)
    if($s.error){throw $s.error}
    if($null -ne $s.input.last_user_age_s -and $s.input.last_user_age_s -lt 6){
        $script:userActed=$true
        throw 'Real human mouse/keyboard activity detected; QA leaves their session untouched.'
    }
    return $s
}
try {
    & $ctl launch --world 'Aetheria-M44-WorldAnchor-QA' --creative --wait --test-terrain --render-distance 5 --width 1280 --height 720 --game-dir $isolation --name M44_Anchor_QA|Out-Null
    if($LASTEXITCODE -ne 0){throw 'Isolated Horizon launch failed'}
    $script:launched=$true
    $first=Check-HandsOff
    Write-Output ('REAL backend '+$first.renderer.backend+' GPU '+$first.renderer.gpu)
    & $ctl tp 0 112 -9|Out-Null
    & $ctl look-at 0 112 0|Out-Null
    Start-Sleep -Milliseconds 250
    $null=Check-HandsOff
    & $ctl screenshot (Join-Path $out 'M44_01_directly_facing_fixed_world.png')|Out-Null
    $null=Check-HandsOff
    & $ctl turn 55 0|Out-Null
    Start-Sleep -Milliseconds 250
    $null=Check-HandsOff
    & $ctl screenshot (Join-Path $out 'M44_02_camera_rotated_55deg_same_world.png')|Out-Null
    $null=Check-HandsOff
    & $ctl turn -55 0|Out-Null
    & $ctl tp 5 112 -9|Out-Null
    Start-Sleep -Milliseconds 250
    $null=Check-HandsOff
    & $ctl screenshot (Join-Path $out 'M44_03_camera_moved_5_blocks_same_world.png')|Out-Null
    [ordered]@{
       backend=$first.renderer.backend;gpu=$first.renderer.gpu
       shader='M44 original shader source, QA cycle enabled in isolated copy only'
       fixed_world_center=[float[]]@(0,112,0)
       shots=@('M44_01_directly_facing_fixed_world.png','M44_02_camera_rotated_55deg_same_world.png','M44_03_camera_moved_5_blocks_same_world.png')
       expected='spell visible only at fixed world position; disappears when turned away; view changes after move'
       visually_accepted=$false
    }|ConvertTo-Json -Depth 4|Set-Content (Join-Path $out 'M44_qa_metadata.json') -Encoding UTF8
    Get-ChildItem $out -Filter 'M44_0*.png'|Select-Object Name,Length
} finally {
    Remove-Item Env:HORIZON_SHADER_PACK -ErrorAction SilentlyContinue
    if($script:launched -and !$script:userActed) {
        & $ctl quit|Out-Null
    }
}
