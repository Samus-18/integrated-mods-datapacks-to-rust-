// Half-resolution emitted radiance. Original 3D field ray integration with
// screen-depth clipping, not a transparent camera-aligned hero card.
fn post(input:PostInput)->vec4<f32> {
    let v=fx_volume(input,12);
    return vec4<f32>(v.light*0.9,1.0);
}
