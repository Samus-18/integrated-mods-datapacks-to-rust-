// Aetheria M14 / 2026-11-19 — restrained enchanted-material surface treatment.
// Shader input is restricted to Horizon's tested PipelineInput + frame.
// This is NOT physically correct Fresnel, raymarched volumetrics, HDR bloom
// or dynamic light (none are exposed by this pinned guest shader interface).
// It supplies time-varying energy density within each actual 3D mesh face.
// Respect player brightness and avoid whiteout from overlapping translucent quads.
fn pipeline_fragment(input: PipelineInput) -> vec4<f32> {
    let uv = input.uv * 2.0 - vec2<f32>(1.0, 1.0);
    let radial = length(uv);
    let t = frame.time;
    let weave = sin(uv.x * 13.0 + uv.y * 7.0 + t * 2.4)
              * sin(uv.y * 8.5 - uv.x * 5.5 - t * 1.3);
    let fine = sin(radial * 19.0 - t * 3.3);
    let rim = smoothstep(0.17, 0.91, radial);
    let center = 1.0 - smoothstep(0.12, 0.95, radial);
    // Subtle decorative material variation on a *real volumetric surface*.
    // Limit the face brightness to preserve color and prevent overexposure.
    let modulation = clamp(
        0.77 + center * 0.16 + weave * 0.065 + fine * 0.035
        + rim * 0.06, 0.66, 1.04
    );
    let color = clamp(input.color.rgb * modulation, vec3<f32>(0.0), vec3<f32>(1.0));
    // Avoid a one-pixel hard edge while retaining opacity of solid crystals.
    let edgeFade = 1.0 - smoothstep(0.86, 1.45, radial) * 0.16;
    let opacity = clamp(input.color.a * edgeFade * (0.92 + 0.05 * weave), 0.0, 0.83);
    return vec4<f32>(color, opacity);
}
