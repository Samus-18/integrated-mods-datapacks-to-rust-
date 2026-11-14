#!/usr/bin/env python3
"""Run an actual vanilla Minecraft Java 26.3 dedicated-server worldgen smoke test.

Only uses public release assets (not a user's private world). Does NOT test Horizon
client LOD/rendering. Written for GitHub Actions, standard library only.
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import threading
import time
import urllib.request
import zipfile

VERSION_MANIFEST = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
WORK = Path(".ci-minecraft")
REPORT = WORK / "test-report.json"
LOG = WORK / "server.log"
VERSION = "26.3"
MAX_BUNDLE_BYTES = 120 * 1024 * 1024
MAX_UNCOMPRESSED = 850 * 1024 * 1024


def fetch(url: str) -> bytes:
    request = urllib.request.Request(url, headers={
        "User-Agent": "ThalvorynWorldgenCI/1.0 (GitHub Actions)",
        "Accept": "application/vnd.github+json",
    })
    with urllib.request.urlopen(request, timeout=120) as response:
        return response.read()


def fetch_json(url: str) -> dict:
    return json.loads(fetch(url))


def extract_pack(data: bytes, source: str) -> list[tuple[int, bytes, str]]:
    """Search direct and singly-nested ZIPs for installable datapack roots."""
    found = []
    queue = [(data, source, 0)]
    while queue:
        raw, label, depth = queue.pop()
        if len(raw) > MAX_BUNDLE_BYTES or not zipfile.is_zipfile(io.BytesIO(raw)):
            continue
        with zipfile.ZipFile(io.BytesIO(raw)) as z:
            files = [info for info in z.infolist() if not info.is_dir()]
            names = {info.filename for info in files}
            for name in names:
                if not (name == "pack.mcmeta" or name.endswith("/pack.mcmeta")):
                    continue
                prefix = name[:-len("pack.mcmeta")]
                if not any(p.startswith(prefix + "data/") and p.endswith(".json") for p in names):
                    continue
                relevant = [i for i in files if i.filename.startswith(prefix)]
                if sum(i.file_size for i in relevant) > MAX_UNCOMPRESSED:
                    continue
                out = io.BytesIO()
                with zipfile.ZipFile(out, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=4) as target:
                    for info in relevant:
                        relative = info.filename[len(prefix):]
                        if not relative or relative.startswith("/") or ".." in Path(relative).parts:
                            raise ValueError("Unsafe ZIP member: " + relative)
                        # Avoid scripts and unrelated documentation in the installed pack.
                        if relative not in ("pack.mcmeta", "pack.png") and not relative.startswith(("data/", "assets/")):
                            continue
                        target.writestr(relative, z.read(info))
                score = (100 if "java" in label.lower() else 0)
                score += (25 if "1024" in label.lower() else 0)
                score -= (25 if "2032" in label.lower() else 0)
                found.append((score, out.getvalue(), label + ":" + prefix))
            if depth < 1:
                for info in files:
                    if info.filename.lower().endswith(".zip") and info.file_size < MAX_BUNDLE_BYTES:
                        queue.append((z.read(info), label + "/" + info.filename, depth + 1))
    return found


def get_pack(repo: str, tag: str) -> tuple[bytes, str]:
    release = fetch_json("https://api.github.com/repos/" + repo + "/releases/tags/" + tag)
    candidates = []
    for asset in release.get("assets", []):
        label = asset.get("name", "")
        if not label.lower().endswith(".zip") or "java" not in label.lower():
            continue
        if int(asset.get("size", 0)) > MAX_BUNDLE_BYTES:
            continue
        print("Inspecting published release asset:", label, flush=True)
        candidates.extend(extract_pack(fetch(asset["browser_download_url"]), label))
    if not candidates:
        raise RuntimeError("No installable Java datapack found in public release tag " + tag)
    _, content, name = max(candidates, key=lambda row: row[0])
    print("Selected datapack:", name, "bytes:", len(content), flush=True)
    return content, name


def download_server() -> Path:
    manifest = fetch_json(VERSION_MANIFEST)
    version = next((v for v in manifest["versions"] if v["id"] == VERSION), None)
    if version is None:
        raise RuntimeError("Official Mojang manifest has no " + VERSION)
    meta = fetch_json(version["url"])
    server = meta["downloads"]["server"]
    jar = fetch(server["url"])
    if hashlib.sha1(jar).hexdigest() != server["sha1"]:
        raise RuntimeError("Mojang server.jar SHA1 mismatch")
    destination = WORK / "server.jar"
    destination.write_bytes(jar)
    return destination


def send(process: subprocess.Popen, command: str) -> None:
    if process.poll() is not None:
        raise RuntimeError("Server exited before command: " + command)
    print("SERVER COMMAND:", command, flush=True)
    process.stdin.write(command + "\n")
    process.stdin.flush()


def run_server(server_jar: Path) -> dict:
    server_root = WORK
    (server_root / "eula.txt").write_text("eula=true\n", encoding="utf-8")
    (server_root / "server.properties").write_text(
        "level-name=world\n"
        "level-seed=123456789\n"
        "level-type=minecraft:normal\n"
        "online-mode=false\n"
        "server-ip=127.0.0.1\n"
        "server-port=25589\n"
        "enable-command-block=true\n"
        "spawn-protection=0\n"
        "max-players=1\n"
        "view-distance=4\n"
        "simulation-distance=4\n"
        "difficulty=normal\n"
        "enable-rcon=false\n",
        encoding="utf-8",
    )
    ready = threading.Event()
    reader_done = threading.Event()
    process = subprocess.Popen(
        ["java", "-Xms512M", "-Xmx5G", "-jar", str(server_jar.name), "nogui"],
        cwd=server_root,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
    )

    def read_output() -> None:
        with LOG.open("w", encoding="utf-8") as logfile:
            for line in process.stdout:
                logfile.write(line)
                logfile.flush()
                print(line.rstrip(), flush=True)
                if re.search(r"\bDone \(", line):
                    ready.set()
        reader_done.set()

    threading.Thread(target=read_output, daemon=True).start()
    try:
        deadline = time.monotonic() + 540
        while not ready.is_set() and process.poll() is None and time.monotonic() < deadline:
            time.sleep(2)
        if not ready.is_set():
            raise RuntimeError("Minecraft server never reached Done; check server.log")

        send(process, "datapack list enabled")
        # Actual generation of new chunks across all three dimensions.
        for cmd in [
            "forceload add -16 -16 16 16",
            "execute in minecraft:the_nether run forceload add -16 -16 16 16",
            "execute in minecraft:the_end run forceload add -16 -16 16 16",
        ]:
            send(process, cmd)
            time.sleep(28)
        send(process, "save-all flush")
        time.sleep(18)
        send(process, "stop")
        try:
            process.wait(timeout=100)
        except subprocess.TimeoutExpired:
            raise RuntimeError("Minecraft server failed to stop")
        if process.returncode != 0:
            raise RuntimeError("Minecraft server exited with code " + str(process.returncode))
    finally:
        if process.poll() is None:
            try:
                send(process, "stop")
                process.wait(timeout=40)
            except Exception:
                process.kill()
                process.wait(timeout=20)
        reader_done.wait(timeout=10)

    # Minecraft 26.1+ stores every dimension under world/dimensions.
    # The older world/region, world/DIM-1 and world/DIM1 paths no longer apply.
    dimensions = {
        "overworld": server_root / "world" / "dimensions" / "minecraft" / "overworld" / "region",
        "nether": server_root / "world" / "dimensions" / "minecraft" / "the_nether" / "region",
        "end": server_root / "world" / "dimensions" / "minecraft" / "the_end" / "region",
    }
    region_counts = {
        name: len([f for f in location.glob("*.mca") if f.stat().st_size > 8192])
        for name, location in dimensions.items()
    }
    content = LOG.read_text(encoding="utf-8", errors="replace")
    errors = []
    patterns = [
        "Failed to load data packs", "Errors in currently selected data packs",
        "Feature order cycle", "Unbound values", "Failed to parse",
        "Error loading registry data", "Couldn't load tag",
    ]
    for p in patterns:
        if p.lower() in content.lower():
            errors.append(p)
    if any(count == 0 for count in region_counts.values()):
        errors.append("Missing generated region files in at least one dimension")
    if "file/thalvoryn.zip" not in content.lower():
        errors.append("Server log did not confirm that file/thalvoryn.zip was enabled")
    return {"regions": region_counts, "errors": errors, "passed": not errors}


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--repo", default=os.environ.get("GITHUB_REPOSITORY",
                   "Samus-18/integrated-mods-datapacks-to-rust-"))
    p.add_argument("--tag", default="datapack")
    args = p.parse_args()
    WORK.mkdir(exist_ok=True)
    report = {
        "status": "FAILED",
        "minecraft": VERSION,
        "edition": "Java dedicated server (not Horizon)",
        "source_release_tag": args.tag,
        "repository": args.repo,
        "source_pack": None,
    }
    try:
        pack, source = get_pack(args.repo, args.tag)
        report["source_pack"] = source
        pack_dir = WORK / "world" / "datapacks"
        pack_dir.mkdir(parents=True, exist_ok=True)
        with zipfile.ZipFile(io.BytesIO(pack)) as z:
            meta = json.loads(z.read("pack.mcmeta"))
            if not isinstance(meta, dict) or "pack" not in meta:
                raise RuntimeError("Invalid pack.mcmeta")
        (pack_dir / "thalvoryn.zip").write_bytes(pack)
        report["pack_sha256"] = hashlib.sha256(pack).hexdigest()
        jar = download_server()
        report["server_sha256"] = hashlib.sha256(jar.read_bytes()).hexdigest()
        report.update(run_server(jar))
        report["status"] = "PASS" if report["passed"] else "FAIL"
    except Exception as exc:
        report["error"] = str(exc)
        print("TEST FAILED:", exc, file=sys.stderr, flush=True)
    finally:
        REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        print("REPORT:", json.dumps(report, ensure_ascii=False, indent=2), flush=True)
    sys.exit(0 if report["status"] == "PASS" else 1)


if __name__ == "__main__":
    main()
