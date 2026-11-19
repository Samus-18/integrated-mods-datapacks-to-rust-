// Aetheria M18: ORIGINAL shader-side 3D light volume integration prototype.
// Direct research finding: the Horizon effects the user likes use scene depth
// and camera-aware fullscreen post passes. The CURRENT WASM .hmod DOES NOT
// submit this shader. This standalone shader is written as a reusable module
// to integrate only with a separately verified Horizon native render hook.
//
// Not a copy of any other mod's shaders. No device bindings are declared;
// the native bridge must provide ray origin/direction, target center, time,
// depth-limited ray distance and quality.
//
// Output is premultiplied radiance + opacity so a native post pass can use:
// output.rgb + scene.rgb * (1.0 - output.a). Accuracy requires linear-light
// compositing on the eventual engine target (not standard gamma RGB).
//
// Naga parser/validator in preview-5spells-wasm/src/shader_validation.rs
// verifies this actual source as part of CI.
fn aetheria_smooth_noise(p: vec3<f32>) -> f32 {
    // Procedural directional magic waves, inexpensive: no large 3D texture.
    let f = sin(dot(p, vec3<f32>(2.173, 1.879, 1.237)));
    let g = sin(dot(p, vec3<f32>(-1.312, 2.517, 1.746)));
    let h = cos(dot(p, vec3<f32>(1.441, 0.974, -2.817)));
    return 0.5 + 0.1666667 * (f + g + h);
}
fn aetheria_aurora_density(q: vec3<f32>, time: f32) -> f32 {
    // q is the position relative to a *fantasy spell*, in world blocks.
    // A tall cathedral of twisting colored light with grounded foot.
    let y = q.y;
    let sweep = 0.58 * sin(y * 0.63 + time * 1.14);
    let sweep2 = 0.41 * cos(y * 0.39 - time * 0.83);
    let local = vec2<f32>(q.x - sweep, q.z - sweep2);
    let radial = length(local);
    let body = exp(-radial * radial * 0.46);
    let filament = exp(-radial * radial * 4.2) *
                   (0.75 + 0.25 * sin(y * 2.1 - time * 2.3));
    let base = smoothstep(-0.75, 0.35, y);
    let crown = 1.0 - smoothstep(5.1, 8.3, y);
    let band = 0.55 + 0.45 * sin(y * 2.4 + time * 1.78 - radial * 3.8);
    let corrugation = aetheria_smooth_noise(q * 1.13 + vec3<f32>(time * 0.25,0.0,0.0));
    // A broader, dimmer enclosing haze expands around the bright core;
    // it is never a lone outline cylinder.
    let halo = exp(-radial * radial * 0.065) * 0.19;
    return max(0.0, (body * (0.45 + 0.45 * corrugation) +
        halo * (0.58 + 0.42 * band) + filament * 0.48) * base * crown);
}
fn aetheria_fantasy_light_color(p: vec3<f32>,time:f32)->vec3<f32>{
    let alt=0.5+0.5*sin(p.y * 0.79 + time * 1.2 + p.x * 1.3);
    let ice=vec3<f32>(0.05,0.85,1.23);      // turquoise glacial light
    let violet=vec3<f32>(0.49,0.20,0.96);  // amethyst ray
    let lilac=vec3<f32>(0.70,0.43,1.08);  // frosty highlight
    return mix(mix(ice,violet,alt),lilac,0.18);
}
/// Integrate an original colored magic pillar using bounded front-to-back
/// volume accumulation. Only the actually visible part of a camera ray is
/// sampled; the caller must derive max_distance from real scene depth.
///
/// This is a shading prototype, NOT a real available GPU hook.
fn aetheria_integrate_magic_volume(
    ray_origin:vec3<f32>,ray_direction:vec3<f32>,
    center:vec3<f32>,time:f32, max_distance:f32, quality:u32
)->vec4<f32>{
    let direction=normalize(ray_direction);
    let closest=dot(center-ray_origin,direction);
    let half_extent=10.5;
    let begin=max(0.0,closest-half_extent);
    let finish=min(max(max_distance,0.0),closest+half_extent);
    if finish<=begin {return vec4<f32>(0.0);}
    let steps=clamp(quality,6u,24u);
    let ds=(finish-begin)/f32(steps);
    var transmittance=1.0;
    var radiance=vec3<f32>(0.0);
    for(var i=0u; i<steps; i=i+1u){
        let t=begin+(f32(i)+0.5)*ds;
        let p=ray_origin+direction*t;
        let density=aetheria_aurora_density(p-center,time);
        let attenuation=1.0-exp(-density*ds*0.82);
        let contribution=transmittance*attenuation;
        // Real shadowing would need world depth/lighting; do not claim it.
        let q=p-center;
        let radial=length(q.xz - vec2<f32>(
            0.58 * sin(q.y * 0.63 + time * 1.14),
            0.41 * cos(q.y * 0.39 - time * 0.83)));
        let hot_core=exp(-radial * radial * 3.4);
        radiance += aetheria_fantasy_light_color(q,time)
                    * contribution * (0.74+0.86*hot_core);
        transmittance *= (1.0-attenuation);
        if transmittance<0.018 {break;}
    }
    return vec4<f32>(radiance,clamp(1.0-transmittance,0.0,0.89));
}
