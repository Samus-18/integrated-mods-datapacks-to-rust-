// Aetheria M37.2 — 11 distinct procedural hero spell MATERIALS.
// ORIGINAL Horizon guest WGSL, NOT a Java/Fabric port or a Blender screenshot.
// This is an OPTIONAL, depth-tested camera-facing world-space VFX material.
// Each pixel reconstructs the NAME'S actual silhouette and evolving energy;
// the world-quad is only a proxy surface carrying UV and material identity.
// The game still requires its actual Horizon GPU video/photographic QA.
// No scene-color, screen-depth, shadow or invented fullscreen HDR APIs.
//
// input.color.a encodes one of 11 named materials: 23+21*i (8-bit).
// input.color.rgb remains the school's themed palette. The material must
// NEVER render as a plain unmasked WorldQuad on shader initialization failure.

fn a37_hash(p:vec2<f32>)->f32{
    return fract(sin(dot(p,vec2<f32>(127.1,311.7)))*43758.5453);
}
fn a37_noise(p:vec2<f32>)->f32 {
    let i=floor(p);
    let f=fract(p);
    let u=f*f*(3.0-2.0*f);
    return mix(
        mix(a37_hash(i),a37_hash(i+vec2<f32>(1.0,0.0)),u.x),
        mix(a37_hash(i+vec2<f32>(0.0,1.0)),
            a37_hash(i+vec2<f32>(1.0,1.0)),u.x),u.y);
}
fn a37_fbm(p:vec2<f32>)->f32 {
    return 0.56*a37_noise(p)+0.28*a37_noise(p*2.13+3.17)
        +0.16*a37_noise(p*4.37-9.11);
}
fn a37_seg(p:vec2<f32>,a:vec2<f32>,b:vec2<f32>)->f32 {
    let pa=p-a; let ba=b-a;
    let t=clamp(dot(pa,ba)/max(dot(ba,ba),0.0001),0.0,1.0);
    return length(pa-ba*t);
}
fn a37_line(d:f32,w:f32)->f32{
    return 1.0-smoothstep(max(w*0.58,0.005),w+0.045,d);
}
fn a37_ellipse(p:vec2<f32>,r:vec2<f32>)->f32{
    return length(p/r);
}
fn a37_edge(p:vec2<f32>)->f32{
    let x=1.0-smoothstep(0.77,1.02,abs(p.x));
    let y=1.0-smoothstep(0.76,1.02,abs(p.y));
    return x*y;
}
fn a37_shape(which:i32,p:vec2<f32>,time:f32)->vec3<f32>{
    let t=time*0.66;
    let n=a37_fbm(p*vec2<f32>(5.4,6.7)+vec2<f32>(t*0.37,-t*0.23));
    let jitter=(n-0.5)*0.13;
    var body=0.0;
    var rim=0.0;
    var aura=0.0;

    if(which==0){
        // EMBER NEEDLE — a wide molten forged spearhead, tapered dart and
        // spiralling pyre wing; pointed silhouette, NOT a skinny glowing rod.
        let q=vec2<f32>(p.x*0.83+p.y*0.35,p.y);
        let spine=a37_seg(q,vec2<f32>(0.0,-0.95),vec2<f32>(0.0,0.43));
        let tip_width=max(0.0,0.36*(0.98-q.y));
        let head=(1.0-smoothstep(tip_width*0.56,tip_width+0.015,abs(q.x)))*
            smoothstep(0.09,0.37,q.y)*(1.0-smoothstep(0.88,1.0,q.y));
        let forged=a37_line(spine,0.105+0.07*n)*
            (1.0-smoothstep(0.45,0.87,q.y));
        let flame_path=q.x-0.34*sin((q.y+0.8)*7.0-t*2.2);
        let flametip=a37_line(abs(flame_path),0.095+0.10*n)*
            (1.0-smoothstep(0.50,0.90,abs(q.y)));
        body=max(head,forged);
        rim=clamp(head*0.90+flametip*0.64,0.0,1.0);
        aura=0.42*flametip+0.16*n*body;

    }else if(which==1){
        // PYRE SERPENT — connected moving S-shaped fire creature, curled tail,
        // broad neck and actual jaw/head; side flames animate from its spine.
        let y=p.y;
        let path=0.43*sin((y+0.16)*4.4-t*0.38)+
            0.14*sin((y+0.8)*8.8-t*0.62);
        let taper=0.16+0.13*smoothstep(-0.88,-0.02,y);
        let range=smoothstep(-0.94,-0.73,y)*
            (1.0-smoothstep(0.54,0.78,y));
        let torso=(1.0-smoothstep(taper*0.65,taper+0.050,
            abs(p.x-path+jitter)))*range;
        let head_origin=vec2<f32>(0.45+0.04*sin(t*0.31),0.57);
        let head=1.0-smoothstep(0.85,1.10,
            a37_ellipse(p-head_origin,vec2<f32>(0.42,0.27)));
        let jaw=a37_line(a37_seg(p,
            head_origin+vec2<f32>(0.02,-0.12),
            head_origin+vec2<f32>(0.44,-0.24)),0.060);
        let eye=exp(-dot((p-head_origin-vec2<f32>(0.14,0.08))*
            vec2<f32>(13.0,16.0),
            (p-head_origin-vec2<f32>(0.14,0.08))*
            vec2<f32>(13.0,16.0)));
        let flame_body=a37_line(abs(p.x-path-
            0.23*sin(y*16.0+t*2.1)),0.125+0.08*n)*range;
        body=max(torso,head*0.95);
        rim=max(flame_body*0.79,clamp(jaw+eye,0.0,1.0));
        aura=0.55*flame_body+0.16*n*torso;

    }else if(which==2){
        // SNOWBLIND — storm shell, snowflake heart, expanding ice shards.
        let r=length(p);
        let theta=atan2(p.y,p.x);
        let spin=sin(theta*4.0+r*13.0-t*2.7);
        let sheets=(1.0-smoothstep(0.10,0.43,abs(spin)))*
            (1.0-smoothstep(0.56,1.00,r));
        let spines=1.0-smoothstep(0.026,0.110,
            abs(sin(theta*6.0))*r);
        let crystal=spines*(1.0-smoothstep(0.19,0.72,r));
        let core=1.0-smoothstep(0.05,0.29,r);
        let dust=a37_noise(p*18.0+vec2<f32>(t*2.0,-t))*sheets;
        body=max(sheets*0.73,crystal);
        rim=max(core,max(crystal*0.95,dust*0.63));
        aura=sheets*0.51+0.31*core;

    }else if(which==3){
        // TWISTER SEED — filled widening wind funnel with rotating gas ribs.
        let y=clamp((p.y+0.83)/1.76,0.0,1.0);
        let w=0.13+0.65*y;
        let twist=p.x+0.16*sin(p.y*12.0-t*1.8);
        let funnel=(1.0-smoothstep(w*0.83,w+0.075,abs(twist)))*
            smoothstep(-0.98,-0.72,p.y)*(1.0-smoothstep(0.89,1.02,p.y));
        let bands=pow(max(0.0,0.5+0.5*sin(
            p.y*21.0+twist*8.0-t*4.0)),2.0);
        let seed=1.0-smoothstep(0.0,0.19,
            length(p-vec2<f32>(0.0,-0.76)));
        body=funnel*(0.40+0.42*bands);
        rim=max(seed,funnel*bands*0.94);
        aura=0.42*funnel+0.30*n*bands;

    }else if(which==4){
        // FAULTLINE — continuous jagged molten fracture splitting stone.
        let jagged=0.13*sin(p.x*9.0+t*0.24)+
            0.065*sin(p.x*21.0)+0.04*sin(p.x*43.0+t*0.3);
        let crack=abs(p.y-jagged);
        let ruptured=1.0-smoothstep(0.09,0.23,crack);
        let veins=a37_line(crack,0.061+0.03*n);
        let slab=1.0-smoothstep(0.42,0.94,abs(p.y))*
            (0.15+0.19*a37_fbm(p*vec2<f32>(11.0,6.0)));
        body=max(ruptured*0.58,slab*0.25);
        rim=veins;
        aura=ruptured*(0.29+0.43*n);

    }else if(which==5){
        // RAINWEAVER — THREE braided water streams with foam and droplets.
        var woven=0.0;
        var foam=0.0;
        for(var k=0;k<3;k=k+1){
            let i=f32(k)-1.0;
            let center=i*0.33+0.29*sin(p.y*5.3+t*1.3+
                f32(k)*2.094);
            let d=abs(p.x-center+0.035*sin(p.y*14.0-t));
            woven=max(woven,1.0-smoothstep(0.10,0.24,d));
            foam=max(foam,a37_line(d,0.11)*0.83);
        }
        let drops=step(0.84,a37_noise(p*vec2<f32>(16.0,18.0)
            +vec2<f32>(t*0.5,-t*1.4)));
        body=woven*(0.69+0.29*n);
        rim=max(foam,0.40*drops* (1.0-smoothstep(0.25,0.92,abs(p.x))));
        aura=0.31*woven+0.15*drops;

    }else if(which==6){
        // ROOT SNARE — living rooted cage with entwined thick tapering vines.
        var vines=0.0;
        var leaves=0.0;
        let y=(p.y+1.0)*0.5;
        let taper=mix(0.20,0.076,smoothstep(0.0,1.0,y));
        for(var k=0;k<4;k=k+1){
            let phase=f32(k)*1.571;
            let x=0.62*sin((p.y+0.78)*3.4+phase+t*0.23);
            let d=abs(p.x-x+jitter*0.4);
            vines=max(vines,1.0-smoothstep(taper*0.60,
                taper+0.035,d));
            let leaf_center=vec2<f32>(x+0.12*cos(phase),
                -0.68+f32(k)*0.43);
            leaves=max(leaves,1.0-smoothstep(0.76,1.07,
                a37_ellipse(p-leaf_center,vec2<f32>(0.19,0.33))));
        }
        let ground=1.0-smoothstep(0.03,0.18,abs(p.y+0.86));
        body=max(vines*0.83,leaves*0.76);
        rim=max(leaves*0.90,0.56*ground);
        aura=0.35*vines+0.20*leaves;

    }else if(which==7){
        // ARC NET — true six-node branching circuit, joined by forked
        // electrical lines, animated instability and high-energy hubs.
        let core=vec2<f32>(0.0,0.35);
        var wire=0.0;
        var sparks=0.0;
        for(var k=0;k<6;k=k+1){
            let phi=6.2831853*f32(k)/6.0+0.07*sin(t);
            let node=vec2<f32>(0.71*cos(phi),0.71*sin(phi)-0.08);
            let nxt=vec2<f32>(0.71*cos(phi+1.04719755),
                0.71*sin(phi+1.04719755)-0.08);
            let noise_offset=0.055*sin(p.x*17.0+p.y*11.0+
                f32(k)*3.2+t*3.1);
            let j=vec2<f32>(noise_offset,-noise_offset*0.5);
            wire=max(wire,a37_line(a37_seg(p+j,node,core),0.026));
            wire=max(wire,a37_line(a37_seg(p-j,node,nxt),0.023));
            sparks=max(sparks,1.0-smoothstep(0.02,0.13,
                length(p-node)));
        }
        body=wire*0.83;
        rim=max(wire*0.94,sparks);
        aura=wire*0.38+0.36*sparks;

    }else if(which==8){
        // RIFT SLICE — elongated torn black void with jagged purple lips,
        // distorted luminous plasma and fragmented spatial glass.
        let diagonal=p.y-0.36*p.x-
            0.13*sin(p.x*7.3+t*0.28)-0.04*sin(p.x*23.0+t);
        let d=abs(diagonal);
        let lengthmask=1.0-smoothstep(0.73,1.00,abs(p.x));
        let tear=1.0-smoothstep(0.20,0.36,d);
        let lips=a37_line(abs(d-0.21),0.052);
        let lightning=a37_line(d,0.065+0.025*n);
        let shards=step(0.79,a37_noise(p*vec2<f32>(12.0,10.0)+t*0.11))
            *(1.0-smoothstep(0.31,0.86,d));
        body=tear*lengthmask;
        rim=max(lips*lengthmask,lightning*0.89*lengthmask);
        aura=0.45*shards+0.34*lips;

    }else if(which==9){
        // MOMENT SLIP — fractured clock arcs, discrete skipped second marks,
        // duplicated time hands and dislocated chrono shards.
        let r=length(p);
        let a=atan2(p.y,p.x);
        let tick=floor(t*4.4);
        let clock=abs(r-0.70);
        let arc=1.0-smoothstep(0.032,0.09,clock);
        let gaps=step(0.25,sin(a*9.0+tick*0.21));
        let hand=a37_seg(p,vec2<f32>(0.0),
            vec2<f32>(0.52*cos(tick*0.10),0.52*sin(tick*0.10)));
        let echo=a37_seg(p,vec2<f32>(-0.24,-0.11),
            vec2<f32>(0.42*cos(tick*0.10-1.0),
                      0.42*sin(tick*0.10-1.0)));
        let markings=1.0-smoothstep(0.0,0.06,
            abs(fract((a+3.14)*1.90986)-0.5));
        let shards=step(0.72,a37_noise(p*15.0+
            vec2<f32>(tick*0.1,tick*0.07)));
        body=arc*gaps*0.85;
        rim=max(a37_line(hand,0.046),a37_line(echo,0.027)*0.66);
        aura=arc*markings+0.52*shards*
            (1.0-smoothstep(0.48,0.92,r));

    }else{
        // CARRION SWARM — ominous winged skull-shaped spectral creatures
        // following a curved dark necromantic wake, not purple glitter.
        var birds=0.0;
        var eyes=0.0;
        for(var k=0;k<5;k=k+1){
            let f=f32(k);
            let center=vec2<f32>(
                0.56*sin(f*2.3+t*0.24),
                0.48*cos(f*1.7+t*0.14));
            let q=p-center;
            let wing_a=a37_seg(q,vec2<f32>(0.0),
                vec2<f32>(-0.32,0.16*sin(t*1.5+f)));
            let wing_b=a37_seg(q,vec2<f32>(0.0),
                vec2<f32>(0.32,0.16*sin(t*1.5+f)));
            let skull=1.0-smoothstep(0.65,1.05,
                a37_ellipse(q,vec2<f32>(0.11,0.16)));
            birds=max(birds,max(a37_line(wing_a,0.056),
                max(a37_line(wing_b,0.056),skull)));
            let eye_p=q-vec2<f32>(0.032,0.045);
            eyes=max(eyes,1.0-smoothstep(0.02,0.068,length(eye_p)));
        }
        let smoke=1.0-smoothstep(0.20,0.82,
            length(p-vec2<f32>(0.16*sin(t*0.17),0.0)));
        body=birds*0.87;
        rim=max(eyes,0.50*birds*n);
        aura=0.33*smoke*(0.5+0.5*n);
    }
    return vec3<f32>(clamp(body,0.0,1.0),
        clamp(rim,0.0,1.0),clamp(aura,0.0,1.0));
}
fn pipeline_fragment(input:PipelineInput)->vec4<f32>{
    let p=input.uv*2.0-vec2<f32>(1.0);
    let time=frame.time;
    let id=clamp(i32(round((input.color.a*255.0-23.0)/21.0)),0,10);
    let shape=a37_shape(id,p,time);
    let softness=a37_edge(p);
    let tint=max(input.color.rgb,vec3<f32>(0.0));
    // Animated, visibly layered emission: dense hero, luminous edge,
    // drifting haze. No monotonous uniform glowy dot field.
    let n=a37_fbm(p*6.2+vec2<f32>(time*0.21,-time*0.16));
    let body=shape.x;
    let rim=shape.y;
    let haze=shape.z;
    let inner=pow(clamp(body*0.65+rim,0.0,1.0),2.0);
    let glow=tint*(0.67*body+1.15*rim+0.44*haze)+
        vec3<f32>(0.24,0.29,0.34)*(0.34+0.66*n)*inner;
    let alpha=clamp((0.78*body+0.70*rim+0.33*haze)*
        softness*(0.78+0.22*n),0.0,0.90);
    // Exact zero alpha at proxy boundaries; never show a rectangular card.
    return vec4<f32>(min(glow,vec3<f32>(1.68)),alpha);
}
