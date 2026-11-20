// M35: original soft-edged, camera-facing GPU particle material.
// Standalone OPTIONAL Horizon guest render pipeline (not a postprocess).
// No scene texture/depth sampling, fabricated framebuffer or fake HDR API.
// Geometry is depth-tested by WorldDepth::Tested at pipeline registration.
// Input.color.a is a local material code, not unmasked billboard opacity.
//
// 64/255 -> thermal flame, 79/255 -> icy star/ion spark,
// 91/255 -> droplet/dust, 110/255 -> drifting spectral wisp.
// Because this is a SEPARATE shader there is no alpha-tag collision with
// existing solid meshes/curtains in ethereal.wgsl.
fn aetheria_soft_hash(p:vec2<f32>)->f32 {
    return fract(sin(dot(p,vec2<f32>(127.1,311.7)))*43758.5453);
}
fn aetheria_soft_noise(p:vec2<f32>)->f32 {
    let cell=floor(p);
    let f=fract(p);
    let w=f*f*(3.0-2.0*f);
    return mix(
        mix(aetheria_soft_hash(cell),
            aetheria_soft_hash(cell+vec2<f32>(1.0,0.0)),w.x),
        mix(aetheria_soft_hash(cell+vec2<f32>(0.0,1.0)),
            aetheria_soft_hash(cell+vec2<f32>(1.0,1.0)),w.x),w.y);
}
fn aetheria_soft_edge(uv:vec2<f32>)->f32 {
    // Hard guarantee: every rectangle is fully transparent on every edge.
    let x=smoothstep(0.0,0.10,uv.x)*(1.0-smoothstep(0.90,1.0,uv.x));
    let y=smoothstep(0.0,0.10,uv.y)*(1.0-smoothstep(0.90,1.0,uv.y));
    return x*y;
}
fn pipeline_fragment(input:PipelineInput)->vec4<f32> {
    let uv=clamp(input.uv,vec2<f32>(0.0),vec2<f32>(1.0));
    let p=uv*2.0-vec2<f32>(1.0);
    let time=frame.time;
    let seed=aetheria_soft_noise(uv*vec2<f32>(5.4,7.6)+
        vec2<f32>(time*0.26,-time*0.18));
    let fleck=aetheria_soft_noise(uv*vec2<f32>(13.2,10.3)+
        vec2<f32>(time*0.09,time*0.23));
    var shape=0.0;
    var intensity=1.0;
    if(input.color.a<0.285) {
        // Flame tongue: swaying tapered teardrop, not a crossed sprite/rod.
        let q=p.x+0.19*sin(p.y*4.1+time*2.1+seed*2.0);
        let narrowing=0.60-0.22*clamp(p.y,0.0,1.0);
        let flame_width=1.0-smoothstep(narrowing*0.48,narrowing,abs(q));
        let flame_bottom=smoothstep(-1.0,-0.65,p.y);
        let flame_top=1.0-smoothstep(0.49,1.0,p.y+0.11*seed);
        shape=flame_width*flame_bottom*flame_top;
        intensity=0.72+0.40*seed;
    }else if(input.color.a<0.335) {
        // Cold six-point glint, bright core and thin branching glacial arms.
        let x=abs(p.x);let y=abs(p.y);
        let long_arm=exp(-x*14.0)*exp(-y*1.6);
        let cross_arm=exp(-y*13.0)*exp(-x*1.7);
        let diagonal=exp(-abs(x-y)*19.0)*exp(-(x+y)*2.3);
        let core=exp(-dot(p,p)*13.0);
        shape=clamp(0.60*long_arm+0.60*cross_arm+
            0.23*diagonal+0.72*core,0.0,1.0);
        intensity=0.74+0.28*fleck;
    }else if(input.color.a<0.39) {
        // Water/dust droplet: rounded volume, asymmetric tail and soft rim.
        let displaced=p.x+0.07*sin(p.y*5.0+time*1.1);
        let r=length(vec2<f32>(displaced*1.06,p.y*0.91));
        let glisten=exp(-dot(p-vec2<f32>(-0.24,0.24),
            p-vec2<f32>(-0.24,0.24))*22.0);
        shape=(1.0-smoothstep(0.40,0.88,r+0.10*(seed-0.5)))*
            (0.82+0.18*glisten);
        intensity=0.79+0.24*glisten;
    }else{
        // Wind, flora, astral and soul: bent, torn luminous wisp.
        let curved=p.x-0.19*sin(p.y*3.1-time*1.4+seed*2.0);
        let ribbon=exp(-curved*curved*9.0)*
            (1.0-smoothstep(0.45,1.0,abs(p.y)));
        let core=exp(-(curved*curved*22.0+p.y*p.y*3.5));
        shape=clamp(ribbon*0.73+core*0.27,0.0,1.0);
        intensity=0.66+0.38*seed;
    }
    // No hard billboard edges and no uniform screen-filling white-out.
    let feather=0.69+0.31*aetheria_soft_noise(uv*8.3+
        vec2<f32>(-time*0.16,time*0.09));
    let opacity=clamp(shape*aetheria_soft_edge(uv)*feather*
        (0.64+0.18*seed),0.0,0.75);
    let base=clamp(input.color.rgb,vec3<f32>(0.0),vec3<f32>(1.0));
    let glow=base*(0.71+0.31*intensity)+
        vec3<f32>(0.075,0.095,0.105)*pow(shape,3.0);
    return vec4<f32>(min(glow,vec3<f32>(1.14)),opacity);
}
