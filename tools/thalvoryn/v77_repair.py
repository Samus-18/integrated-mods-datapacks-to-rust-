#!/usr/bin/env python3
"""Thalvoryn V77 cautious worldgen ZIP patcher for private V76 archives.

This is a *data-only patch builder*, not proof that Minecraft/Horizon runs it.
No unlicensed third-party NBT is copied into this repository.
Run: python v77_repair.py --input "folder-with-four-V76-zips" --output "V77-out"
Python 3.10+, standard library only.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
import re
import zipfile
from pathlib import Path

VARIANTS = [(p, h) for p in ("RUST", "JAVA") for h in (1024, 2032)]
SHIP = re.compile(r"(^|[/_:.-])(ship(?:s|wreck)?|boat(?:s)?|frigate|pirate|galleon|sail(?:s)?)(?:[/_:.-]|$)", re.I)
OLD = re.compile(r"\bV(?:6[0-9]|7[0-6])\b")
DIAGNOSTICS = (
    "data/thalvoryn/function/version.mcfunction",
    "data/thalvoryn/function/check.mcfunction",
    "data/thalvoryn/function/help.mcfunction",
    "data/thalvoryn/function/horizon/status.mcfunction",
)
DUNGEONS = "data/thalvoryn/worldgen/structure_set/ecology/ow_dungeons.json"
GEODE_PATH = re.compile(r"/worldgen/(placed_feature|configured_feature)/.*geode", re.I)


def encode(value):
    return (json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode("utf-8")


def dimension(path):
    lower = path.lower()
    # Only touch resources that are explicitly dimension-scoped.
    if "/ecology/end_" in lower or "/worldgen/template_pool/end/" in lower or "/worldgen/structure_set/end_" in lower:
        return "end"
    if "/ecology/nether_" in lower or "/worldgen/template_pool/nether/" in lower or "/worldgen/structure_set/nether_" in lower:
        return "nether"
    return None


def banned_structure(sid, dim):
    if dim not in ("end", "nether"):
        return False
    sid_lower = sid.lower()
    if SHIP.search(sid_lower):
        return True
    return dim == "nether" and "overworld_leak" in sid_lower


def patch_zip(src, target, platform, height):
    changes = {}
    log = {
        "source": src.name, "output": target.name,
        "removed_structure_entries": [],
        "disabled_pool_elements": [],
        "rarer_geode_features": [],
        "dungeon_density": None,
        "remaining_unknown_visual_ships": "Cannot rule out ships hidden in nonspecific NBT names; inspect saved world."
    }

    with zipfile.ZipFile(src, "r") as zin:
        names = set(zin.namelist())
        if "pack.mcmeta" not in names:
            raise ValueError(f"{src}: missing pack.mcmeta")
        if zin.testzip() is not None:
            raise ValueError(f"{src}: input ZIP has CRC errors")
        for entry in zin.infolist():
            n = entry.filename
            if not n.endswith(".json"):
                continue
            if "/worldgen/structure_set/" in n:
                dim = dimension(n)
                if dim not in ("end", "nether"):
                    continue
                obj = json.loads(zin.read(n))
                entries = obj.get("structures", [])
                if not isinstance(entries, list):
                    raise ValueError(f"Bad structure set: {n}")
                removed = [e for e in entries if banned_structure(e.get("structure", ""), dim)]
                if removed:
                    remaining = [e for e in entries if e not in removed]
                    if not remaining:
                        # Avoid creating an invalid empty structure set.
                        raise ValueError(f"{n}: all structure entries banned; requires redesign")
                    obj["structures"] = remaining
                    changes[n] = encode(obj)
                    log["removed_structure_entries"].extend(
                        {"set": n, "structure": e["structure"]} for e in removed
                    )
            elif "/worldgen/template_pool/" in n and dimension(n) in ("end", "nether"):
                obj = json.loads(zin.read(n))
                altered = 0
                for element in obj.get("elements", []):
                    if not isinstance(element, dict):
                        continue
                    piece = element.get("element", {})
                    if not isinstance(piece, dict):
                        continue
                    location = piece.get("location", "")
                    if not isinstance(location, str):
                        continue
                    if SHIP.search(location) or (
                        dimension(n) == "nether" and "overworld_leak" in location.lower()
                    ):
                        element["element"] = {"element_type": "minecraft:empty_pool_element"}
                        altered += 1
                        log["disabled_pool_elements"].append({"pool": n, "location": location})
                if altered:
                    changes[n] = encode(obj)
            elif "/worldgen/placed_feature/" in n:
                obj = json.loads(zin.read(n))
                feature = obj.get("feature", "")
                is_geode = bool(GEODE_PATH.search(n) or
                                (isinstance(feature, str) and "geode" in feature.lower()))
                if not is_geode:
                    continue
                placement = obj.get("placement")
                if not isinstance(placement, list):
                    continue
                # Make geodes moderately rarer; do not erase geodes or alter other ores.
                rarities = [x for x in placement if isinstance(x, dict)
                            and x.get("type") == "minecraft:rarity_filter"]
                if rarities:
                    for rf in rarities:
                        old = rf.get("chance")
                        if isinstance(old, int) and old > 0:
                            rf["chance"] = min(4096, max(24, int(old * 1.75)))
                else:
                    placement.insert(0, {"type": "minecraft:rarity_filter", "chance": 3})
                changes[n] = encode(obj)
                log["rarer_geode_features"].append(n)

        # Extra underground variety, without returning to the previous room spam.
        if DUNGEONS in names:
            obj = json.loads(changes.get(DUNGEONS, zin.read(DUNGEONS)))
            placement = obj.get("placement", {})
            if placement.get("type") == "minecraft:random_spread":
                before = {k: placement.get(k) for k in ("spacing", "separation", "frequency")}
                spacing = placement.get("spacing", 48)
                separation = placement.get("separation", 19)
                if isinstance(spacing, int) and spacing >= 40:
                    placement["spacing"] = 36
                    placement["separation"] = min(int(separation), 18)
                    placement["frequency"] = max(0.60, float(placement.get("frequency", 1.0)))
                    if placement["separation"] >= placement["spacing"]:
                        raise ValueError(f"Bad dungeon random spread in {DUNGEONS}")
                    changes[DUNGEONS] = encode(obj)
                    log["dungeon_density"] = {"from": before, "to": {
                        k: placement.get(k) for k in before
                    }}

        # Diagnostics must agree with the actual ZIP variant.
        meta = json.loads(zin.read("pack.mcmeta"))
        desc = f"Thalvoryn V77 | {platform} | Nether {height} | DATA TEST - not game verified"
        meta["pack"]["description"] = desc
        changes["pack.mcmeta"] = encode(meta)
        for path in DIAGNOSTICS:
            if path not in names:
                continue
            original = zin.read(path).decode("utf-8")
            new = OLD.sub("V77", original)
            if path.endswith("status.mcfunction"):
                new = re.sub(r"0[-–]1023", f"0-{height-1}", new)
                new = re.sub(r"0[-–]2031", f"0-{height-1}", new)
            changes[path] = new.encode("utf-8")
        changes[DIAGNOSTICS[0]] = (
            'tellraw @s {"text":"Thalvoryn V77 | ' + platform +
            ' | Nether ' + str(height) + ' | STATIC TEST","color":"aqua"}\n'
        ).encode("utf-8")
        log["source_sha256"] = hashlib.sha256(src.read_bytes()).hexdigest()
        report_path = "V77_PATCH_REPORT.json"
        if report_path in names:
            raise ValueError(f"{src} already has V77 report")
        changes[report_path] = encode(log)

        target.parent.mkdir(parents=True, exist_ok=True)
        temporary = target.with_suffix(".incomplete")
        try:
            with zipfile.ZipFile(temporary, "w", compression=zipfile.ZIP_DEFLATED,
                                 compresslevel=6, allowZip64=True) as zout:
                for item in zin.infolist():
                    data = changes.pop(item.filename, None)
                    if data is None:
                        data = zin.read(item.filename)
                    zout.writestr(item, data)
                for path, data in sorted(changes.items()):
                    zout.writestr(path, data, compress_type=zipfile.ZIP_DEFLATED)
            with zipfile.ZipFile(temporary) as check:
                bad = check.testzip()
                if bad:
                    raise ValueError(f"Output CRC error: {bad}")
                for p in check.namelist():
                    if p.endswith(".json") and p.startswith("data/"):
                        json.loads(check.read(p))
                for p in check.namelist():
                    if "/worldgen/structure_set/" in p and dimension(p) in ("end", "nether"):
                        obj = json.loads(check.read(p))
                        for entry in obj.get("structures", []):
                            if banned_structure(entry.get("structure", ""), dimension(p)):
                                raise ValueError(f"Forbidden ship start remains: {p}")
            os.replace(temporary, target)
        except Exception:
            temporary.unlink(missing_ok=True)
            raise
    log["output_sha256"] = hashlib.sha256(target.read_bytes()).hexdigest()
    return log


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--input", type=Path, required=True, help="Directory with four V76 ZIPs")
    ap.add_argument("--output", type=Path, required=True, help="Directory for four V77 ZIPs")
    args = ap.parse_args()
    results = []
    for platform, height in VARIANTS:
        stem = f"Thalvoryn-{platform}-Nether{height}"
        src = args.input / f"{stem}-V76-LOOT-QA.zip"
        if not src.is_file():
            raise FileNotFoundError(f"Missing required exact V76 source: {src}")
        target = args.output / f"{stem}-V77-DATA-TEST.zip"
        if target.exists():
            raise FileExistsError(f"Refusing to overwrite: {target}")
        row = patch_zip(src, target, platform, height)
        results.append(row)
        print("BUILT", target.name, "removed starts", len(row["removed_structure_entries"]),
              "disabled pool pieces", len(row["disabled_pool_elements"]))
    path = args.output / "V77_BUILD_MANIFEST.json"
    path.write_bytes(encode({
        "status": "STATIC_PATCH_ONLY_NOT_GAME_TESTED",
        "warning": "No archived world runtime verification. Visual ship-like templates may remain.",
        "variants": results,
    }))
    print("MANIFEST", path)


if __name__ == "__main__":
    main()
