param(
  [Parameter(Mandatory=$true)][string]$ModPath,
  [string]$OutputRoot = "",
  [switch]$KeepGameOpen
)
$ErrorActionPreference = 'Stop'
$ctl = Join-Path $env:APPDATA 'horizon-launcher\versions\nightly-2026-11-20-0a875da\horizon-ctl.exe'
if (-not (Test-Path -LiteralPath $ctl -PathType Leaf)) { throw 'Official pinned Horizon control CLI missing' }
if (-not (Test-Path -LiteralPath $ModPath -PathType Leaf)) { throw 'CI-certified M41 .hmod missing' }
# Never touch/interrupt a user's already-running game, even for automated QA.
if (Get-Process horizon-client -ErrorAction SilentlyContinue) {
  throw 'Horizon already running. Do NOT hijack a live user session.'
}
if (-not $OutputRoot) { $OutputRoot = Join-Path $PSScriptRoot 'aetheria-horizon\preview-5spells-wasm\target\m41-runtime-qa' }
New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null
$isolated = Join-Path $OutputRoot 'isolated-game'
New-Item -ItemType Directory -Force -Path $isolated | Out-Null
Write-Output 'Launching official real Horizon Vulkan renderer, fresh isolated QA world'
& $ctl launch --world 'Aetheria-M41-Signature-QA' --creative --wait --test-terrain --render-distance 5 --width 1280 --height 720 --dev-mod $ModPath --game-dir $isolated --name VFX_QA
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
  & $ctl screenshot (Join-Path $OutputRoot 'M41_00_baseline_actual_Horizon.png') | Out-Null
  # R once per school, from PYROMANCER to WILD/DRUID school rank 1.
  for($j=0;$j -lt 5;$j++){ & $ctl key r down | Out-Null; Start-Sleep -Milliseconds 240; & $ctl key r up | Out-Null; Start-Sleep -Milliseconds 220 }
  & $ctl screenshot (Join-Path $OutputRoot 'M41_01_thorn_selected_actual_Horizon.png') | Out-Null
  Write-Output 'CHECK SELECTION: MUST say Thorn Dart WILD 01/20'
  $null=check_user
  Start-Sleep -Milliseconds 900 # let school wheel vanish before the actual visual capture
  & $ctl key z tap | Out-Null
  Start-Sleep -Milliseconds 240
  & $ctl screenshot (Join-Path $OutputRoot 'M41_02_thorn_charge_actual_Horizon.png') | Out-Null
  Start-Sleep -Milliseconds 540
  & $ctl screenshot (Join-Path $OutputRoot 'M41_03_thorn_flight_actual_Horizon.png') | Out-Null
  Start-Sleep -Milliseconds 640
  & $ctl screenshot (Join-Path $OutputRoot 'M41_04_thorn_impact_actual_Horizon.png') | Out-Null
  Start-Sleep -Milliseconds 1500
  & $ctl screenshot (Join-Path $OutputRoot 'M41_05_thorn_afterglow_actual_Horizon.png') | Out-Null
  # M41 also changes material overlays for 100 named drafted spells.
  # Actual per-school screenshots MUST sample non-SDF named routes rather
  # than only the specialized Thorn Dart and SDF signatures.
  $sampling=@(
    @{ name='Spark Fork'; file='M41_06_spark_fork_actual_Horizon.png' },
    @{ name='Star Shard'; file='M41_07_star_shard_actual_Horizon.png' },
    @{ name='Second Hand'; file='M41_08_second_hand_actual_Horizon.png' },
    @{ name='Bone Thorn'; file='M41_09_bone_thorn_actual_Horizon.png' }
  )
  foreach($sample in $sampling) {
    $null=check_user
    & $ctl key r down | Out-Null
    Start-Sleep -Milliseconds 240
    & $ctl key r up | Out-Null
    Start-Sleep -Milliseconds 850 # collapse school wheel
    & $ctl key z tap | Out-Null
    Start-Sleep -Milliseconds 920
    & $ctl screenshot (Join-Path $OutputRoot $sample.file) | Out-Null
    Write-Output ('Actual M41 material sample: '+$sample.name)
  }
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
    version = 'M41 0.8.9 INTERNAL'
    screenshot_count = 10
  }
  $report|ConvertTo-Json -Depth 6 |Set-Content -LiteralPath (Join-Path $OutputRoot 'QA_M41_real_game_report.json') -Encoding utf8
  Write-Output ('Actual QA GPU: '+$report.gpu+' / FPS near cast '+$report.fps_observed_after_cast)
  Get-ChildItem -LiteralPath $OutputRoot -Filter 'M41_*.png' | Select-Object Name,Length
}finally{
  if (-not $KeepGameOpen -and -not $userActive) {
    & $ctl quit | Out-Null
  }
}
