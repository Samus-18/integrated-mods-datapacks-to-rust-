# Aetheria Horizon — original reference video archive

**Archive manifest prepared on 2026-11-19 from the user's four re-uploaded original MP4 attachments.** Video byte integrity was checked locally with SHA-256. Each original video has also been saved as a persistent file in the user's ChatGPT Library under **`/Aetheria Horizon/Reference Videos/Original Uploads/`**, plus one consolidated ZIP (`/Aetheria Horizon/Reference Videos/Aetheria_Original_4_Videos_References.zip`).

**Integrity distinction:** the actual original MP4 binaries are **NOT in this GitHub repository yet**. The GitHub connector currently supports text files / blobs supplied as a string, not reading 40 MB of local binary uploads. This directory is a searchable index, not fake placeholder files. Do not claim that merely having filenames/notes in git preserves the original video bytes. The user has a verified ZIP and four Library copies; manual upload or a binary-capable connector would be needed to add them to GitHub.

## Original videos and SHA-256

| Original name | Duration | Exact bytes | SHA-256 |
| --- | ---: | ---: | --- |
| `Download (3).mp4` | 27.367 s | 2,243,511 | `ef97a0be535c641300c28a2fcc55cbf552ea43ab4e4df14f4f4d5e40ff294373` |
| `Download (4).mp4` | 58.033 s | 12,287,281 | `5c1c47dff851af34989e63540caa36aa139a8ef3d9763c7bcb98d13459512343` |
| `Download (5).mp4` | 150.720 s | 21,986,856 | `20681300b509d314e49988af30a2f2198f9f8e1656bc8a935bdf035f7df14dbc` |
| `Download (6).mp4` | 40.100 s | 3,794,259 | `36dba52f0ea71a457105b3e5e660eee683c4439805a772635873946d55cce10a` |

All at 30fps. Video 5 is 1024×576 landscape; 3/4/6 are 576×1024 portrait. Full audiovisual originals are distinct from the sampled frame/contact-sheet notes.

## Captured reference motifs — sampled at 8 times per clip

- **(3)** ~1.7–12s: sky-filling aerial magical beam, fluctuating green/gold light; ~18.8–25.7s: circular ground ritual, intense violet/gold aftermath.
- **(4)** ~3.6–25.4s: dark suspended aperture among radial translucent cracks in the blue sky; ~32.6–47.1s: falling masses, airborne dust, landscape-scale impact/crater aftermath.
- **(5)** ~9.4–47.1s: layered air/terrain ripples and broad destructive landscape effects; ~65.9–103.6s: hovering dark construction, light-ring ritual; ~122.5–141.3s: deep atmospheric lighting and lava/ember eruption.
- **(6)** ~2.5–17.5s: purple/teal crystalline eruptions and beams; ~22.6–37.6s: bright crustal field and more gigantic crystal mass protruding from the ground.

**Original fantasy implementation, never a copy:** shader-readable depth, 5–8-sided world-space beams, 3D crystal surfaces and staggered ground attachment, rings with visible volume, animated buildup/impact/afterglow, bounded draw budgets, spatial audio. No copied sprites, textures, music, character names or video frames in released mod assets. The user's latest visual target also references God Wand for general scale and choreography; not for copying its assets.

## How to add the binaries to this repo later

After obtaining the ZIP from the project's ChatGPT Library or its conversation attachment, extract its `originals/` folder and upload the four exact MP4 files under `aetheria-horizon/references/original-videos/`. Verify each SHA-256 above before considering the repository archive complete. For larger history, consider Git LFS, releases or external binary storage rather than many revisions of video files in ordinary git.

See [source-analysis](../../docs/VIDEO_REFERENCE_ANALYSIS.md), [user feedback](../../docs/memory/USER_LOG.md), [master requirements](../../docs/PROJECT_SPECIFICATION.md).
