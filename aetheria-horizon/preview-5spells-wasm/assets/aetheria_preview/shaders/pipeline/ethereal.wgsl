// Aetheria native pipeline fragment. Horizon injects PipelineInput + frame.
// No custom bindings/entry points; quads are provided by the renderer.
fn pipeline_fragment(input: PipelineInput) -> vec4<f32> {
    let uv = input.uv * 2.0 - vec2<f32>(1.0, 1.0);
    let s = sin(uv.x * 9.0 + frame.time * 7.0) * sin(uv.y * 11.0 - frame.time * 5.0);
    let shimmer = 0.82 + 0.18 * s;
    let edge = max(abs(uv.x), abs(uv.y));
    let veil = 1.0 - smoothstep(0.78, 1.05, edge);
    let hot = 0.20 + 0.14 * sin(frame.time * 3.7 + uv.x * 4.0);
    let color = input.color.rgb * (1.65 + hot) * shimmer;
    return vec4<f32>(color, input.color.a * veil);
}
