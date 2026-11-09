# RC60 – source-level height-aware Nether portal fix + optional safe LOD profile

**9 Nov 2026. PRIVATE SOURCE PATCH / 9 synthetic unit tests PASS / NOT COMPILED, NO RUNTIME TEST.**

This is the first substantial **Horizon engine-side** correction package after a long series of static datapack work. The combined private archive still contains the **four original RC59 datapacks unchanged**. Thus the word “RC60” names a new *engine source patch and safety tooling*, **not a newly validated stable datapack or fixed binary client**.

## Actual code-level issue, pinned to the user's Horizon Nightly 2ebf00a

Direct inspection of these two **actual** Rust code files at the user's tested Nightly commit:

- [nether_portal.rs, Nightly 2ebf00a](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-client/src/game/nether_portal.rs)
- [portal_travel.rs, Nightly 2ebf00a](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-world/src/level/portal_travel.rs)

In the first, **two** portal-lookup routines use static section limits:
```rust
let (min_section, max_section) = if to_nether { (0, 15) } else { (-4, 19) };
```
These cover only Nether Y0..255 and Overworld Y-64..319, even though the Thalvoryn datapack uses **Nether Y0..1023 / Y0..2031 and Overworld Y-64..2031**. The actual `closest_portal` function indexes loaded portals by `y >> 4` and filters by this section range. Thus legitimate high-Y portals may never be candidates for selection.

Furthermore, the caller of `PortalForcer.createPortal` always passes hardcoded logical heights:
```rust
let logical = if to_nether { 128 } else { 384 };
```
The world engine's `create_portal` computes `max_placeable = min(world.max_y-1, world.min_y + logical_height - 1)`. Under this code path, new portals may therefore be artificially confined to the low 128/384 dimension layers instead of the high custom dimension specified in the datapack.

**This demonstrates hardcoded height restrictions, but does not PROVE they alone caused the user's portal freeze.** Portal loading, rendering, area readiness and engine state remain outside the test.

## Actual fix delivered

The contained source patcher `ENGINE_RC60/apply_horizon_portal_height.py` makes **only three exact source changes**, guarded by strict literal-pattern counts:
1. Replace **two static section bounds** with `self.world.bottom_y().div_euclid(16)` and `(self.world.top_y() - 1).div_euclid(16)`, respectively.
2. Replace **one hardcoded portal creation logical height** with conditional tall-world dynamic logic: preserve vanilla Nether physical256/logical128 and vanilla Overworld physical384/logical384, but when physical height exceeds these normal profiles, use the dimension's actual physical height. This matches the `logical_height` fields present in the four Thalvoryn datapacks; a modded dimension where logical height differs from its physical height will need a further dimension-metadata-based refinement.
3. Back up the original Rust file before any actual edit; default to **dry-run**. Reject unknown source revisions, already patched text and duplicate backup filenames. This is NOT an in-place update to the user's installed EXE.

Automated tests: **9/9 PASS** (exact three edits; refuses unknown source and repeat edits; original vanilla Nether/Overworld bounds and logic preserved; Thalvoryn 1024, 2032 and Overworld 2096 bounds; misaligned dimensions refused; CLI dry-run/backup/write checks). These are **source-text and arithmetic tests**, not Cargo compilation, thread safety, event-loop or portal regression tests.

| Dimension | Original lookup sections | RC60 targeted sections | Original create logical | RC60 targeted logical |
| --- | --- | --- | --- | --- |
| Vanilla Nether 0..255 | 0..15 | **0..15** | 128 | **128** |
| Nether1024 0..1023 | 0..15 | **0..63** | 128 | **1024** |
| Nether2032 0..2031 | 0..15 | **0..126** | 128 | **2032** |
| Vanilla Overworld -64..319 | -4..19 | **-4..19** | 384 | **384** |
| Thalvoryn Overworld -64..2031 | -4..19 | **-4..126** | 384 | **2096** |

**Potential performance tradeoff:** searching 127 instead of 16 sections across area chunks visits more Y-layers. The existing portal block scan already scans all actual sections. This extra lookup may cost time; it has not been profiled.

## Optional user-setting recovery for very distant LOD

Horizon Nightly `2ebf00a` source [LOD max](https://github.com/horizon-rust/horizon/blob/2ebf00a/crates/horizon-client-render/src/lod.rs) has `MAX_CHUNKS = 1 << 22`. [Official graphics docs](https://github.com/horizon-rust/horizon/blob/2ebf00a/docs/graphics.md) suggest much smaller ranges on limited GPUs, e.g. 256 chunks.

`ENGINE_RC60/Safe-LOD-Horizon.ps1` takes an explicit path to `horizon-graphics.txt`. Default dry-run; `-Apply` backs up then clamps a present `lodDistance` to 256; `-DisableLOD -Apply` disables it. If no LOD line is present, the file is untouched. **This is a resource-use recovery workaround only:** it does not fix LOD stripe rendering, >16k code bugs, other graphics settings or engine GPU logic. The PowerShell script has not been executed in Windows in this environment.

## Independent pack data check and exact file hashes

The RC59 combined source archive was reopened and its **four inner candidates were CRC-verified, SHA256-verified and dimension/noise fields compared**. All 1,378 NBT resources and the exact 19 original RC59 archive entries are **byte-for-byte preserved** in the new handoff. No data pack files were modified in RC60.

- Source RC59 combined SHA256: `94830048cc14bc2fc9791a44710713201318b18527a7235154160df1c1b69335`
- New combined handoff: **`Thalvoryn_RC60_PORTAL_HEIGHT_ENGINE_PATCH_AND_LOD_SAFETY_PRIVATE.zip`**
- Size **40,019,928 bytes**, SHA256 **`da8f4dcc558508ed350d0fcaa2e426cd243f54ee91d28354272e848d2c54e16f`**.
- Contains 4 unchanged RC59 candidate ZIPs, old Converter v7.2, the expressly non-final Photon WGSL preview, old QA source, plus six new source/testing/recovery files under `ENGINE_RC60/` and two external private Library manifest/README entries.
- Independently checked each original candidate: **Nether dimension type `height=logical_height` equals 1024/2032**, matching its `noise_settings/nether.json` height; Overworld height/logical height both 2096. **1,378 NBT members per inner ZIP**. Existing RC59 independent JSON/feature DAG validation remains applicable because no original files changed.

Full private archive and QA files stored in ChatGPT Library: `/Horizon/Thalvoryn/RC60_ENGINE`.

## Known blockers and milestone report

- No Rust toolchain, Horizon Nightly source checkout or GPU runtime was available here. **The patch has NOT been compiled, linked or tested with real gameplay.**
- No Java25/MC26.3 server, live chunk generation, chest roll, measured portal arrival, native F3 biome registration, structural full-footprint guard, true Photon WGSL parity, LOD black/white stripe fix or FPS/TPS benchmark took place.
- The Python source patcher **requires a checkout of the Horizon repo and a rebuilt client**; it is not sufficient to copy the code files into a datapack.
- The user's requested **10% milestones** are respected: the last confirmed qualitative work milestone is **50%**, the 60% milestone is **IN PROGRESS, not earned from a ZIP rename**.
- Do not publish user-imported NBT map assets on GitHub before source copyrights and attribution are resolved. Only this technical document is public.

Next engine goal: compile and test the portal changes on the pinned Nightly with a controlled Y700+ high Nether destination, then improve the renderer LOD transition, large-structure full-footprint placement and the native Photon port. This cannot be represented as 100% completion now.