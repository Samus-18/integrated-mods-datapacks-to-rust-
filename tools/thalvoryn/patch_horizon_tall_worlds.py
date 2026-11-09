#!/usr/bin/env python3
"""Backup-first source-only fix for Horizon Nightly 2ebf00a tall dimensions.

No installed EXE is changed. Requires building Horizon with its Rust toolchain.
Source literal matchers deliberately refuse unknown revisions / previous patches.
"""
from __future__ import annotations
import argparse
import hashlib
import pathlib
import sys

PORTAL = pathlib.Path('crates/horizon-client/src/game/nether_portal.rs')
RAIDS = pathlib.Path('crates/horizon-client/src/game/raids.rs')

SECTIONS_OLD = 'let (min_section, max_section) = if to_nether { (0, 15) } else { (-4, 19) };'
SECTIONS_NEW = '''let min_section = self.world.bottom_y().div_euclid(16);
        let max_section = (self.world.top_y() - 1).div_euclid(16);'''
LOGICAL_OLD = 'let logical = if to_nether { 128 } else { 384 };'
LOGICAL_NEW = '''// Keep the vanilla logical-height cutoff in vanilla-height dimensions.
        // The Thalvoryn dimension types set logical_height == physical height.
        let physical_height = self.world.top_y().saturating_sub(self.world.bottom_y());
        let logical = if to_nether {
            if physical_height > 256 { physical_height } else { 128 }
        } else if physical_height > 384 {
            physical_height
        } else {
            384
        };'''
RAIDS_OLD = 'let top = if self.dim == Dim::Nether { 127 } else { 319 };'
RAIDS_NEW = '''// Use the actual height for custom tall dimensions.
        // Vanilla Nether historically restricts this world-surface helper to Y127.
        let physical_height = self.world.top_y().saturating_sub(self.world.bottom_y());
        let top = if self.dim == Dim::Nether && physical_height <= 256 {
            127
        } else {
            self.world.top_y() - 1
        };'''


def patch_portal(s: str) -> str:
    if s.count(SECTIONS_OLD) != 2 or s.count(LOGICAL_OLD) != 1:
        raise ValueError('Portal source mismatch or already patched; no change')
    if s.count(SECTIONS_NEW) or s.count(LOGICAL_NEW):
        raise ValueError('Portal already contains new code; no change')
    out = s.replace(SECTIONS_OLD, SECTIONS_NEW).replace(LOGICAL_OLD, LOGICAL_NEW)
    assert out.count(SECTIONS_NEW) == 2 and out.count(LOGICAL_NEW) == 1
    return out


def patch_raids(s: str) -> str:
    if s.count(RAIDS_OLD) != 1 or s.count(RAIDS_NEW):
        raise ValueError('Raid surface_height source mismatch or already patched; no change')
    # Narrow the occurrence to the expected function to guard against a coincidental match.
    if 'pub(super) fn surface_height(&self, x: i32, z: i32) -> i32 {' not in s:
        raise ValueError('Expected surface-height method is absent')
    return s.replace(RAIDS_OLD, RAIDS_NEW)


def profile(min_y: int, height: int, to_nether: bool) -> dict[str, int]:
    if min_y % 16 or height < 16 or height % 16:
        raise ValueError('World dimensions must be 16-aligned')
    return {
        'min_section': min_y // 16,
        'max_section': (min_y + height - 1) // 16,
        'portal_logical_height': (height if height > 256 else 128) if to_nether else (height if height > 384 else 384),
        'raid_surface_top': 127 if to_nether and height <= 256 else min_y + height - 1,
    }


def plan(root: pathlib.Path) -> list[tuple[pathlib.Path, bytes, bytes]]:
    changes = []
    for rel, transformer in ((PORTAL, patch_portal), (RAIDS, patch_raids)):
        target = root / rel
        if not target.is_file() or target.is_symlink():
            raise ValueError(f'Missing or symlink source: {target}')
        before = target.read_bytes()
        if before.startswith(b'\xef\xbb\xbf'):
            raise ValueError('Unexpected BOM')
        after = transformer(before.decode('utf-8')).encode('utf-8')
        changes.append((target, before, after))
    return changes


def apply(changes: list[tuple[pathlib.Path, bytes, bytes]]) -> None:
    backups = [p.with_name(p.name + '.thalvoryn-original.bak') for p, _, _ in changes]
    if any(p.exists() for p in backups):
        raise ValueError('Existing .thalvoryn-original.bak file; no writes permitted')
    created = []
    try:
        # Back up ALL source files before changing ANY source file.
        for (_, old, _), backup in zip(changes, backups):
            with backup.open('xb') as f:
                f.write(old)
            created.append(backup)
        for target, _, new in changes:
            target.write_bytes(new)
            if target.read_bytes() != new:
                raise IOError(f'Post-write byte verification failed for {target}')
    except Exception:
        for target, old, _ in changes:
            if target.exists() and not target.is_symlink():
                target.write_bytes(old)
        raise


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description='Horizon tall worlds: portal + raid surface-height fixes (source-only)')
    parser.add_argument('horizon_checkout', type=pathlib.Path)
    parser.add_argument('--apply', action='store_true', help='Create two backups and change two Rust source files')
    args = parser.parse_args(argv)
    try:
        changes = plan(args.horizon_checkout)
        for target, old, new in changes:
            print(str(target), 'SHA256', hashlib.sha256(old).hexdigest(), '->', hashlib.sha256(new).hexdigest())
        if args.apply:
            apply(changes)
            print('Two Rust source files patched; original backups preserved. Build/test Horizon separately.')
        else:
            print('DRY RUN: no files changed. To write sources use --apply.')
        return 0
    except (ValueError, OSError, UnicodeError) as exc:
        print('STOP:', exc, file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())