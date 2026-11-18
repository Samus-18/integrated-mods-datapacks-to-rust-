// Radial shockwave image distortion. Requires actual scene texture from renderer.
@group(0) @binding(0) var scene:texture_2d<f32>;
@group(0) @binding(1) var scene_sampler:sampler;
struct Params{center:vec2<f32>,radius:f32,strength:f32}
@group(0) @binding(2) var<uniform> params:Params;
struct Out{@builtin(position) clip:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn vs_main(@builtin(vertex_index) idx:u32)->Out{
 let vertices=array<vec2<f32>,3>(vec2<f32>(-1.0,-1.0),vec2<f32>(3.0,-1.0),vec2<f32>(-1.0,3.0));
 var o:Out;o.clip=vec4<f32>(vertices[idx],0.0,1.0);
 o.uv=vertices[idx]*0.5+vec2<f32>(0.5);return o;
}
@fragment fn fs_main(i:Out)->@location(0) vec4<f32>{
 let delta=i.uv-params.center;let dist=length(delta);
 let ring=exp(-pow((dist-params.radius)*40.0,2.0));
 let offset=delta/max(dist,0.00001)*ring*params.strength*0.015;
 return textureSample(scene,scene_sampler,clamp(i.uv+offset,vec2<f32>(0.0),vec2<f32>(1.0)));
}
