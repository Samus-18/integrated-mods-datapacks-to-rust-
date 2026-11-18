// Liquid mana material shader, not yet connected to custom HUD render pass.
struct Params{mana_fill:f32,time:f32,pad:vec2<f32>}
@group(0) @binding(0) var<uniform> params:Params;
struct Out{@builtin(position) clip:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn vs_main(@builtin(vertex_index) idx:u32)->Out{
 let vertices=array<vec2<f32>,3>(vec2<f32>(-1.0,-1.0),vec2<f32>(3.0,-1.0),vec2<f32>(-1.0,3.0));
 var o:Out;o.clip=vec4<f32>(vertices[idx],0.0,1.0);
 o.uv=vertices[idx]*0.5+vec2<f32>(0.5);return o;
}
@fragment fn fs_main(i:Out)->@location(0) vec4<f32>{
 let q=i.uv-vec2<f32>(0.5);
 let mask=1.0-smoothstep(0.45,0.455,length(q));
 let wave=sin(q.x*18.0+params.time*2.0)*0.013
          +sin(q.x*31.0-params.time*3.0)*0.006;
 let liquid=1.0-smoothstep(0.008,0.018,i.uv.y-(1.0-params.mana_fill+wave));
 let light=vec3<f32>(0.06,0.55,0.95)*liquid*3.0;
 return vec4<f32>(light+vec3<f32>(0.03,0.05,0.07),mask*0.9);
}
