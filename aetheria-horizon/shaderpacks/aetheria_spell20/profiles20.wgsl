// Aetheria M43, original 20-profile WGSL volumetric renderer.
// Art direction comes from the user's canonical Aetheria 200-spell registry.
// No God Wand assets, textures, GLSL or code copied.
// This module is shared ONLY by Horizon's full-screen POST stages.
// WASM .hmod custom-quad pipelines cannot do this depth-aware work.
//
// Contract for a future NATIVE Rust bridge (not yet shipped):
// frame.mods[0] = [profile_id 1..20, age_norm 0..1, intensity, active 1]
// frame.mods[1] = [center_x, center_y, center_z, bound_radius], camera-relative
// frame.mods[2] = [direction_x, direction_y, direction_z, reserved]
// frame.mods[3] may be used for future artist palette overrides.
// The public demo option works independently, but is OFF by default.
override aetheria_demo: i32 = 0;
override aetheria_cycle: bool = false;
override aetheria_bloom: f32 = 0.65;
override aetheria_quality: i32 = 1;

const FX_TAU:f32 = 6.28318530718;
const FX_PI:f32 = 3.14159265359;
struct FXState {
    id:i32,
    center:vec3<f32>,
    direction:vec3<f32>,
    phase:f32,
    power:f32,
    radius:f32,
};
struct FXVolume { light:vec3<f32>, coverage:f32, distortion:f32 };
fn fx_hash3(p:vec3<f32>)->f32 {
    return fract(sin(dot(p,vec3<f32>(127.1,311.7,74.7)))*43758.5453);
}
fn fx_noise(p:vec3<f32>)->f32 {
    let i=floor(p);
    let f=fract(p);
    let u=f*f*(vec3<f32>(3.0)-2.0*f);
    return mix(
        mix(mix(fx_hash3(i),fx_hash3(i+vec3<f32>(1.0,0.0,0.0)),u.x),
            mix(fx_hash3(i+vec3<f32>(0.0,1.0,0.0)),fx_hash3(i+vec3<f32>(1.0,1.0,0.0)),u.x),u.y),
        mix(mix(fx_hash3(i+vec3<f32>(0.0,0.0,1.0)),fx_hash3(i+vec3<f32>(1.0,0.0,1.0)),u.x),
            mix(fx_hash3(i+vec3<f32>(0.0,1.0,1.0)),fx_hash3(i+vec3<f32>(1.0,1.0,1.0)),u.x),u.y),u.z);
}
fn fx_smin(a:f32,b:f32,k:f32)->f32 {
    let h=clamp(0.5+0.5*(b-a)/max(k,0.001),0.0,1.0);
    return mix(b,a,h)-k*h*(1.0-h);
}
fn fx_box(p:vec3<f32>,b:vec3<f32>,r:f32)->f32 {
    let q=abs(p)-b;
    return length(max(q,vec3<f32>(0.0)))+min(max(q.x,max(q.y,q.z)),0.0)-r;
}
fn fx_capsule(p:vec3<f32>,a:vec3<f32>,b:vec3<f32>,r:f32)->f32 {
    let ba=b-a;
    let h=clamp(dot(p-a,ba)/max(dot(ba,ba),0.0001),0.0,1.0);
    return length(p-a-ba*h)-r;
}
fn fx_ellipsoid(p:vec3<f32>,r:vec3<f32>)->f32 {
    let k0=length(p/r);
    let k1=length(p/(r*r));
    return k0*(k0-1.0)/max(k1,0.0001);
}
fn fx_torus_y(p:vec3<f32>,r:f32,t:f32)->f32 {
    return length(vec2<f32>(length(p.xz)-r,p.y))-t;
}
fn fx_torus_z(p:vec3<f32>,r:f32,t:f32)->f32 {
    return length(vec2<f32>(length(p.xy)-r,p.z))-t;
}
fn fx_rotate(p:vec2<f32>,a:f32)->vec2<f32> {
    let s=sin(a); let c=cos(a);
    return vec2<f32>(p.x*c-p.y*s,p.x*s+p.y*c);
}
fn fx_hex(p:vec2<f32>)->f32 {
    let v=abs(p);
    return max(v.x*0.8660254+v.y*0.5,v.y);
}
fn fx_segment2(p:vec2<f32>,a:vec2<f32>,b:vec2<f32>)->f32 {
    let ba=b-a;
    let h=clamp(dot(p-a,ba)/max(dot(ba,ba),0.0001),0.0,1.0);
    return length(p-a-ba*h);
}
fn fx_phase_window(t:f32)->f32 {
    return smoothstep(0.005,0.11,t)*(1.0-smoothstep(0.79,1.0,t));
}
fn fx_read_state()->FXState {
    let src=frame.mods[0];
    if(src.w>0.5 && src.x>=1.0 && src.x<=20.0) {
        let dir=frame.mods[2].xyz;
        var aim=vec3<f32>(0.0,0.0,1.0);
        if(dot(dir,dir)>0.001) {aim=normalize(dir);}
        return FXState(i32(src.x+0.5),frame.mods[1].xyz,aim,
            clamp(src.y,0.0,1.0),clamp(src.z,0.0,2.0),
            clamp(frame.mods[1].w,2.5,9.0));
    }
    if(aetheria_demo>0 || aetheria_cycle) {
        var id=clamp(aetheria_demo,1,20);
        if(aetheria_cycle) { id=1+i32(floor(frame.time/4.0))%20; }
        let aim=normalize((frame.view_inverse*vec4<f32>(0.0,0.0,-1.0,0.0)).xyz);
        let center=aim*7.0+vec3<f32>(0.0,0.20,0.0);
        return FXState(id,center,aim,fract(frame.time/4.0),1.0,5.5);
    }
    return FXState(0,vec3<f32>(0.0),vec3<f32>(0.0,0.0,1.0),0.0,0.0,0.0);
}
fn fx_to_spell_space(p:vec3<f32>,direction:vec3<f32>)->vec3<f32> {
    let forward=normalize(direction);
    let guide=select(vec3<f32>(0.0,1.0,0.0),vec3<f32>(1.0,0.0,0.0),abs(forward.y)>0.94);
    let right=normalize(cross(guide,forward));
    let up=normalize(cross(forward,right));
    return vec3<f32>(dot(p,right),dot(p,up),dot(p,forward));
}
// Returns [signed-distance, in-body emission multiplier, local refraction].
// Every case is a separate authored spatial form, NOT one generic recoloured
// rod/ring. Large stone/ice objects also require real world meshes.
fn fx_profile(id:i32,p:vec3<f32>,t:f32)->vec3<f32> {
    let n=fx_noise(p*1.75+vec3<f32>(t*2.9,t*0.9,-t*2.1));
    if(id==1) { // Ember Needle: erupting spiralled tapered lance and escaping tongues.
        let q=p-vec3<f32>(0.12*sin(t*10.0),0.0,0.0);
        let swirl=vec2<f32>(sin(q.z*3.5+t*8.0),cos(q.z*2.9-t*7.0))*0.23;
        let len=clamp((q.z+2.6)/5.2,0.0,1.0);
        let shaft=max(length(q.xy-swirl)-(0.50-0.42*len),abs(q.z)-2.6);
        let plume=fx_ellipsoid(q-vec3<f32>(0.0,0.20,-1.2),vec3<f32>(0.68,0.98,1.7))
            +0.28*(n-0.5);
        return vec3<f32>(fx_smin(shaft,plume,0.25),1.30,0.035);
    }
    if(id==2) { // Cinder Fan: unfolding scalloped ember wings, depth-layered.
        let q=p; let ang=atan2(q.y,q.x);
        let sector=sin(7.0*ang+0.4*sin(t*9.0));
        let ring=length(q.xy*vec2<f32>(1.0,0.78));
        let petal=max(ring-(2.3+0.18*sector)*smoothstep(0.0,0.42,t),
            abs(q.z+0.28*sin(ang*4.0+t*3.0))-0.24-0.20*sector);
        let fire=fx_ellipsoid(q+vec3<f32>(0.0,0.05,0.45),
            vec3<f32>(0.63,0.70,0.80));
        return vec3<f32>(fx_smin(petal,fire,0.32)+0.09*(n-0.5),1.25,0.021);
    }
    if(id==3) { // Frost Needle: hexagonal multi-facet lance with real branches.
        let q=p;
        let body=max(fx_hex(q.xy)-mix(0.52,0.06,clamp((q.z+2.0)/4.0,0.0,1.0)),
            abs(q.z)-2.0);
        let cut=fx_capsule(q,vec3<f32>(-1.2,0.15,-0.4),vec3<f32>(0.78,0.0,0.8),0.18);
        let cut2=fx_capsule(q,vec3<f32>(0.4,-0.6,-1.1),vec3<f32>(-0.5,0.85,0.5),0.20);
        return vec3<f32>(min(body,min(cut,cut2))-0.025*n,1.65,0.009);
    }
    if(id==4) { // Rime Bridge: stepped interlocking icy slabs and icicles.
        var d=100.0;
        for(var i=0;i<6;i=i+1) {
            let z=f32(i)-2.5;
            let pos=vec3<f32>(0.16*sin(z*0.8),-0.40+0.16*sin(z*0.9+t),z*0.78);
            let q=p-pos;
            let plate=fx_box(q,vec3<f32>(0.88+0.06*f32(i%2),0.10,0.40),0.08);
            let icicle=fx_ellipsoid(q+vec3<f32>(0.5,0.38,0.0),
                vec3<f32>(0.12,0.37,0.10));
            d=min(d,fx_smin(plate,icicle,0.06));
        }
        return vec3<f32>(d,0.9,0.008);
    }
    if(id==5) { // Gale Palm: traveling pressed-air wave with vortex scallops.
        let q=p; let a=atan2(q.y,q.x);
        let ring=length(q.xy)-1.80-0.19*sin(5.0*a+t*7.0);
        let wave=length(vec2<f32>(ring,q.z-0.10*sin(a*4.0+t*7.0)))-0.35;
        let center=fx_ellipsoid(q,vec3<f32>(1.15,1.0,0.12));
        return vec3<f32>(fx_smin(wave,center,0.25),0.78,0.075);
    }
    if(id==6) { // Razor Crescent: a thick curved aerodynamic scythe.
        let q=p; let a=atan2(q.y,q.x);
        let arc=length(vec2<f32>(length(q.xy)-1.48,q.z))-0.22;
        let cut=max(abs(a-1.0)-1.93,-q.x-1.12);
        let blade=max(arc,cut);
        let edge=fx_ellipsoid(q-vec3<f32>(0.65,0.0,-0.18),
            vec3<f32>(0.72,0.21,0.19));
        return vec3<f32>(fx_smin(blade,edge,0.16),1.15,0.052);
    }
    if(id==7) { // Stone Knuckle: connected articulated fist and heavy cuboid punch.
        let q=p;
        var d=fx_box(q+vec3<f32>(0.0,0.35,0.25),vec3<f32>(1.17,0.68,0.75),0.25);
        for(var i=0;i<4;i=i+1) {
            let x=(f32(i)-1.5)*0.58;
            let finger=fx_box(q-vec3<f32>(x,0.62,-0.50),
                vec3<f32>(0.26,0.44+0.07*f32(i%2),0.46),0.20);
            d=fx_smin(d,finger,0.11);
        }
        let thumb=fx_capsule(q,vec3<f32>(-1.07,-0.3,-0.1),vec3<f32>(-0.80,0.50,-0.78),0.28);
        return vec3<f32>(fx_smin(d,thumb,0.19)+0.05*(n-0.5),0.86,0.013);
    }
    if(id==8) { // Crystal Traverse: rising faceted steps + reflective crystal spires.
        var d=100.0;
        for(var i=0;i<6;i=i+1) {
            let f=f32(i);
            let q=p-vec3<f32>(0.15*sin(f*1.7),-0.9+0.34*f,-2.0+0.72*f);
            let step=fx_box(q,vec3<f32>(0.62,0.13,0.34),0.05);
            let shard=max(fx_hex(q.xz)-0.29+0.05*q.y,abs(q.y-0.37)-0.61);
            d=min(d,fx_smin(step,shard,0.15));
        }
        return vec3<f32>(d,1.55,0.012);
    }
    if(id==9) { // Tide Needle: liquid braided jet whose volume pulses in 3D.
        let q=p;
        let spiral=vec2<f32>(0.30*sin(q.z*3.7+t*6.0),
            0.31*cos(q.z*3.2+t*6.5));
        let tube=max(length(q.xy-spiral)-(0.35+0.16*sin(q.z*4.0+t*9.0)),
            abs(q.z)-2.5);
        let lobe=max(length(q.xy+spiral*0.7)-0.30,abs(q.z)-2.0);
        return vec3<f32>(fx_smin(tube,lobe,0.17)+0.09*(n-0.5),0.92,0.061);
    }
    if(id==10) { // Flowstep: genuine moving wave crest and breaking fluid sheet.
        let q=p;
        let wave=0.52*sin(q.x*2.5+q.z*0.94-t*7.0)+0.23*sin(q.x*4.2+t*8.0);
        let water=max(abs(q.y-wave)-0.22,max(abs(q.x)-2.1,abs(q.z)-1.1));
        let crest=fx_capsule(q,vec3<f32>(-2.1,0.68,-0.2),
            vec3<f32>(2.1,0.30,0.2),0.22);
        return vec3<f32>(fx_smin(water,crest,0.24)+0.07*(n-0.5),0.72,0.065);
    }
    if(id==11) { // Thorn Dart: CLOSED botanical seed and broad overlapping leaf volumes.
        let q=p;
        var d=fx_ellipsoid(q,vec3<f32>(0.62,0.63,0.94));
        for(var i=0;i<7;i=i+1) {
            let a=FX_TAU*f32(i)/7.0+0.10*sin(t*6.0+f32(i));
            let tip=vec3<f32>(1.06*cos(a),0.97*sin(a),0.45*cos(t*3.0+f32(i)));
            let center=tip*0.56;
            let rot=fx_rotate(q.xy,-a);
            let petal=fx_ellipsoid(vec3<f32>(rot.x-center.x*0.3,rot.y,
                q.z-center.z),vec3<f32>(0.48,0.33,0.44));
            d=fx_smin(d,petal,0.22);
        }
        return vec3<f32>(d+0.06*(n-0.5),1.05,0.015);
    }
    if(id==12) { // Root Snare: branching tendrils feeding a dense leaf canopy.
        let q=p;
        var d=fx_ellipsoid(q+vec3<f32>(0.0,0.55,0.0),vec3<f32>(0.75,0.26,0.70));
        for(var i=0;i<6;i=i+1) {
            let a=FX_TAU*f32(i)/6.0;
            let e=vec3<f32>(1.68*cos(a),-0.63+0.16*sin(t*5.0+a),1.54*sin(a));
            let bend=vec3<f32>(0.54*cos(a+t*0.3),0.27,0.56*sin(a+t*0.3));
            let root=fx_capsule(q,bend,e,0.17+0.07*sin(f32(i)));
            let leaf=fx_ellipsoid(q-e*0.68,vec3<f32>(0.37,0.22,0.46));
            d=fx_smin(d,fx_smin(root,leaf,0.12),0.17);
        }
        return vec3<f32>(d+0.028*(n-0.5),0.90,0.017);
    }
    if(id==13) { // Spark Fork: jagged cloud-to-ground branching channel.
        let q=p;
        let jx=0.35*sin(q.y*4.4+t*13.0)+0.13*sin(q.y*11.3-t*5.0);
        let jz=0.27*cos(q.y*5.8-t*9.0);
        var d=max(length(q.xz-vec2<f32>(jx,jz))-0.16,abs(q.y)-2.0);
        for(var i=0;i<3;i=i+1) {
            let f=f32(i)-1.0;
            let from=vec3<f32>(jx*0.6,-0.4+f*0.42,0.0);
            let to=vec3<f32>(f*0.88,1.35+f*0.17,0.65*cos(f*2.0+t));
            d=min(d,fx_capsule(q,from,to,0.10));
        }
        return vec3<f32>(d,2.20,0.038);
    }
    if(id==14) { // Static Halo: broken spinning plasma torus, NOT an empty UI ring.
        let q=p;
        let a=atan2(q.z,q.x);
        let tor=fx_torus_y(q,1.52+0.13*sin(8.0*a+t*11.0),0.19);
        let core=fx_ellipsoid(q,vec3<f32>(0.58,0.75,0.60));
        let arc=max(tor,0.14*cos(5.0*a-t*8.0)-q.y-0.12);
        return vec3<f32>(fx_smin(arc,core,0.30)+0.075*(n-0.5),1.90,0.049);
    }
    if(id==15) { // Star Shard: offset faceted meteor pieces and deep ion center.
        let q=p;
        var d=fx_ellipsoid(q,vec3<f32>(0.48,0.63,0.48));
        for(var i=0;i<6;i=i+1) {
            let a=FX_TAU*f32(i)/6.0;
            let v=vec3<f32>(cos(a),0.55*sin(a*1.8+t*3.0),sin(a));
            let shard=fx_capsule(q,v*0.32,v*(1.30+0.19*cos(a*2.0+t)),0.29);
            d=fx_smin(d,shard,0.07);
        }
        return vec3<f32>(d,1.8,0.045);
    }
    if(id==16) { // Gravity Thread: lensing funnel with folded dark core.
        let q=p;
        let w=atan2(q.z,q.x);
        let r=length(q.xz);
        let spiral=sin(w*3.0+r*4.3-t*9.0);
        let funnel=abs(q.y+0.26*r*r-0.16*spiral)-0.18;
        let bound=r-1.75;
        let well=fx_ellipsoid(q,vec3<f32>(0.46,0.64,0.47));
        return vec3<f32>(fx_smin(max(funnel,bound),well,0.28),1.04,0.13);
    }
    if(id==17) { // Second Hand: clockwork fragments orbit a solid chrono dial.
        let q=p;
        let disk=fx_ellipsoid(q,vec3<f32>(1.42,1.42,0.25));
        let cut=fx_box(q+vec3<f32>(0.0,0.0,0.26),vec3<f32>(1.18,1.18,0.05),0.12);
        var d=max(disk,-cut);
        let hand=fx_capsule(q,vec3<f32>(0.0,0.0,0.31),
            vec3<f32>(0.86*sin(t*FX_TAU),0.86*cos(t*FX_TAU),0.31),0.12);
        d=min(d,hand);
        for(var i=0;i<7;i=i+1) {
            let a=FX_TAU*f32(i)/7.0+0.2*t;
            let pos=vec3<f32>(1.65*cos(a),1.65*sin(a),0.22*sin(a*4.0));
            d=min(d,fx_box(q-pos,vec3<f32>(0.20,0.36,0.10),0.03));
        }
        return vec3<f32>(d,1.18,0.042);
    }
    if(id==18) { // Moment Slip: displaced temporal ghosts, tearing rather than rings.
        let q=p;
        var d=100.0;
        for(var i=0;i<4;i=i+1) {
            let f=f32(i);
            let shadow=q-vec3<f32>((f-1.5)*0.52+0.16*sin(t*9.0+f),
                0.0,0.22*(f-1.5));
            let slice=fx_ellipsoid(shadow,vec3<f32>(0.55,1.18,0.17+0.04*f));
            let fracture=abs(shadow.y+0.31*sin(shadow.x*5.0+f+t*5.0))-0.22;
            d=min(d,max(slice,fracture));
        }
        return vec3<f32>(d,1.33,0.09);
    }
    if(id==19) { // Bone Thorn: hooked branching bone and fractured skeletal shards.
        let q=p;
        var d=fx_ellipsoid(q+vec3<f32>(0.0,0.45,0.0),
            vec3<f32>(0.56,0.78,0.40));
        for(var i=0;i<5;i=i+1) {
            let a=FX_TAU*f32(i)/5.0+0.3;
            let from=vec3<f32>(0.24*cos(a),0.22,0.20*sin(a));
            let tip=vec3<f32>(1.38*cos(a),0.68+0.52*sin(a*1.4),0.98*sin(a));
            let hook=tip+vec3<f32>(-0.38*cos(a),0.58,-0.19*sin(a));
            d=fx_smin(d,fx_capsule(q,from,tip,0.22),0.09);
            d=min(d,fx_capsule(q,tip,hook,0.13));
        }
        return vec3<f32>(d+0.025*(n-0.5),1.17,0.022);
    }
    if(id==20) { // Soul Lantern: filled spectral vessel and three rising wisps.
        let q=p;
        let core=fx_ellipsoid(q,vec3<f32>(0.72,1.05,0.71));
        let mantle=fx_torus_y(q+vec3<f32>(0.0,0.67,0.0),0.62,0.24);
        var d=fx_smin(core,mantle,0.16);
        for(var i=0;i<3;i=i+1) {
            let a=FX_TAU*f32(i)/3.0;
            let x=0.44*sin(q.y*2.2+t*6.0+a)+0.24*cos(a);
            let z=0.40*cos(q.y*2.1-t*5.0+a);
            let wisp=max(length(q.xz-vec2<f32>(x,z))-0.23,
                abs(q.y-1.3)-1.25);
            d=fx_smin(d,wisp,0.14);
        }
        return vec3<f32>(d+0.05*(n-0.5),1.42,0.065);
    }
    return vec3<f32>(100.0,0.0,0.0);
}
// Original colour compositions, one per exact spell. Profiles may still need
// per-spell artist tuning in game; shader compile does NOT certify quality.
fn fx_palette(id:i32,p:vec3<f32>,t:f32)->vec3<f32> {
    var a=vec3<f32>(0.22,0.52,1.0);
    var b=vec3<f32>(0.85,0.97,1.0);
    switch(id) {
        case 1: {a=vec3<f32>(1.00,0.105,0.006);b=vec3<f32>(1.90,0.74,0.09);}
        case 2: {a=vec3<f32>(0.54,0.08,0.025);b=vec3<f32>(1.8,0.47,0.06);}
        case 3: {a=vec3<f32>(0.03,0.45,0.76);b=vec3<f32>(0.65,1.55,2.05);}
        case 4: {a=vec3<f32>(0.16,0.44,0.73);b=vec3<f32>(0.79,1.03,1.48);}
        case 5: {a=vec3<f32>(0.12,0.54,0.51);b=vec3<f32>(0.75,1.23,1.10);}
        case 6: {a=vec3<f32>(0.17,0.46,0.47);b=vec3<f32>(0.84,1.76,1.57);}
        case 7: {a=vec3<f32>(0.25,0.17,0.09);b=vec3<f32>(1.12,0.71,0.32);}
        case 8: {a=vec3<f32>(0.30,0.22,0.44);b=vec3<f32>(1.12,0.96,1.66);}
        case 9: {a=vec3<f32>(0.02,0.25,0.65);b=vec3<f32>(0.36,1.11,1.90);}
        case 10: {a=vec3<f32>(0.05,0.40,0.58);b=vec3<f32>(0.68,1.55,1.75);}
        case 11: {a=vec3<f32>(0.13,0.31,0.045);b=vec3<f32>(0.57,1.25,0.16);}
        case 12: {a=vec3<f32>(0.20,0.33,0.06);b=vec3<f32>(0.98,1.35,0.33);}
        case 13: {a=vec3<f32>(0.28,0.36,1.15);b=vec3<f32>(1.23,1.7,2.45);}
        case 14: {a=vec3<f32>(0.16,0.40,0.98);b=vec3<f32>(0.68,1.60,2.37);}
        case 15: {a=vec3<f32>(0.20,0.13,0.56);b=vec3<f32>(1.05,0.82,2.15);}
        case 16: {a=vec3<f32>(0.065,0.008,0.21);b=vec3<f32>(0.83,0.34,1.42);}
        case 17: {a=vec3<f32>(0.48,0.24,0.055);b=vec3<f32>(1.87,1.44,0.54);}
        case 18: {a=vec3<f32>(0.13,0.21,0.45);b=vec3<f32>(0.82,0.95,1.95);}
        case 19: {a=vec3<f32>(0.24,0.11,0.34);b=vec3<f32>(1.10,0.66,1.40);}
        case 20: {a=vec3<f32>(0.13,0.12,0.40);b=vec3<f32>(0.73,1.16,1.62);}
        default: {}
    }
    let mottling=fx_noise(p*2.9+vec3<f32>(0.0,t*4.0,-t*1.7));
    let pulse=0.43+0.57*smoothstep(0.17,0.91,mottling);
    return mix(a,b,pulse);
}
fn fx_volume(input:PostInput,steps:i32)->FXVolume {
    let state=fx_read_state();
    if(state.id==0 || state.power<0.002) {
        return FXVolume(vec3<f32>(0.0),0.0,0.0);
    }
    // Depth and camera-space reconstruction belong to the FULLSCREEN post,
    // never to the old .hmod quad fragment stage.
    let r=normalize(view_position(input.uv,0.02));
    let along=dot(r,state.center);
    let offset2=dot(state.center,state.center)-along*along;
    let r2=state.radius*state.radius;
    if(along<0.0 || offset2>=r2) {
        return FXVolume(vec3<f32>(0.0),0.0,0.0);
    }
    let h=sqrt(max(0.0,r2-offset2));
    let enter=max(0.0,along-h);
    let exit=along+h;
    let terrain_depth=input.depth;
    var maxdist=256.0;
    if(terrain_depth>0.00001) {
        maxdist=length(view_position(input.uv,terrain_depth))-0.07;
    }
    let far=min(exit,maxdist);
    if(far<=enter) {return FXVolume(vec3<f32>(0.0),0.0,0.0);}
    let amount=select(10,select(12,18,steps>12),aetheria_quality>0);
    let increment=(far-enter)/f32(amount);
    var trans=1.0;
    var acc=vec3<f32>(0.0);
    var refraction=0.0;
    let active=fx_phase_window(state.phase)*state.power;
    if(active<=0.002){return FXVolume(vec3<f32>(0.0),0.0,0.0);}
    for(var i=0;i<18;i=i+1) {
        if(i>=amount || trans<0.035){break;}
        let distance=enter+(f32(i)+0.5)*increment;
        let world=r*distance-state.center;
        let local=fx_to_spell_space(world,state.direction);
        let m=fx_profile(state.id,local,state.phase);
        let density=(1.0-smoothstep(-0.16,0.48,m.x))*
            (0.66+0.34*fx_noise(local*3.0+vec3<f32>(state.phase*7.0)));
        let optical=clamp(density*increment*1.35*active,0.0,0.38);
        let contribution=trans*optical;
        acc+=fx_palette(state.id,local,state.phase)*
            contribution*(0.50+0.72*m.y);
        refraction+=contribution*m.z;
        trans*=1.0-optical;
    }
    return FXVolume(acc,1.0-trans,refraction);
}
