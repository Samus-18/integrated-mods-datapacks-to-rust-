param(
  [Parameter(Mandatory=$true)][string]$ModPath,
  [string]$OutputRoot = "",
  [switch]$KeepGameOpen
)
$ErrorActionPreference = 'Stop'
$ctl = Join-Path $env:APPDATA 'horizon-launcher\versions\nightly-2026-11-20-0a875da\horizon-ctl.exe'
if (-not (Test-Path -LiteralPath $ctl -PathType Leaf)) { throw 'Official pinned Horizon control CLI missing' }
if (-not (Test-Path -LiteralPath $ModPath -PathType Leaf)) { throw 'CI-certified M40 .hmod missing' }
# Never touch/interrupt a user's already-running game, even for automated QA.
if (Get-Process horizon-client -ErrorAction SilentlyContinue) {
  throw 'Horizon already running. Do NOT hijack a live user session.'
}
if (-not $OutputRoot) { $OutputRoot = Join-Path $PSScriptRoot 'aetheria-horizon\preview-5spells-wasm\target\m40-runtime-qa' }
New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null
$isolated = Join-Path $OutputRoot 'isolated-game'
New-Item -ItemType Directory -Force -Path $isolated | Out-Null
Write-Output 'Launching official real Horizon Vulkan renderer, fresh isolated QA world'
& $ctl launch --world 'Aetheria-M40-Signature-QA' --creative --wait --test-terrain --render-distance 5 --width 1280 --height 720 --dev-mod $ModPath --game-dir $isolated --name VFX_QA
if ($LASTEXITCODE -ne 0) { throw 'Real Horizon game launch failed' }
$userActive = $false
function check_user {
  $state=(& $ctl status | ConvertFrom-Json)
  if ($state.error) { throw "Horizon control error: $($state.error)" }
  if ($null -ne $state.input.last_user_age_s -and $state.input.last_user_age_s -lt 4) {
    $script:userActive = $true
    throw 'User is interacting with the QA game; stop agent control and leave game open'
  }
  return $state
}
try {
  $null=check_user
  # Avoid the M37 failure where effects disappeared behind hills/trees.
  # This is a disposable test world, not the user's existing saves.
  & $ctl tp 0 112 0 | Out-Null
  & $ctl key space tap | Out-Null
  Start-Sleep -Milliseconds 140
  & $ctl key space tap | Out-Null
  & $ctl look 0 -23 | Out-Null
  Start-Sleep -Milliseconds 240
  & $ctl screenshot (Join-Path $OutputRoot 'M40_00_baseline_actual_Horizon.png') | Out-Null
  # R once per school, from PYROMANCER to WILD/DRUID school rank 1.
  for($j=0;$j -lt 5;$j++){ & $ctl key r down | Out-Null; Start-Sleep -Milliseconds 240; & $ctl key r up | Out-Null; Start-Sleep -Milliseconds 220 }
  & $ctl screenshot (Join-Path $OutputRoot 'M40_01_thorn_selected_actual_Horizon.png') | Out-Null
  Write-Output 'CHECK SELECTION: MUST say Thorn Dart WILD 01/20'
  $null=check_user
  & $ctl key z tap | Out-Null
  Start-Sleep -Milliseconds 240
  & $ctl screenshot (Join-Path $OutputRoot 'M40_02_thorn_charge_actual_Horizon.png') | Out-Null
  Start-Sleep -Milliseconds 540
  & $ctl screenshot (Join-Path $OutputRoot 'M40_03_thorn_flight_actual_Horizon.png') | Out-Null
  Start-Sleep -Milliseconds 640
  & $ctl screenshot (Join-Path $OutputRoot 'M40_04_thorn_impact_actual_Horizon.png') | Out-Null
  Start-Sleep -Milliseconds 850
  & $ctl screenshot (Join-Path $OutputRoot 'M40_05_thorn_afterglow_actual_Horizon.png') | Out-Null
  $state=check_user
  $report=[ordered]@{
    engine = 'Actual Horizon Nightly 2026-11-20, Minecraft 26.3'
    renderer = $state.renderer.backend
    gpu = $state.renderer.gpu
    fps_observed_after_cast = $state.fps
    camera = @($state.yaw, $state.pitch)
    position = $state.player.position
    shader_is_beautiful = $false # requires human/visual inspection of actual PNGs
    user_controls_intact = $true
    version = 'M40 0.8.8 INTERNAL'
    screenshot_count = 6
  }
  $report|ConvertTo-Json -Depth 6 |Set-Content -LiteralPath (Join-Path $OutputRoot 'QA_M40_real_game_report.json') -Encoding utf8
  Write-Output ('Actual QA GPU: '+$report.gpu+' / FPS near cast '+$report.fps_observed_after_cast)
  Get-ChildItem -LiteralPath $OutputRoot -Filter 'M40_*.png' | Select-Object Name,Length
}finally{
  if (-not $KeepGameOpen -and -not $userActive) {
    & $ctl quit | Out-Null
  }
}
