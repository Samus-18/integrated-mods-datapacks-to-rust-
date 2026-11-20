# Repository guidance for coding agents

This GitHub repository contains **several independent projects**. Do not assume
the default `main` branch is the newest version of every project.

## Aetheria Horizon (current active magic-mod project)

- **Read first:** `aetheria-horizon/AGENTS.md`, then
  `aetheria-horizon/docs/CODEX_HANDOFF_2026-11-20.md`.
- **Working source branch:** `aetheria-30-spell-test` (M28 as of 2026-11-20).
  `main` is older (M11); do not overwrite newer code with `main` and do not
  claim that an unrelated branch is the live Aetheria source.
- Work within `aetheria-horizon/` and its dedicated
  `.github/workflows/aetheria-*.yml` when working on Aetheria.
  The other projects (Thalvoryn/JJThunder/TREK/Photon, etc.) have independent
  histories and are **out of scope** unless explicitly requested.
- Project owner prefers **Czech progress summaries**, concrete changed code,
  tests and short milestone updates, not repeated suggestions/questions.
- Never label a compilable client VFX demo as a finished playable mod:
  state verified integration, visual quality and real game tests separately.

Read project-local guidance before proposing modifications.
