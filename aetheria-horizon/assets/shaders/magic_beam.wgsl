// Dual energy beam texture, requiring a vertex/index mesh and HDR render integration.
struct Params{time:f32,intensity:f32,beam_length:f32,pad:f32}
@group(0) @binding(0) var<uniform> params:Params;
struct VertexIn{@location(0) position:vec3<f32>,@location(1) uv:vec2<f32>}
struct VertexOut{@builtin(position) clip:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn vs_main(v:VertexIn)->VertexOut{
 var o:VertexOut;o.clip=vec4<f32>(v.position,1.0);o.uv=v.uv;return o;
}
@fragment fn fs_main(i:VertexOut)->@location(0) vec4<f32>{
 let pattern=sin(i.uv.y*26.0-params.time*6.0+sin(i.uv.x*17.0+params.time)*1.4)*0.5+0.5;
 let core=pow(1.0-abs(i.uv.x*2.0-1.0),7.0);
 let strength=(0.3+pattern*0.7+core*2.0)*params.intensity;
 return vec4<f32>(vec3<f32>(0.18,0.46,1.0)*strength+vec3<f32>(core*3.0),clamp(strength,0.0,1.0));
}
