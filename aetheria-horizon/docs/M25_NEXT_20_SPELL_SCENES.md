# M25 — 20 more personally authored Standard spell scenes (2026-11-19)

**Not a release.** The user wants **200 actually beautiful, unique, functioning magic spells**, not 200 palette-swap labels or sticks/rings. The project had 200 registered names and spell intents already. M25 replaces two additional semantic Standard slots per class with separately authored, source-specific procedural 3D world-quad scenes. They are **technical layout drafts** requiring game-real material, lighting and world-mechanic work.

## 20 authored IDs and their actual distinct compositions

| ID | Spell | M25 scene |
|---:|---|---|
| 008 | Lava Bloom | Multilayer unfolding magma flower with 10 large bent petals and a molten center |
| 009 | Coalstorm | Orbiting physically jagged incandescent coal bodies within hot moving fog |
| 028 | Sleet Mirror | Three independently tilted heavy translucent ice mirror slabs with ice support feet |
| 029 | Glacier Lance | Long solid crystalline forward spear with support icicles and snow fins |
| 048 | Twister Seed | Continuous surface funnel tapering to grounded points (not empty hoops) |
| 049 | Aerial Mantle | Substantial mirrored floating feather/wing cape around a torso |
| 068 | Earthbind | Four foot-rooted stone cuffs and fractured ground spires |
| 069 | Pebble Barrage | Forward winding projectile stream of jagged stone bullets with wide air wake |
| 088 | Foam Cascade | Three staggered water walls and distinct spray/crest stones |
| 089 | Abyss Jet | Long, thick, many-sided fluid core with moving wide water membranes and spray |
| 108 | Barkskin | Irregular layered articulated bark armor with shoulders and leaf plates |
| 109 | Bloomstep | Eight full sequential flower path bodies with physical centers and individual petals |
| 128 | Chain Arc | Substantial branching lightning tree mass and folded electric surfaces |
| 129 | Surge Palm | A five-finger articulated electrical hand and four wide outward wave ribbons |
| 148 | Weightless Field | Floating individual feathers and stones carried by pale spatial veils |
| 149 | Nightglass Mirror | Heavy dark obsidian depth-window and thick separated frame shards |
| 168 | Stolen Breath | Distinct displaced hourmark stones and fractured crystal time wedges |
| 169 | Clockwork Ward | Twelve-heavy-toothed floating protective gear with rotating inner hands |
| 188 | Marrow Shard | Long solid angular bone dagger and separated marrow fragments |
| 189 | Epitaph Mark | Arched physical gravestone, carved front hourmark and translucent spirit haze |

### Source

The canonical original spell names, geometry prompts and mechanics are in `docs/memory/SPELL_REGISTRY.md`. M25 code lives in `preview-5spells-wasm/src/standard_combo.rs` and `src/standard_charged2.rs` and is routed in `src/lib.rs` for precisely those 20 IDs, bypassing the prior `mass_forms.rs` generic visual generation.

All geometric scene variants are original Rust code, not reproduced textures/models from reference mods. No server-world mechanic was marked implemented. The 170 expanded spell actions remain deliberately rejected `SpellAction::DesignIntent` until their actual Horizon server adapters exist.

## Actual two-camera appearance review, not AI mockups

We opened the `Aetheria-200Spells-Visual-Audit-v0.6.0-Internal` artifact generated from the exact Rust `render_spell` function in CI, and rendered a side-by-side collection of all 20 M25 Cast shapes, each from TWO distinct oblique viewpoints. This is **accurate CPU geometry**, not actual Horizon GPU screenshots or finished WGSL material output.

The first review exposed specific failures: **Coalstorm** and **Pebble Barrage** looked like heaps of plain boxes, **Barkskin** was a big green block, **Surge Palm** was a spiky fan on a box, **Weightless Field** was an obscure dark bag of cards, **Clockwork Ward** was a block with tiny teeth and **Epitaph Mark** was a flat square with disconnected fragments. **Bloomstep** was already close to the hard 500-face limit and left no headroom for impact details.

Follow-up source revisions: physical jagged rocks and launched crystal fragments; closed overlapping bark armor; a separated hand with only four broad storm ribbons; a more open anti-gravity feather field; a substantially open 12-toothed mechanical shield with three rotating hands; an arched and layered gravestone with ambient spiritual haze; and eight complete detailed flowers instead of nine almost-budget-clipped scenes. The final 20 Cast views were independently reopened and compared. These shape changes are visible from both cameras. They remain rough, simple procedural geometry, not rich cinema-ready magic materials; in particular some stone and time bodies still look too chunky.

## Test evidence

- Final verified source SHA: `feacbc9e3c60164d9ef0cb98026675c84cea4592`.
- [Pin-validated Horizon SDK GitHub Actions run 44617003375](https://github.com/Samus-18/integrated-mods-datapacks-to-rust-/actions/runs/44617003375) — **SUCCESS**.
- **23 preview Rust tests passed / 0 failed**, including new per-ID M25 Combo + Charged finiteness/uniqueness/strict budget tests.
- Existing source-geometry review produces **860 SVG views** (the initial 30 × 3 phases × two cameras and the 170 extended × 2 phases × two cameras), plus an extra **160** Cast/Impact, two-camera numeric regression checks across the 40 specifically authored M24 + M25 new Standard scenes.
- Compiled internal WASM `.hmod`, original 30 OGG school/stage audio cues, custom surface WGSL and SHA512 asset integrity checks passed. All drawn assets are from code, not AI-generated screenshots.

**NOT verified:** client visual beauty on the user's real Horizon build; actual GPU native depth-aware bloom/refraction/terrain response, 60 FPS with RX 580, 200 live spell mechanics, multiplayer, authoritative saves/unlocks, 200 original individual sound identities. Passing 23 tests is NOT artistic approval.

## Current exact development status

| Asset | Exact scope |
|---|---|
| Names and independent design text | **200/200** |
| Portable cast-intent and four-stage timing | **200/200**, but 170 are inactive `DesignIntent` |
| Client-local visual draft | **200/200**, varied quality |
| Independently authored per-ID visual layouts | **150/200** (30 curated + 20 M23 + 20 M24 + 20 M25 + 40 Awakening + 20 Ultimate) |
| Extended Standard semantic rough drafts still to customize | **50/200**, local slots 10–14 for every school |
| Truly playable, polished 200 spell RPG | **NO** |

**Next quality work:** add fully original visual scenes for the final 50 Standards, but prioritize huge meaningful spell silhouettes and implement a verified native Horizon scene-depth/camera GPU renderer for light and atmosphere. The user's original four videos prove that procedural mesh alone is insufficient. Do not ship unfinished user test builds.
