//! Deterministic, CPU-side 3D effect instance planner.
//! Produces render intents, not GPU draw calls. Horizon renderer integration is pending.
//! No world state, camera matrices or wgpu handles are accessed here.

use std::f32::consts::{PI, TAU};
use super::particles::ParticleBudget;
use super::timeline::{for_spell, Stage};

/// Each prototype has its own spatial silhouette, not just a changed tint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Motif {
    EmberNeedle, CinderFan, FrostNeedle, RimeBridge,
    WindPush, RazorCrescent, StoneFist, CrystalTraverse,
    WaterJet, Flowstep, ThornSeed, RootSnare,
    ForkLightning, StaticHalo, StarShard, GravityThread,
    ProjectileDelay, MomentSlip, BoneSpike, SoulLantern,
}
impl Motif {
    pub fn for_spell(id: u16) -> Option<Self> {
        Some(match id {
            1 => Self::EmberNeedle, 2 => Self::CinderFan,
            21 => Self::FrostNeedle, 22 => Self::RimeBridge,
            41 => Self::WindPush, 42 => Self::RazorCrescent,
            61 => Self::StoneFist, 62 => Self::CrystalTraverse,
            81 => Self::WaterJet, 82 => Self::Flowstep,
            101 => Self::ThornSeed, 102 => Self::RootSnare,
            121 => Self::ForkLightning, 122 => Self::StaticHalo,
            141 => Self::StarShard, 142 => Self::GravityThread,
            161 => Self::ProjectileDelay, 162 => Self::MomentSlip,
            181 => Self::BoneSpike, 182 => Self::SoulLantern,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActiveCast {
    pub spell_id: u16,
    pub start_tick: u64,
    pub origin: [f32; 3],
    pub direction: [f32; 3],
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VfxInstance {
    pub spell_id: u16,
    pub motif: Motif,
    pub stage: Stage,
    pub position: [f32; 3],
    pub scale: f32,
    pub tint: [f32; 3],
    pub emissive: f32,
    pub opacity: f32,
}

#[derive(Debug, Default, PartialEq)]
pub struct FramePlan {
    pub instances: Vec<VfxInstance>,
    pub visible_casts: usize,
}

#[derive(Clone, Copy)]
struct Job {
    cast: ActiveCast,
    motif: Motif,
    stage: Stage,
    progress: f32,
    wanted: usize,
    direction: [f32; 3],
}

fn finite3(v: [f32; 3]) -> bool { v.iter().all(|x| x.is_finite()) }
fn length(v: [f32; 3]) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}
fn normalized(v: [f32; 3]) -> Option<[f32; 3]> {
    let len = length(v);
    if !len.is_finite() || len <= 0.00001 { return None; }
    Some([v[0] / len, v[1] / len, v[2] / len])
}
fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    length([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
}

/// A stable per-frame plan, bounded to 1,200 instances and 64 active effects.
/// The round-robin allocator prevents one spectacular spell from starving all others.
pub fn plan_frame(
    casts: &[ActiveCast],
    now_tick: u64,
    camera: [f32; 3],
    quality: f32,
    budget: ParticleBudget,
) -> FramePlan {
    let mut output = FramePlan::default();
    if !finite3(camera) || !quality.is_finite() || quality <= 0.0
        || !budget.distance_falloff.is_finite() || budget.distance_falloff <= 0.0
    { return output; }
    let quality = quality.clamp(0.0, 1.0);
    let max_instances = budget.max_per_frame.min(1_200) as usize;
    if max_instances == 0 { return output; }

    let mut jobs = Vec::new();
    for &cast in casts {
        if now_tick < cast.start_tick
            || !finite3(cast.origin) || !finite3(cast.direction)
            || !cast.intensity.is_finite() || cast.intensity <= 0.0
        { continue; }
        let Some(direction) = normalized(cast.direction) else { continue };
        let Some(motif) = Motif::for_spell(cast.spell_id) else { continue };
        let Some(timeline) = for_spell(cast.spell_id) else { continue };
        let Ok(elapsed) = u32::try_from(now_tick - cast.start_tick) else { continue };
        let mut phase_begin = 0u32;
        let mut phase_data = None;
        for phase in timeline.phases {
            let end = phase_begin + u32::from(phase.duration_ticks);
            if elapsed < end {
                let progress = (elapsed - phase_begin) as f32 / phase.duration_ticks as f32;
                phase_data = Some((phase.kind, progress, phase.particle_cap));
                break;
            }
            phase_begin = end;
        }
        let Some((stage, progress, cap)) = phase_data else { continue };
        let dist = distance(camera, cast.origin);
        if !dist.is_finite() { continue; }
        let raw = budget.allowed(u32::from(cap), dist);
        // Visual intensity is capped: a charged spell may not bypass the GPU budget.
        let wanted = ((raw as f32 * quality * cast.intensity.clamp(0.25, 2.0))
            .floor() as usize).min(max_instances);
        if wanted == 0 { continue; }
        jobs.push(Job { cast, motif, stage, progress, wanted, direction });
    }

    jobs.sort_by_key(|j| (
        j.cast.start_tick, j.cast.spell_id,
        j.cast.origin[0].to_bits(), j.cast.origin[1].to_bits(),
        j.cast.origin[2].to_bits(),
    ));
    jobs.truncate(64);
    output.visible_casts = jobs.len();
    let mut shares = vec![0usize; jobs.len()];
    let mut remaining = max_instances;
    while remaining > 0 {
        let mut distributed = false;
        for (share, job) in shares.iter_mut().zip(jobs.iter()) {
            if remaining == 0 { break; }
            if *share < job.wanted {
                *share += 1;
                remaining -= 1;
                distributed = true;
            }
        }
        if !distributed { break; }
    }

    output.instances.reserve(max_instances - remaining);
    for (job, count) in jobs.into_iter().zip(shares) {
        let class = ((job.cast.spell_id - 1) / 20) as usize;
        let palette: [[f32; 3]; 10] = [
            [1.0, 0.27, 0.06], [0.40, 0.83, 1.0], [0.63, 0.91, 0.95],
            [0.72, 0.49, 0.96], [0.11, 0.76, 1.0], [0.32, 0.95, 0.42],
            [0.58, 0.40, 1.0], [0.78, 0.26, 0.94], [1.0, 0.75, 0.39],
            [0.88, 0.30, 0.55],
        ];
        let phase_size = match job.stage {
            Stage::Telegraph => 0.46,
            Stage::Cast => 1.0,
            Stage::Impact => 1.65,
            Stage::Afterglow => 0.72,
        };
        let dir = job.direction;
        let side = normalized([-dir[2], 0.0, dir[0]]).unwrap_or([1.0, 0.0, 0.0]);
        for i in 0..count {
            let u = (i as f32 + 0.5) / count as f32;
            let v = silhouette(job.cast.spell_id, u, job.progress) ;
            let p = [v[0] * phase_size, v[1] * phase_size, v[2] * phase_size];
            // Local x: perpendicular to aim, y: vertical, z: forward.
            let position = [
                job.cast.origin[0] + side[0] * p[0] + dir[0] * p[2],
                job.cast.origin[1] + p[1] + dir[1] * p[2],
                job.cast.origin[2] + side[2] * p[0] + dir[2] * p[2],
            ];
            let opacity = match job.stage {
                Stage::Telegraph => 0.35 + 0.6 * job.progress,
                Stage::Afterglow => (1.0 - job.progress) * 0.7,
                _ => 1.0,
            };
            output.instances.push(VfxInstance {
                spell_id: job.cast.spell_id, motif: job.motif, stage: job.stage,
                position, scale: (0.08 + 0.16 * u) * phase_size,
                tint: palette[class], emissive: 1.6 + 1.5 * (1.0 - u),
                opacity,
            });
        }
    }
    output
}

/// Twenty deliberately distinct 3D paths in spell-local coordinates.
/// These are GPU instance placement guides, not claims of finished meshes/shaders.
fn silhouette(id: u16, u: f32, time: f32) -> [f32; 3] {
    let a = TAU * u;
    match id {
        1 => [0.08 * (a * 3.0).sin(), 0.08 * (a * 3.0).cos(), 6.0 * u],
        2 => [5.0 * (u - 0.5), 0.25 * (a * 2.0).sin(), 3.5 * (1.0 - (u - 0.5).abs())],
        21 => [0.4 * (a * 6.0).sin(), 0.4 * (a * 6.0).cos(), 7.0 * u],
        22 => [0.6 * (a * 4.0).sin(), -0.9 + 0.12 * (a * 8.0).cos(), u * 8.0],
        41 => [2.0 * (a + time).cos(), 1.5 * (a + time).sin(), u * 4.0],
        42 => [4.2 * (PI * (u - 0.5)).sin(), 1.1 * (PI * u).cos(), 3.0 * u],
        61 => [1.2 * a.cos(), 5.5 * u * u, 1.2 * a.sin()],
        62 => [1.6 * (a * 1.5).cos(), 5.0 * u, 1.6 * (a * 1.5).sin()],
        81 => [0.18 * (a * 9.0).sin(), 0.4 * (a * 4.0).cos(), 9.0 * u],
        82 => [0.85 * (a * 2.0).sin(), -0.7 + 0.2 * u, 7.0 * u],
        101 => [0.6 * (a * 3.0).sin(), 4.0 * u, 2.6 * (a * 0.5).cos()],
        102 => [4.0 * u * a.cos(), -0.4 + u * 1.5, 4.0 * u * a.sin()],
        121 => [1.8 * (a * 7.0).sin() * u, 1.4 * (a * 5.0).cos(), 7.0 * u],
        122 => [3.5 * a.cos(), 1.5 + 0.3 * (a * 3.0).sin(), 3.5 * a.sin()],
        141 => [2.0 * a.cos() * u, 2.0 * a.sin() * u, 8.0 * u],
        142 => [3.0 * (a * 0.5).sin(), 2.0 * (a * 1.5).cos(), 4.0 * u],
        161 => [3.0 * a.cos() * (1.0 - u), 3.0 * a.sin() * (1.0 - u), 3.0 * (u - 0.5)],
        162 => [2.4 * (a * 2.0 - time).cos() * (1.0 - u), 0.8 * a.sin(), 5.0 * (1.0 - u)],
        181 => [2.8 * (a * 4.0).cos(), 4.0 * u.powf(1.5), 2.8 * (a * 4.0).sin()],
        182 => [2.2 * a.cos(), 1.0 + 0.4 * (a * 2.0 + time).sin(), 2.2 * a.sin()],
        _ => [0.0; 3],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cast(id: u16) -> ActiveCast {
        ActiveCast { spell_id: id, start_tick: 100, origin: [0.0; 3],
            direction: [0.0, 0.0, 1.0], intensity: 1.0 }
    }
    fn frame(casts: &[ActiveCast], now: u64, cap: u32) -> FramePlan {
        plan_frame(casts, now, [0.0; 3], 1.0,
            ParticleBudget { max_per_frame: cap, distance_falloff: 96.0 })
    }
    #[test]
    fn all_twenty_spells_have_distinct_spatial_motifs() {
        use std::collections::HashSet;
        let mut motifs = HashSet::new();
        let mut silhouettes = HashSet::new();
        for class in 0..10u16 {
            for n in 1..=2 {
                let id = class * 20 + n;
                assert!(motifs.insert(Motif::for_spell(id).unwrap()));
                let s = silhouette(id, 0.32, 0.7);
                assert!(silhouettes.insert(s.map(f32::to_bits)));
                let plan = frame(&[cast(id)], 103, 100);
                assert!(!plan.instances.is_empty(), "spell {id}");
                assert!(plan.instances.iter().all(|p| p.motif == Motif::for_spell(id).unwrap()));
            }
        }
        assert_eq!(motifs.len(), 20);
        assert_eq!(silhouettes.len(), 20);
        assert_eq!(Motif::for_spell(3), None);
    }
    #[test]
    fn global_cap_and_fairness_for_simultaneous_spells() {
        let plan = frame(&[cast(1), cast(21), cast(41), cast(61)], 103, 12);
        assert_eq!(plan.instances.len(), 12);
        for id in [1, 21, 41, 61] {
            assert_eq!(plan.instances.iter().filter(|s| s.spell_id == id).count(), 3);
        }
    }
    #[test]
    fn order_independent_and_deterministic() {
        let a = frame(&[cast(2), cast(1)], 104, 50);
        let b = frame(&[cast(1), cast(2)], 104, 50);
        assert_eq!(a, b);
    }
    #[test]
    fn phase_boundaries_and_expiration() {
        assert_eq!(frame(&[cast(1)], 100, 50).instances[0].stage, Stage::Telegraph);
        assert_eq!(frame(&[cast(1)], 103, 50).instances[0].stage, Stage::Cast);
        assert_eq!(frame(&[cast(1)], 107, 50).instances[0].stage, Stage::Impact);
        assert!(frame(&[cast(1)], 121, 50).instances.is_empty());
        assert!(frame(&[cast(1)], 99, 50).instances.is_empty());
    }
    #[test]
    fn bad_inputs_and_distant_effects_emit_nothing() {
        let mut bad = cast(1);
        bad.intensity = f32::NAN;
        assert!(frame(&[bad], 103, 50).instances.is_empty());
        bad = cast(1); bad.origin[0] = f32::INFINITY;
        assert!(frame(&[bad], 103, 50).instances.is_empty());
        bad = cast(1); bad.direction = [0.0; 3];
        assert!(frame(&[bad], 103, 50).instances.is_empty());
        bad = cast(1); bad.origin = [200.0, 0.0, 0.0];
        assert!(frame(&[bad], 103, 50).instances.is_empty());
        assert!(plan_frame(&[cast(1)], 103, [0.0; 3], f32::NAN,
            ParticleBudget::default()).instances.is_empty());
    }
    #[test]
    fn very_large_cast_bursts_never_exceed_safe_cap() {
        let mut casts = Vec::new();
        for i in 0..300 {
            let mut c = cast(if i % 2 == 0 { 122 } else { 121 });
            c.start_tick = i % 5;
            casts.push(c);
        }
        let p = frame(&casts, 8, u32::MAX);
        assert!(p.instances.len() <= 1_200);
        assert!(p.visible_casts <= 64);
        assert!(p.instances.iter().all(|v|
            finite3(v.position) && v.opacity >= 0.0 && v.opacity <= 1.0));
    }
}
