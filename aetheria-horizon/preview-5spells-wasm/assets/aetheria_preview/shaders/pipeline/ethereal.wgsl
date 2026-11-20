// Aetheria M30 — original animated MATERIAL on the real Horizon world
// sculpture renderer (NOT a fullscreen shader pack or fake raymarch).
// Verified pinned Horizon PipelineInput has uv, linear color, camera-relative
// position (world blocks), and frame.time (seconds). No depth texture or
// camera matrices are exposed to this guest fragment: world occlusion comes
// from RenderPipeline::WorldDepth::Tested instead. No HDR/bloom claim.
// Real 3D mass comes from each Rust source scene, not drawn rods/rings here.
//
// Material goal: DIFFERENT physical motion for fire, frost, organic roots,
// storm and astral void. Each pigment responds to distinct original motion
// fields rather than a single moving noise + re-color. Avoid pure-white glare.
fn aetheria_hash2(p:vec2<f32>)->f32 {
    return fract(sin(dot(p,vec2<f32>(127.1,311.7)))*43758.5453);
}
fn aetheria_noise2(p:vec2<f32>)->f32 {
    let cell=floor(p);
    let v=fract(p);
    let u=v*v*(vec2<f32>(3.0)-2.0*v);
    let h00=aetheria_hash2(cell);
    let h10=aetheria_hash2(cell+vec2<f32>(1.0,0.0));
    let h01=aetheria_hash2(cell+vec2<f32>(0.0,1.0));
    let h11=aetheria_hash2(cell+vec2<f32>(1.0,1.0));
    return mix(mix(h00,h10,u.x),mix(h01,h11,u.x),u.y);
}
fn aetheria_facet_wave(p:vec2<f32>,t:f32)->f32 {
    // Wide, coherent moving foliation across a full solid face.
    let fold=sin(p.x*5.4+p.y*3.1+t*0.54);
    let shear=sin(p.x*2.6-p.y*6.7-t*0.37);
    return 0.5+0.25*fold+0.25*shear;
}
fn aetheria_ember_flow(uv:vec2<f32>,t:f32)->f32 {
    let lift=vec2<f32>(uv.x*4.1,uv.y*4.3-t*1.67);
    let curl=vec2<f32>(
        0.27*sin(lift.y*1.7+t*0.35),
        0.19*sin(lift.x*2.1-t*0.21));
    let heat=aetheria_noise2(lift+curl);
    let tongue=0.5+0.5*sin((uv.x+0.2*heat)*11.0-uv.y*5.4+t*3.9);
    return smoothstep(0.40,0.94,0.62*heat+0.38*tongue);
}
fn aetheria_glacier_cleavage(uv:vec2<f32>,t:f32)->f32 {
    let p=uv*vec2<f32>(8.0,6.4);
    let split=abs(sin(p.x*0.96+p.y*1.43+0.38*sin(p.y*1.9-t*0.21)));
    let stress=abs(sin(p.y*1.18-p.x*0.47+t*0.19));
    return (1.0-smoothstep(0.80,0.992,split*0.63+stress*0.37));
}
fn aetheria_living_sap(uv:vec2<f32>,t:f32)->f32 {
    let p=uv*vec2<f32>(5.2,8.1);
    let growth=sin(p.y*0.76+sin(p.x*1.2+t*0.26)*1.52);
    let branching=sin(p.x*2.74+p.y*1.89+0.54*sin(t*0.19+p.y));
    return smoothstep(0.49,0.96,0.5+0.26*growth+0.24*branching);
}
fn aetheria_lightning_branches(uv:vec2<f32>,t:f32)->f32 {
    let p=uv*vec2<f32>(7.2,5.1);
    let jitter=aetheria_noise2(p*0.7+vec2<f32>(t*0.61,-t*0.17));
    let bolt=abs(sin(p.x*2.2+p.y*0.81+2.1*jitter+t*2.4));
    let fork=abs(sin(p.y*3.2-p.x*0.44+jitter*3.0-t*1.1));
    return (1.0-smoothstep(0.04,0.24,min(bolt,fork)))*
        (0.54+0.46*sin(t*9.1+p.y*1.7)*sin(t*9.1+p.y*1.7));
}
fn aetheria_rift_fault(uv:vec2<f32>,t:f32)->f32 {
    let p=uv*vec2<f32>(6.8,8.5);
    let bending=sin(p.x*0.7+p.y*0.51+t*0.18);
    let rupture=abs(sin(p.x*1.63-p.y*1.16+1.4*bending));
    return 1.0-smoothstep(0.01,0.22,rupture);
}
// M31 test VFX: a small genuine, bounded PROCEDURAL 3D RAY-MARCH
// through each authored low-alpha atmosphere sheet. Unlike the old flat
// vertex-colour noise, each fragment integrates a rotating, density-varying
// three-dimensional medium (local UV, volume depth), not just a gradient.
// No scene-color/depth texture exists in this guest pipeline: opacity and
// world occlusion are handled conservatively by the actual depth-tested
// 3D quad. This is NOT a native fullscreen HDR raymarch or global bloom.
fn aetheria_haze_density(p:vec3<f32>,t:f32)->f32 {
    // Folded curtains of fantasy nebula, tapering at the edge of the
    // authored surface. High spatial frequencies are deliberately muted
    // to keep this a luminous body instead of a sparkly wire lattice.
    let sway=vec2<f32>(
        0.20*sin(p.y*2.4-p.z*2.6+t*0.63),
        0.18*cos(p.x*2.2+p.z*3.1-t*0.47));
    let q=p.xy+sway;
    let slow=aetheria_noise2(q*2.6+vec2<f32>(t*0.13,-t*0.17));
    let curl=0.5+0.5*sin(q.x*6.1+q.y*2.9+p.z*8.2+t*0.81);
    let depth_window=smoothstep(0.0,0.17,p.z)*
        (1.0-smoothstep(0.83,1.0,p.z));
    let lateral=1.0-smoothstep(0.65,1.1,abs(p.x));
    return clamp((0.20+0.52*slow+0.32*curl)*
        depth_window*lateral,0.0,1.0);
}
fn aetheria_integrate_haze(uv:vec2<f32>,position:vec3<f32>,
                           t:f32)->vec4<f32> {
    // Camera-relative position is used ONLY to select LOD. The medium
    // itself is in UV + authored local sheet depth, so it does not
    // swim across faces when the player moves in world coordinates.
    let distance=length(position);
    let steps=select(6u,10u,distance<34.0);
    let ds=1.0/f32(steps);
    var tr=1.0;
    var lum=vec3<f32>(0.0);
    let local=uv*2.0-vec2<f32>(1.0);
    for(var i=0u;i<10u;i=i+1u) {
        if(i>=steps){break;}
        let z=(f32(i)+0.5)*ds;
        let p=vec3<f32>(local*vec2<f32>(1.1,1.55),z);
        let dens=aetheria_haze_density(p,t);
        let optical=1.0-exp(-dens*ds*1.72);
        let contribution=tr*optical;
        // A soft inner light near its folded centre: coloured by the
        // *spell's own pigment* later, not a hard-coded ice tint.
        let hot=exp(-dot(local,local)*0.72)*
            (0.70+0.30*sin(t*0.74+z*4.6));
        lum+=vec3<f32>(0.63+0.40*hot,0.74+0.20*hot,
                       0.89+0.16*hot)*contribution;
        tr*=1.0-optical;
        if(tr<0.03){break;}
    }
    let opacity=clamp(1.0-tr,0.0,1.0);
    return vec4<f32>(lum,opacity);
}
fn pipeline_fragment(input:PipelineInput)->vec4<f32> {
    let uv=input.uv;
    let t=frame.time;
    // UV keeps detail attached to authored 3D face while the small 3D term
    // adds spatial variation without sampling the world or hiding the model.
    // PipelineInput.position is camera-relative (NOT an absolute UV anchor).
    let pos=input.position*0.16;
    let p=uv+0.036*vec2<f32>(
        sin(pos.y*1.31+t*0.22),cos(pos.x*1.17-t*0.18));
    let c=clamp(input.color.rgb,vec3<f32>(0.0),vec3<f32>(1.0));

    // Geometric facet normals are recovered FROM THE DRAWN 3D SURFACE in
    // fragment derivatives, not fake 2D circle outlines. The absolute
    // facing term supports both sides of crystals, wings and fog sheets.
    let dx=dpdx(input.position);
    let dy=dpdy(input.position);
    let face_normal=normalize(cross(dx,dy)+vec3<f32>(0.00001,0.00002,0.00003));
    let view_dir=normalize(-input.position+vec3<f32>(0.00001));
    let angle=abs(dot(face_normal,view_dir));
    let rim=pow(clamp(1.0-angle,0.0,1.0),3.0);
    let facet_light=0.45+0.55*abs(dot(
        face_normal,normalize(vec3<f32>(0.39,0.85,0.35))));
    let r=c.r;let g=c.g;let b=c.b;

    // Unnormalised weights: each magical material contributes based on
    // actual authored per-face pigments, never a slot hash or school ID.
    // These are continuous so alternate violet/teal ice faces shimmer too.
    let fire=smoothstep(0.12,0.49,r-b)*
             smoothstep(-0.09,0.28,r-g);
    let frost=smoothstep(0.07,0.38,b-r)*
              smoothstep(-0.05,0.26,g-r);
    let life=smoothstep(0.08,0.38,g-max(r,b));
    let lightning=smoothstep(0.07,0.29,b-r)*
                  smoothstep(-0.01,0.23,b-g);
    let astral=smoothstep(0.04,0.31,b-g)*
               smoothstep(-0.03,0.20,r-g);
    let mass=max(0.0,1.0-max(max(fire,frost),max(life,max(lightning,astral))));

    let faceted=aetheria_facet_wave(p*1.4,t);
    let embers=aetheria_ember_flow(p,t);
    let glacial=aetheria_glacier_cleavage(p,t);
    let sap=aetheria_living_sap(p,t);
    let voltage=aetheria_lightning_branches(p,t);
    let fracture=aetheria_rift_fault(p,t);

    // Large opaque volume surfaces remain readable; only the material's
    // local energy veins are brighter. Never white-out the whole quad.
    let brightness=0.53+0.17*facet_light+0.18*faceted+
        0.10*rim*(frost+astral+lightning)+
        0.25*fire*embers+
        0.16*frost*glacial+
        0.18*life*sap+
        0.17*lightning*voltage+
        0.12*astral*fracture+
        0.09*mass;
    let ember_color=vec3<f32>(0.25,0.046,0.004)*fire*embers;
    let ice_color=vec3<f32>(0.009,0.16,0.23)*frost*glacial+
        vec3<f32>(0.03,0.10,0.17)*rim*frost;
    let sap_color=vec3<f32>(0.035,0.12,0.012)*life*sap;
    let storm_color=vec3<f32>(0.10,0.18,0.29)*lightning*voltage;
    let rift_color=vec3<f32>(0.07,0.018,0.16)*astral*fracture;

    // 1.14 linear cap: small saturated highlights where supported, no
    // uncontrolled white flash, always preserve the base spell pigment.
    let color=min(c*brightness+
        ember_color+ice_color+sap_color+storm_color+rift_color,
        vec3<f32>(1.14));
    let a=clamp(input.color.a*(0.94+0.055*faceted),0.0,0.88);
    // Transparent atmospheric sheets use an actual local 3D density
    // integral; solid crystal facets keep their face-resolved material.
    // These surfaces already pass Horizon's WorldDepth::Tested so nearer
    // blocks suppress their coverage, instead of bogus ThroughWalls.
    if(input.color.a>0.005 && input.color.a<0.22) {
        let volume=aetheria_integrate_haze(input.uv,input.position,t);
        let haze_color=min(c*(0.58+1.62*volume.rgb)+
            0.045*vec3<f32>(c.b,c.g,c.r)*volume.a,
            vec3<f32>(1.14));
        let haze_alpha=clamp(input.color.a*
            (0.48+1.30*volume.a),0.0,0.31);
        return vec4<f32>(haze_color,haze_alpha);
    }
    return vec4<f32>(max(color,vec3<f32>(0.0)),a);
}
