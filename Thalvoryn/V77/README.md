# Thalvoryn V77 — active worldgen repair candidate

**Status (14 Nov 2026): SOURCE PATCH WRITTEN, NOT EXECUTED.** No V77 ZIP has been built or game-tested. Do **not** label this release as completed.

I recovered access to the actual private filenames `New World(1).rar` and `Thalvoryn-RUST-Nether1024-V76-LOOT-QA.zip` through the conversation file service, but the Python/container execution environment was unavailable in this session. Consequently the world save was **not** parsed and no new bundle was generated.

## Concrete source changes prepared

1. `tools/thalvoryn/v77_repair.py`: operates on the four **actual V76** archive filenames and produces four separate *V77-DATA-TEST* archives when run.
2. Removes ship/boat/frigate/pirate/galleon/sail **structure start entries** from Nether and End `structure_set` resources; also removes `overworld_leak` start entries in Nether.
3. Replaces named ship/boat **jigsaw pool pieces** in End and Nether with valid empty pool elements. Does not rewrite unrelated NBT templates.
4. Reduces custom geode placed-feature attempts where identifiable, while keeping geodes in the world.
5. Moderately increases actual underground dungeon candidate density without reviving the historical small room flood.
6. Regenerates V77 metadata and version diagnostics; preserves other terrain and loot data byte-for-byte.
7. `tools/thalvoryn/v77_verify.py`: independently compares each input V76 archive against the corresponding V77 output and rejects unintended NBT, biome or density/noise changes, invalid JSON, bad ZIP CRC, or remaining visibly named ship structure-set entries.

## How to build once Python filesystem execution works

```sh
python tools/thalvoryn/v77_repair.py --input ./v76 --output ./v77
python tools/thalvoryn/v77_verify.py --old ./v76 --new ./v77
```

Both scripts use Python's standard library.

## Critical unsolved issues

- The large red-sail ship seen at End X≈-1329, Z≈3269 **may have a generic, non-ship name** in an imported structure template. The name-based filter cannot guarantee removing it. The actual saved End chunk and active NBT ancestry must be read before issuing a final pack.
- Screenshot Nether structures at X≈-24 Z≈169, X≈-681 Z≈-317, X≈-1722 Z≈-440 require direct on-disk structure identification before fixing placement and block palettes. NBT edits were **not** performed in V77 source patch.
- Actual chest loot generation, thematic substitutions, worldgen cave entrances, Nether mountains, End atmosphere and LOD mismatch are **not** solved by this patch.
- Do not promise a no-bugs final release or claim 4/4 QA passes until the scripts are executed and a new world is actually tested.
- User-uploaded / third-party binary assets are not copied into public GitHub pending licensing clearance.
