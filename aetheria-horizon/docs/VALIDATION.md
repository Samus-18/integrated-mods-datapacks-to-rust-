# Aetheria Horizon — validation ledger

| Milestone | Verification | Evidence | Scope |
| --- | --- | --- | --- |
| M0 | 19 Rust unit tests passed | [GitHub Actions 44394835431](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44394835431) | Portable Rust logic only |
| M1 | 28 Rust unit tests passed | [GitHub Actions 44395652564](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44395652564) | Mana, casting, spellbook, reactions |
| M2 | **33 passed / 0 failed** | [GitHub Actions 44397904415](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44397904415), code commit `2016ec34013ed97970e1e3ef1984be441b25163d` | 20 portable action intents, timelines, invalid-input guards |


| M3 | **39 passed / 0 failed** | [GitHub Actions 44482049153](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44482049153), commit `cec3131b3d9b2a13de8985a49d6276c5d8aafe02` | 20 distinct 3D VFX instance silhouettes; fair per-frame budgets and guard tests |

Command reproduced in CI:

```sh
cd aetheria-horizon
cargo test --all-targets
```

Initial M2 run 44397753631 failed **one legacy unit test** checking that spell ID 2 was unavailable. ID 2 (Cinder Fan) became implemented in M2, so the test now asserts on ID 3 (still design-only); subsequent run passed all 33 tests.

## Validation **not** performed yet

- Native Horizon launcher SDK compilation, manifest validation or playable `.hmod` build
- WGSL validation with Naga or GPU render integration
- Multiplayer/server-authoritative target, world saves, rollback or networking
- Actual VFX/audio/UI in a Minecraft 26.3 Horizon world
- VRAM/performance budgets or visual equivalence to the four supplied video clips

The word **implemented** in project documentation means the standalone Rust **action intent**, not working in-game magic.


## M27–M28 internal pinned Horizon branch (2026-11-19)

This section supersedes the old validation ledger's broad 'not yet compiled' statements for the **precompiled WASM preview only**. It does NOT supersede their separate warnings about real server/world integration and native GPU HDR.

| Milestone | Verified | Evidence | Still unverified |
| --- | --- | --- | --- |
| M27 | 28/28 preview Rust tests, 200/200 distinct scene routes, 860 source 3D SVG diagnostics, pinned Horizon WASM .hmod/SHA512/30 OGG | [Actions 44629836762](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44629836762) | User game appearance, depth effects, world damage/network, coherent GPU silhouettes |
| M28 | **31/31 preview Rust tests**, three new fair-quad-allocation tests, full source and 120 M27 2-view gates, valid pinned WASM .hmod/OGG/WGSL/ZIP/SHA512 | [Actions 44632013016](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44632013016) | Real FPS, visually coherent LOD under many casts, native GPU postpass, 170 actual world mechanics |
| M28 portable Rust + Rapier | Both jobs completed SUCCESS | [Actions 44632013002](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44632013002) | Collision physics wired to Horizon block/entity world |

**Counting policy:** 200/200 authored client scene identities are complete as renderer routes; 0/100 user-accepted fully integrated RPG spells are verified. Shader Naga compilation and 860 CPU SVG geometry snapshots are not Minecraft in-game screenshots or aesthetic sign-off. Final user-facing package is withheld.
