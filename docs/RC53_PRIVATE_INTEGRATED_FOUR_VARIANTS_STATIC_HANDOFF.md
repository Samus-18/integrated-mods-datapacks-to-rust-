# RC53 — Private integrated handoff: Java and Horizon Rust × Nether 1024/2032

**9 November 2026. Status: static-validated private handoff. NOT FINISHED / NOT STABLE / NO RUNTIME TESTS.**

The user explicitly requested one immediate complete package instead of further serial preview releases and cannot currently test in Horizon. A concrete, downloadable integrated archive was therefore created from the latest physical RC52 ZIP, while explicitly separating working data-file preservation and static validation from unverified runtime claims.

## Input provenance and exact handoff

- Source RC52 SHA256: `89d65425476ac31d2614732048f71660854e311ca2f2b3d4b0b4ed9233eb9c63`.
- **Private combined archive:** `Thalvoryn_HORIZON_26_3_INTEGRATED_STATIC_HANDOFF_PRIVATE.zip`
- SHA256 **`c9f7e6bf0398739c7c848e4c81f0ae4710e61a276a7402dec1bc35a439389183`**
- Size **39,950,849 bytes**, 15 entries.
- Saved in private ChatGPT Library: `/Horizon/Thalvoryn/INTEGRATED_HANDOFF_2026-11-09/`.
- The archive includes reproducible Python builder, second separately written preflight, machine-readable QA, installation/limitations/readme and GitHub description, as well as prior native Windows Converter v7.2 with its source/audit, and a distinct native WGSL Photon **preview**, explicitly NOT a full Photon 1.4 port.

### Four actual installable ZIPs

| Variant | Bytes | SHA256 |
| --- | ---: | --- |
| `Thalvoryn-JAVA-Nether1024-STATIC-CANDIDATE.zip` | 10,188,422 | `48eed75d4a693d5f4f9b728597a554cd210fdb7dec248926c9cff8eb378b9a0d` |
| `Thalvoryn-JAVA-Nether2032-STATIC-CANDIDATE.zip` | 10,188,478 | `c4ab3baf45f217301c733665521b63fbf9583d0f542ab6168280fd9304fce5e0` |
| `Thalvoryn-RUST-Nether1024-STATIC-CANDIDATE.zip` | 10,188,422 | `c6cd9b199637db45de9c81b2f84d73e4ed4ac55631c8c4b2f5a8b3690f44baec` |
| `Thalvoryn-RUST-Nether2032-STATIC-CANDIDATE.zip` | 10,188,478 | `cc7479f38675641207a5b8325c07272d18048a6a9aa70ea222e1530ec0343466` |

**What is actually different?** Java and Rust use precisely the same 26.3 worldgen data; only human-readable `pack.mcmeta` description differs. Nether1024 uses all RC52 data byte-for-byte except the metadata description. Nether2032 is a *new, **experimental*** vertical adaptation, not a simple dimension-height edit: both relevant noise settings and the Nether dimension type are made 2032; 53 files differ from RC52, covering Y-specific custom Nether density gradients, Nether structure start heights, flower/ore/ship placed features, Nether material rules and relevant vanilla Nether ores. Original lower Y 0–128 is held fixed; above it Y is scaled from original Y1024 to new Y2032. **There has been no generated-chunk test of this profile; appearance, biome density, and frame performance could be wrong.**

## Static QA, independently reopened

- **Each of the four ZIPs:** CRC PASS, no duplicated entries, **1,707 JSON/metadata parse**, **1,378 NBT byte-identical to RC52**, icon preserved, all 36 active structure sets retain valid referenced custom structure IDs, no invalid custom biome→placed-feature IDs, and Nether noise + dimension type have matching heights.
- Source and output normalized verification compares JAVA/RUST variants byte-for-byte except `pack.mcmeta`.
- Second independent preflight verifies **1,707 biome→feature references**, **282 worldgen feature-order edges**, **no detected feature order cycle** and unchanged 1,378 NBT template count per ZIP.
- Verified version **Minecraft Java 26.3 data pack format 121**. Its official server requires Java 25+, while this environment has Java 21; therefore no vanilla 26.3 codec-load test was possible. Horizon Nightly binary/GPU was not available to run.
- **Combined ZIP** independently reopened and all four embedded ZIPs verified by CRC; all four content digests were compared to the files included.

## Critical unfinished work, do not claim otherwise

1. The engine-side full-footprint validator for all original large structures/ships and clearance from trees/terrain/lava is **not supplied**. Unsafe huge imported vessels are preserved as manual-only as documented in RC47/RC52.
2. The user's reported cave walls, naturally generated biome variety and accessible dungeons, 2k Nether noise and high-End behavior have **not been proven with world generation**.
3. Horizon custom biome registration/F3, white-black LOD artifacts, 16k LOD limit, safe Nether portal arrival, real FPS/TPS/VRAM optimization are **not fixed by these ZIPs**.
4. Native WGSL Photon preview is **not the complete source Photon 1.4 port**, and cannot be honestly labeled working without `horizon-shader check` or Vulkan runs; the archived prior `photon_v1.4-RUST.zip` was actually an unchanged GLSL original.
5. The Windows EXE was recovered from an earlier project snapshot; only its *previously reported* automated tests are retained; no new Windows GUI test was executed.
6. End-to-end gameplay, Java/Horizon runtime load, chest-open and player acceptability tests **remain outstanding**. User cannot test at present, no new user test is required now.
7. Imported map rights and proper author attribution must be checked before posting any public binary assets to GitHub. **Only this status documentation is published on GitHub, not the archives.**

**Practical choice for eventual first runtime validation:** Horizon Rust Nether1024 candidate. Do not install multiple variants simultaneously or overwrite existing worlds without backup. See private `START_HERE_CZ.md` and `CO_JESTE_NENI_HOTOVE_CZ.md` bundled with the files.

## Scope status

Earlier percentage was a rough **48%** of total feature/runtime scope. Packaging four static candidates is real progress but does not magically complete the remaining renderer/portal/biome/shader and gaming QA work. No new unverified percentage is labeled "100%".
