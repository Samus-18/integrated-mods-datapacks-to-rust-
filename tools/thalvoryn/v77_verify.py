#!/usr/bin/env python3
"""Independent read-only static comparison of four V76 -> V77 ZIP builds.

Usage: python v77_verify.py --old DIR_WITH_V76 --new DIR_WITH_V77
No game runtime simulated. Never modifies either directory.
"""
from pathlib import Path
import argparse
import hashlib
import json
import re
import zipfile

SHIP = re.compile(r"(^|[/_:.-])(ship(?:s|wreck)?|boat(?:s)?|frigate|pirate|galleon|sail(?:s)?)(?:[/_:.-]|$)", re.I)
VARIANTS = [(p, h) for p in ("RUST", "JAVA") for h in (1024, 2032)]
OLD_DUNGEON = "data/thalvoryn/worldgen/structure_set/ecology/ow_dungeons.json"


def dim(path):
    p = path.lower()
    if "/ecology/end_" in p or "/worldgen/template_pool/end/" in p:
        return "end"
    if "/ecology/nether_" in p or "/worldgen/template_pool/nether/" in p:
        return "nether"
    return ""


def banned(sid, dimension):
    return ((dimension in ("end", "nether") and bool(SHIP.search(sid)))
            or (dimension == "nether" and "overworld_leak" in sid.lower()))


def sha(b):
    return hashlib.sha256(b).hexdigest()


def verify(old_path, new_path, platform, height):
    with zipfile.ZipFile(old_path) as old, zipfile.ZipFile(new_path) as new:
        assert old.testzip() is None, "V76 input CRC failed"
        assert new.testzip() is None, "V77 output CRC failed"
        a, b = set(old.namelist()), set(new.namelist())
        assert "V77_PATCH_REPORT.json" in b
        assert b-a == {"V77_PATCH_REPORT.json"}, "Unexpected new files: " + str(b-a)
        assert not a-b, "V77 deleted resources: " + str(a-b)
        changed = []
        for n in sorted(a):
            ai, bi = old.getinfo(n), new.getinfo(n)
            if ai.CRC == bi.CRC and ai.file_size == bi.file_size:
                continue
            assert old.read(n) != new.read(n), "Inconsistent ZIP metadata"
            changed.append(n)
            if n.endswith(".nbt"):
                raise AssertionError("Original NBT template changed: " + n)
            if "/worldgen/noise" in n or "/worldgen/density_function/" in n or "/worldgen/biome/" in n:
                raise AssertionError("Unexpected terrain or biome change: " + n)
            if n.endswith(".json") and "/worldgen/" in n:
                assert any(s in n for s in ("/structure_set/", "/template_pool/", "/placed_feature/")), (
                    "Unexpected registry JSON change: " + n)
            elif n not in (
                "pack.mcmeta",
                "data/thalvoryn/function/version.mcfunction",
                "data/thalvoryn/function/check.mcfunction",
                "data/thalvoryn/function/help.mcfunction",
                "data/thalvoryn/function/horizon/status.mcfunction",
            ):
                raise AssertionError("Unexpected changed file: " + n)
        assert "pack.mcmeta" in changed
        metadata = json.loads(new.read("pack.mcmeta"))
        assert "V77" in metadata["pack"]["description"]
        assert str(height) in metadata["pack"]["description"]
        assert platform in metadata["pack"]["description"]
        ver = new.read("data/thalvoryn/function/version.mcfunction").decode("utf-8")
        assert "V77" in ver and platform in ver and str(height) in ver
        report = json.loads(new.read("V77_PATCH_REPORT.json"))
        assert report["source"] == old_path.name
        for n in sorted(b):
            if n.endswith(".json"):
                obj = json.loads(new.read(n))
                if "/worldgen/structure_set/" in n and dim(n):
                    assert obj["structures"], "Empty set: " + n
                    for entry in obj["structures"]:
                        assert not banned(entry["structure"], dim(n)), (
                            "Forbidden ship-like structure: " + n + " " + entry["structure"])
                if n == OLD_DUNGEON:
                    placement = obj["placement"]
                    assert placement["spacing"] > placement["separation"]
        original_nbt = sorted(n for n in a if n.endswith(".nbt"))
        assert original_nbt, "No structure NBT in source"
        # Compare digest over the actual bytes, not just counts or manifest text.
        for n in original_nbt:
            assert old.getinfo(n).CRC == new.getinfo(n).CRC
        return {"variant": f"{platform}-{height}", "status": "PASS_STATIC_ONLY",
                "changed": changed, "nbt_unchanged": len(original_nbt),
                "removed_active_ship_entries": len(report["removed_structure_entries"]),
                "disabled_pool_pieces": len(report["disabled_pool_elements"]),
                "geode_placement_files": len(report["rarer_geode_features"])}


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--old", type=Path, required=True)
    ap.add_argument("--new", type=Path, required=True)
    args = ap.parse_args()
    result = {"status": "PASS_STATIC_ONLY", "runtime_tested": False, "variants": []}
    for platform, height in VARIANTS:
        prefix = f"Thalvoryn-{platform}-Nether{height}"
        row = verify(args.old / f"{prefix}-V76-LOOT-QA.zip",
                     args.new / f"{prefix}-V77-DATA-TEST.zip", platform, height)
        result["variants"].append(row)
        print(row["variant"], row["status"], "changed", len(row["changed"]))
    output = args.new / "V77_INDEPENDENT_QA.json"
    output.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("Report:", output)


if __name__ == "__main__":
    main()
