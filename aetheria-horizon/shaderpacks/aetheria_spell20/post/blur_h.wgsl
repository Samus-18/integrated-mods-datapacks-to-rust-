// Original linear-light horizontal bloom after half-res volume emission.
fn post(input:PostInput)->vec4<f32> {
    let pixel=1.0/vec2<f32>(textureDimensions(aetheria_emission));
    let o=vec2<f32>(pixel.x,0.0);
    let uv=input.uv;
    var sum=textureSampleLevel(aetheria_emission,linear_sampler,uv,0.0).rgb*0.295;
    sum+=textureSampleLevel(aetheria_emission,linear_sampler,uv+o*1.0,0.0).rgb*0.226;
    sum+=textureSampleLevel(aetheria_emission,linear_sampler,uv-o*1.0,0.0).rgb*0.226;
    sum+=textureSampleLevel(aetheria_emission,linear_sampler,uv+o*2.0,0.0).rgb*0.1265;
    sum+=textureSampleLevel(aetheria_emission,linear_sampler,uv-o*2.0,0.0).rgb*0.1265;
    return vec4<f32>(sum,1.0);
}
