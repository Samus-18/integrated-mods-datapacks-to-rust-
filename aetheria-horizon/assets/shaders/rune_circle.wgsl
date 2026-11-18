// Original standalone WGSL vertex/fragment prototype. Not integrated in Horizon yet.
struct Params {time:f32,intensity:f32,radius:f32,pad:f32}
@group(0) @binding(0) var<uniform> params:Params;
struct VertexIn{@location(0) position:vec2<f32>,@location(1) uv:vec2<f32>}
struct VertexOut{@builtin(position) clip:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn vs_main(v:VertexIn)->VertexOut{
 var o:VertexOut; o.clip=vec4<f32>(v.position,0.0,1.0);o.uv=v.uv;return o;
}
fn band(q:vec2<f32>,r:f32,w:f32)->f32{
 return 1.0-smoothstep(w,w+0.01,abs(length(q)-r));
}
@fragment fn fs_main(i:VertexOut)->@location(0) vec4<f32>{
 let q=i.uv*2.0-vec2<f32>(1.0);
 let angle=atan2(q.y,q.x)+params.time*0.3;
 let runes=1.0-smoothstep(0.04,0.09,abs(sin(angle*14.0+sin(angle*7.0))*length(q)-0.68));
 let luminous=(band(q,0.7,0.018)+band(q,0.85,0.008)+runes*0.65)*max(params.intensity,0.0);
 return vec4<f32>(vec3<f32>(0.18,0.75,1.4)*luminous*3.0,clamp(luminous,0.0,1.0));
}
