// Aetheria V0.3.0: preserve colour instead of overexposing stacked bright sheets.
// Horizon provides PipelineInput and frame; no custom bind groups.
fn pipeline_fragment(input: PipelineInput) -> vec4<f32> {
    let uv = input.uv * 2.0 - vec2<f32>(1.0, 1.0);
    let ripples = sin(uv.x * 6.0 + frame.time * 3.0) * sin(uv.y * 7.0 - frame.time * 2.0);
    let edge = max(abs(uv.x), abs(uv.y));
    let veil = 1.0 - smoothstep(0.64, 1.0, edge);
    let light = 0.93 + 0.11 * ripples;
    let color = input.color.rgb * light;
    let opacity = input.color.a * veil * (0.82 + 0.08 * ripples);
    return vec4<f32>(color, opacity);
}
