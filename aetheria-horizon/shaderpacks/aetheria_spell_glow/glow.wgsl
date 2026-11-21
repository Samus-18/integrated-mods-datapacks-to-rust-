// Aetheria original Horizon shader API 0.2 OPTIONAL POST pack.
// This uses the documented PostInput/previous/linear_sampler/frame API.
// Never loaded as a guest WASM PipelineQuad material; shaderpacks are a
// separate user-selectable category and only one Horizon pack runs at once.
// Designed to preserve contrast on RX580-class hardware, not a screen flash.
override magic_glow:f32 = 0.28;
override highlight_gate:f32 = 0.90;
override magic_vibrance:f32 = 0.10;

fn magic_bright(uv:vec2<f32>)->vec3<f32> {
    let coord=clamp(uv,vec2<f32>(0.002),vec2<f32>(0.998));
    let c=max(textureSampleLevel(previous,linear_sampler,coord,0.0).rgb,
        vec3<f32>(0.0));
    let lum=dot(c,frame.luminance);
    let bright=smoothstep(highlight_gate,highlight_gate+0.60,lum);
    let maxc=max(c.r,max(c.g,c.b));
    let minc=min(c.r,min(c.g,c.b));
    let chroma=maxc-minc;
    // Allow warm sun/lava to glow, but favour the coloured emitted spells,
    // and avoid making pale daytime clouds one enormous white halo.
    let weight=mix(0.16,0.92,smoothstep(0.08,0.48,chroma));
    return c*bright*weight;
}
fn post(input:PostInput)->vec4<f32>{
    if magic_glow<=0.0 && magic_vibrance<=0.0 {return input.color;}
    let texel=vec2<f32>(1.0/max(1.0,frame.resolution.x),
                        1.0/max(1.0,frame.resolution.y));
    // Only EIGHT samples per pixel; no quadratic Gaussian kernel,
    // ray tracing, motion blur, scene distortion or undocumented resources.
    var halo=vec3<f32>(0.0);
    for(var i=0u;i<8u;i=i+1u){
        let phi=f32(i)*0.785398163;
        let radius=select(2.4,5.6,(i&1u)==1u);
        let offset=vec2<f32>(cos(phi),sin(phi))*texel*radius;
        halo+=magic_bright(input.uv+offset);
    }
    let base=max(input.color.rgb,vec3<f32>(0.0));
    let chroma=max(base.r,max(base.g,base.b))-
        min(base.r,min(base.g,base.b));
    let lively=smoothstep(0.08,0.48,chroma);
    let lum=dot(base,frame.luminance);
    let colourful=mix(vec3<f32>(lum),base,
        1.0+magic_vibrance*lively);
    let result=colourful+halo*(magic_glow/7.5);
    return vec4<f32>(result,1.0);
}
