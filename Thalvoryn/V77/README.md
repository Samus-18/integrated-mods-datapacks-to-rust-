# Thalvoryn V77 — WorldFix test package (14 November 2026)

**Status: four private ZIP builds completed and independently STATIC-QA tested. No Horizon or Minecraft V77 gameplay test has been performed.**

V77 was built **directly from the four V76** private ZIPs after forensic inspection of the user's `New World(1).rar` (529 MB) uploaded world. Binaries remain delivered in the original ChatGPT conversation, not uploaded to this public GitHub repo pending third-party structure asset license verification.

## Confirmed in the actual saved world
- End ship is **`thalvoryn:end/handmade/forgotten_platform`**, structure start chunk **(-87, 208)**. Original template was a giant red-sailed wooden vessel (15×44×66 blocks). Replaced its NBT with an original End stone / purpur shrine, and changed its start to surface projection.
- Custom Nether `thalvoryn:nether/imported/buildings_10` starts in chunk **(-106, -29)** at Y≈455. NBT had **1,428 dirt**, **319 grass** and hundreds of overworld leaves. Re-themed eight Nether imported NBTs and nine End imported NBTs by changing palette states only; preserved all original block placements and loot metadata.

## Implemented in V77
- Also replaced `minecraft:end_city/ship.nbt` by a floating End cache (not a boat) including one random treasure chest and one guaranteed Elytra chest.
- Removed two Nether `overworld_leak` candidates and Nether port candidate, without removing Nether fortresses/piglin villages.
- Added Horizon-only terrain guards to 17 imported structures, reducing cliff/floating starts.
- Added **2 unique, procedurally authored Nether peaks with blackstone ruins and random bastion loot**, and **2 open overworld sinkholes, one with waterfall**.
- Reduced vanilla amethyst geode feature chance from 1/24 to **1/58**.
- Raised the encounter rate of large underground structures and cave entrances. Version diagnostics now identify V77.
- Java and Rust both have separate Nether1024 and Nether2032 ZIPs.

## Independent verification
- **4/4 static archive checks PASS**.
- **6,839 NBT templates deep-scanned** on RUST1024; no missing jigsaw connector pools reachable from active structures, 0 issues in supported checks.
- **4/4 full ZIP CRC and JSON/diff verifications PASS**. Exactly 19 existing NBT changed (17 palette-only + 2 reconstructed ships), 4 new NBT created, and unchanged original block coordinates, containers and entities across those imported template rewrites.
- **140 missing connector names remain in inactive source templates**, not made active.
- Minecraft 26.3 GitHub Actions real server smoke test currently runs only public **RC13**; it **has not tested V77**.

**Important:** Existing explored chunks will retain their generated ships / odd structures. Use a fresh world with **only one** V77 ZIP to visually test changes. This is a reproducible test candidate, not a proven zero-error release.