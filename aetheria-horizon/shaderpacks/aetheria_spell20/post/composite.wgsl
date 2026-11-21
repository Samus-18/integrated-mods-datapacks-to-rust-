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
    let glow=textureSampleLevel(aetheria_blur_v,linear_sampler,uv,0.0).rgb;
    // On untouched pixels preserve the original image EXACTLY. The old
    // shader unconditionally sampled and clamped the entire game frame.
    if(vol.coverage<0.001 && length(glow)<0.001){return input.color;}
    let luma=dot(input.color.rgb,vec3<f32>(0.2126,0.7152,0.0722));
    let scene_adaptation=1.0/(1.0+2.3*max(luma,0.0));
    var rgb=input.color.rgb;
    if(vol.solid>0.5) {
        // Colored, lit world-space faces replace the real background at
        // their screen-space silhouette, not a giant emissive fog overlay.
        rgb=mix(input.color.rgb,vol.light,vol.coverage);
    } else {
        rgb=distorted.rgb + vol.light*0.72*scene_adaptation;
    }
    rgb+=glow*aetheria_bloom*0.13*scene_adaptation;
    let limit=max(1.10,frame.headroom*1.25);
    return vec4<f32>(min(rgb,vec3<f32>(limit)),input.color.a);
}
