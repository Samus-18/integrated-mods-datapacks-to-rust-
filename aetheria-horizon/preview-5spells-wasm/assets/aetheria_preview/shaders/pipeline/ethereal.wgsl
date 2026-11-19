// Aetheria M17: flowing fantasy-material surface shader for procedural 3D
// geometry. Based on actual reference video motion and material layering,
// not copied artwork. The pinned Horizon guest PipelineInput currently has
// only UV + per-vertex color and frame.time: this DOES NOT add real HDR,
// scene-space refraction, bloom, world-space lighting or shadow casting.
// Crucially, the spell mesh supplies its own original silhouette and colors.
fn aetheria_hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}
fn aetheria_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (vec2<f32>(3.0) - 2.0 * f);
    let a = aetheria_hash(i);
    let b = aetheria_hash(i + vec2<f32>(1.0, 0.0));
    let c = aetheria_hash(i + vec2<f32>(0.0, 1.0));
    let d = aetheria_hash(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}
fn pipeline_fragment(input: PipelineInput) -> vec4<f32> {
    let uv = input.uv * 2.0 - vec2<f32>(1.0);
    let t = frame.time;

    // Kinetic material layers: one broad slow fold, one fast moving vein,
    // one carefully bounded local shimmer. Never a grid of flat rods/rings.
    let distorted = uv + vec2<f32>(
        0.052 * sin(uv.y * 8.5 + t * 1.17),
        0.048 * cos(uv.x * 7.2 - t * 0.83)
    );
    let drift = distorted * 3.6 + vec2<f32>(t * 0.19, -t * 0.26);
    let broad = aetheria_noise(drift);
    let micro = aetheria_noise(drift * 2.3 + vec2<f32>(t * 0.21, 0.0));
    let veining = abs(sin(
        (distorted.x * 9.7 - distorted.y * 7.1) +
        2.4 * broad + t * 1.7
    ));
    let frostEdges = smoothstep(0.76, 0.97, veining);
    let fireMotion = sin(distorted.x * 12.0 + distorted.y * 4.5 - t * 5.2);

    // The renderer provides per-face fantasy palettes: respect their hue.
    // Different pigments get subtly different temporal treatment, not
    // the SAME washed-out white gradient across all ten schools.
    let c = clamp(input.color.rgb, vec3<f32>(0.0), vec3<f32>(1.0));
    let icy = smoothstep(0.02, 0.37, c.b - c.r) *
              smoothstep(-0.10, 0.25, c.g - c.r);
    let fiery = smoothstep(0.10, 0.50, c.r - c.b);
    let arcane = smoothstep(0.06, 0.33, c.b - c.g);
    let pulse = 0.5 + 0.5 * sin(t * 2.3 + broad * 6.28);
    let base = 0.71 + 0.14 * broad + 0.08 * micro
                    + 0.07 * pulse + 0.05 * fireMotion * fiery;
    let spectral = vec3<f32>(
        0.01 * icy + 0.05 * fiery + 0.04 * arcane,
        0.07 * icy + 0.008 * fiery + 0.01 * arcane,
        0.05 * icy + 0.008 * fiery + 0.07 * arcane
    );
    let blueVeins = (0.04 + 0.13 * frostEdges) * (icy + arcane);
    let innerGlow = vec3<f32>(0.28, 0.56, 0.70) * blueVeins;
    let color = clamp(c * (base + 0.08 * frostEdges) +
                        spectral + innerGlow,
                        vec3<f32>(0.0), vec3<f32>(0.98));
    // Airy translucent faces stay airy. Filled crystals do not turn into
    // fully transparent glass. Preserve conservative color alpha; never
    // fabricate bright opaque sheets out of low-alpha atmosphere.
    let a = clamp(input.color.a * (0.90 + 0.07 * broad) *
                  (0.95 + 0.045 * pulse), 0.0, 0.88);
    return vec4<f32>(color, a);
}
