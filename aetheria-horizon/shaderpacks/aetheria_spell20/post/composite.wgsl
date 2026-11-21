// Screen color + world-occluded 3D volume + downsampled dual-axis bloom.
// This stage may lens the original image locally, without replacing the
// entire frame or pretending a quad is a 3D ray-marched volume.
fn post(input:PostInput)->vec4<f32> {
    let current=fx_read_state();
    if(current.id==0) {return input.color;}
    let vol=fx_volume(input,18);
    let uv=input.uv;
    let shimmer=vec2<f32>(
        sin(uv.y*42.0+frame.time*4.5),
        cos(uv.x*31.0-frame.time*3.7));
    let offset=clamp(vol.distortion,0.0,0.12)*shimmer*0.016;
    let distorted=textureSampleLevel(scene_color,linear_sampler,
        clamp(uv+offset,vec2<f32>(0.001),vec2<f32>(0.999)),0.0);
    let near=vol.light*1.08;
    let glow=textureSampleLevel(aetheria_blur_v,linear_sampler,uv,0.0).rgb;
    let rgb=distorted.rgb + near + glow*aetheria_bloom*0.66;
    // Linear-light HDR: do not globally bleach the world or fake fullscreen whiteout.
    return vec4<f32>(min(rgb,vec3<f32>(max(2.5,frame.headroom*1.8))),input.color.a);
}
