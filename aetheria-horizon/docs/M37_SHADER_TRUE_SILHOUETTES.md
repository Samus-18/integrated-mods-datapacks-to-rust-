# Aetheria M37 — names MUST dictate real procedural shader silhouettes

Date 2026-11-21. DEVELOPMENT / DRAFT / not a visually approved release.
Parent branch: `aetheria-m36-100-authored-fx`. All changes in
`aetheria-m37-name-forms`. The unrelated Roblox project is untouched.

## Exact user corrections to preserve and honor

> "kamo analyzuj ten obrazek co jsi udelala a zapremyslej jestli fakt takhle to ma vypadat protoze to stoji za starouberlu slabe a nema to ani poradny tvar podivej se na jmena vsech kouzel a podle tech jmen logicky udelej tvar z toho neivm jak jse to jmenuje shader neco co si rikala jako u modu god want tam je tvar i krasa a sila"

> "tak to udelej a snaz se jak to udelas udelej si obrazek analyzuj a vzdy uvidis jak to vypada dokud to nebude kvalitove jako god wand tak to opakuj a zlepsuj"

> "zadny ai slop obrazky ty nidky nejsou tak jak ve hre takze precti si znovu co pisu a udelej to klidne se propoj zapl jsem to"

**Non-negotiable:** Do not create/generate any AI concept or imply it
shows Horizon gameplay. Look at ACTUAL game images and footage. Offline
rendered source SVGs are legitimate *internal engineering QA only*,
not proof of beauty, GPU shader output or Minecraft compatibility.

## Why M36 art FAILED despite passing 58 Rust tests

Read exact `docs/M36_100_AUTHORED_PROCEDURAL_SCENES.md`. M36 produced
small camera-facing quads and individual sparks with 12 soft masks.
Hero-shape logic produced 1–3 large transparent rectangles or
nonspecific solid generic `volumetric_forms`. Neither amount of
particles nor number of named profiles gives a serpent or cracked
earth silhouette.

M37.0 first physically connected Rust world-quad shapes used
`src/signature_forms.rs`, replacing old generic bodies for 11
name-selected spells. All 25 standalone exact-source QA tests +
79 shared RPG tests passed. The real Rust geometry was projected
to SVG with camera depth and face material; developer opened all 11
true-source renders, then REJECTED them:
  * Ember Needle looked like a tiny polygonal dagger.
  * Pyre Serpent visibly segmented, too short and rigid.
  * Arc Net was a tiny triangular web.
  * Rainweaver was a cuboid cage instead of flowing liquid.
  * Rift Slice looked like a closed purple loop.
  * Moment Slip still resembled clock shards/flat triangles.
  * The remaining storm/earth/vine/swarm silhouettes were too small.

M37.1 used larger physical scale, a continuous parametric serpent
skin with shared segment loops, linked plasma and time trajectories,
root/arc/necromancy changes. The SECOND actual-source geometry
gallery was downloaded and visually inspected. It became
more readable but remained dominated by flat coloured polygons,
bars and faceted ribbons. **ART FAILED AGAIN.** Passing Rust tests
never made it the user's requested God Wand visual quality.

## M37.2 — true computational WGSL shape material

- `src/signature_shader.rs`: one large, camera-facing WORLD-SPACE
  proxy quad for each of 11 specifically named spells. It is only a
  bounded, depth-tested support surface; it must NEVER be shown
  unmasked in a plain WorldQuad fallback.
- `shaders/pipeline/signature_sdf.wgsl`: ACTUAL GPU fragment shape
  algorithms: signed-distance segments, anti-aliased mask edges,
  turbulence/noise, interwoven flow, animated rift, branching electric
  nodes, snowstorm vortex, detailed spectral birds and different
  coloured emissive materials. These are original code, NOT copied
  from God Wand or an unsupported Java shader API.
- Separate optional render pipeline `aetheria_preview:signature_sdf`
  with translucent depth test. In `submit_custom_geometry` these
  eleven named spells are submitted FIRST to the GPU SDF shader.
  ONLY on success are their old faceted 3D bodies suppressed.
  Pipeline unavailable/submission fails => draw the actual earlier
  `signature_forms` 3D meshes; **never draw raw giant opaque
  SDF proxy quads**.
- M36's 100 individually named particle choreography profiles still
  exist as secondary accents, with 48 motion signatures and 12
  other GPU soft-material families. The **other 89 of these 100**
  have NOT received an individually fully approved signature shader
  hero yet; remaining 100 of 200 retain M35 background emitters.
- New signature index slots = 0 Ember Needle; 6 Pyre Serpent; 29
  Snowblind; 47 Twister Seed; 63 Faultline; 86 Rainweaver;
  101 Root Snare; 123 Arc Net; 144 Rift Slice; 161 Moment Slip;
  186 Carrion Swarm. Frost Needle #021 and Rime Bridge #022
  retain their M33/M34 original source meshes and are not overridden.
- M37 GPU shader is still **2.5D depth-tested world quad shading**,
  NOT true fullscreen ray-traced HDR/postprocessing, physics
  simulation, dynamic light integration or native depth refraction.
  A separately validated OPTIONAL Horizon shaderpack can add mild
  full-screen glow but can only be user-selected and may replace
  the active shaderpack.

## Runtime/acceptance requirements

Use official pinned Horizon Nightly SDK build, Naga tests for
`signature_sdf.wgsl`, 200 ID routing, alpha-mask fallback tests,
budget/overlap/LOD tests, 11 SDF signature IDs and non-11
safe route tests. Source SVG projections are not enough:
**actual Horizon 26.3 screenshots/video** of the WGSL material
must be captured before claiming beautiful, final or screenshot
accurate VFX. Need real sound timing, authoritative damage/status,
collision/terrain edits, world rollback, multiplayer and saves
and FPS test on the user's RX580/Ryzen5 1600. Failing these means
0/100 production-quality spell certification.

The user's PC connector was re-enabled and used for editing the
M37 git worktree ONLY (never overwrote M36/M33 or the user's local
game); the active Roblox Player was intentionally not interrupted.
Do not install or launch an incomplete Horizon build over an
authenticated running game window without an explicit real testing
window from the user.

## Continuous learning rules

1. Reject weak visual assets instead of defensively claiming they
   are finished because they compiled.
2. For each hero spell, **read the canonical named intent first**,
   then synthesize its fundamental shape and timed interaction.
3. Do not spend limited PC connection budget on minor isolated
   file calls; batch tests, CI and source changes.
4. Keep 3d source volume, shader-created form, and gameplay effects
   separate in every status statement.
5. Preserve original conversation requests and this log; never
   substitute AI concept art for game captures again.
