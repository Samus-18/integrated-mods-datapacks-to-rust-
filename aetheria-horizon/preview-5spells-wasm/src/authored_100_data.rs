//! M36: One hundred NAMED, original procedural VFX choreography presets.
//! Exactly first ten canonical spells of each ten schools. Source: the actual
//! 200-entry Rust spell registry. Not a seed-only colour swap or 100 Blender
//! models. Every row explicitly pairs its name/intent to a distinct physical
//! motion family with individual burst, sweep, spread, height and phase skew.
//! IMPORTANT: authored geometry preset != fully game-verified polished spell.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(super) enum Pattern {Lance,Fan,Lash,Footsteps,Cloud,Sigil,Serpent,Bloom,Orbit,Bridge,Cage,Roots,Claw,Mirror,Vortex,Sonic,Crescent,Swarm,Implode,Pulse,Wings,Jaws,Crack,Shield,Net,Barrage,Surf,Tether,Rain,Cascade,Jet,Weave,Canopy,Fork,Corona,Coil,Starfield,Fold,Veil,Comet,Compass,Clock,TimeEcho,Sandglass,Stasis,Lantern,Wisp,Pool}
#[derive(Clone,Copy,Debug)]
pub(super) struct FxAuthor {
    pub id:u16,pub name:&'static str,pub pattern:Pattern,
    pub density:u8,pub spread:f64,pub rise:f64,pub twist:f64,
    pub tempo:f64,pub phase:f64,pub sweep:f64,
}
// Coordinates are bounded world-space blocks; values are intentional
// offline art direction, not unbounded work-unit or literal spell damage.
pub(super) const AUTHOR_100:[FxAuthor;100]=[
    FxAuthor{id:1,name:"Ember Needle",pattern:Pattern::Lance,density:64,spread:1,rise:0.9,twist:0.5,tempo:0.65,phase:0.1,sweep:0.55}, // twisting ember thorn
    FxAuthor{id:2,name:"Cinder Fan",pattern:Pattern::Fan,density:76,spread:1.17,rise:1.13,twist:0.86,tempo:0.76,phase:0.176,sweep:0.68}, // seven curved coal blades
    FxAuthor{id:3,name:"Magma Lash",pattern:Pattern::Lash,density:72,spread:1.34,rise:1.36,twist:1.22,tempo:0.87,phase:0.252,sweep:0.81}, // segmented molten whip
    FxAuthor{id:4,name:"Furnace Step",pattern:Pattern::Footsteps,density:68,spread:1.51,rise:1.59,twist:0.62,tempo:0.98,phase:0.328,sweep:0.94}, // molten footstep ribbon
    FxAuthor{id:5,name:"Ashen Veil",pattern:Pattern::Cloud,density:64,spread:1.68,rise:0.9,twist:0.98,tempo:1.09,phase:0.404,sweep:1.07}, // rolling ash cloak
    FxAuthor{id:6,name:"Firebrand Sigil",pattern:Pattern::Sigil,density:76,spread:1,rise:1.13,twist:1.34,tempo:1.2,phase:0.138,sweep:1.2}, // engraved fire seal
    FxAuthor{id:7,name:"Pyre Serpent",pattern:Pattern::Serpent,density:72,spread:1.17,rise:1.36,twist:0.74,tempo:1.31,phase:0.214,sweep:1.33}, // coiled fire serpent
    FxAuthor{id:8,name:"Lava Bloom",pattern:Pattern::Bloom,density:68,spread:1.34,rise:1.59,twist:1.1,tempo:0.65,phase:0.29,sweep:1.46}, // opening magma flower
    FxAuthor{id:9,name:"Coalstorm",pattern:Pattern::Orbit,density:64,spread:1.51,rise:0.9,twist:0.5,tempo:0.76,phase:0.366,sweep:0.55}, // swirling charcoal stones
    FxAuthor{id:10,name:"Searing Javelin",pattern:Pattern::Lance,density:76,spread:1.68,rise:1.13,twist:0.86,tempo:0.87,phase:0.1,sweep:0.68}, // barbed flame lance
    FxAuthor{id:21,name:"Frost Needle",pattern:Pattern::Lance,density:76,spread:1.1,rise:0.9,twist:0.62,tempo:0.87,phase:0.138,sweep:1.07}, // thin hexagonal ice dart
    FxAuthor{id:22,name:"Rime Bridge",pattern:Pattern::Bridge,density:72,spread:1.27,rise:1.13,twist:0.98,tempo:0.98,phase:0.214,sweep:1.2}, // interlocking suspended ice plates
    FxAuthor{id:23,name:"Shard Chorus",pattern:Pattern::Fan,density:68,spread:1.44,rise:1.36,twist:1.34,tempo:1.09,phase:0.29,sweep:1.33}, // asymmetric crystal fan
    FxAuthor{id:24,name:"Glaze Prison",pattern:Pattern::Cage,density:64,spread:1.61,rise:1.59,twist:0.74,tempo:1.2,phase:0.366,sweep:1.46}, // faceted six-sided cage
    FxAuthor{id:25,name:"Hail Carousel",pattern:Pattern::Orbit,density:76,spread:1.78,rise:0.9,twist:1.1,tempo:1.31,phase:0.1,sweep:0.55}, // transparent hail asteroids
    FxAuthor{id:26,name:"Hoarfrost Trail",pattern:Pattern::Roots,density:72,spread:1.1,rise:1.13,twist:0.5,tempo:0.65,phase:0.176,sweep:0.68}, // branching frost floor vines
    FxAuthor{id:27,name:"Permafrost Grip",pattern:Pattern::Claw,density:68,spread:1.27,rise:1.36,twist:0.86,tempo:0.76,phase:0.252,sweep:0.81}, // clawed crystalline hand
    FxAuthor{id:28,name:"Sleet Mirror",pattern:Pattern::Mirror,density:64,spread:1.44,rise:1.59,twist:1.22,tempo:0.87,phase:0.328,sweep:0.94}, // three slanted ice mirrors
    FxAuthor{id:29,name:"Glacier Lance",pattern:Pattern::Lance,density:76,spread:1.61,rise:0.9,twist:0.62,tempo:0.98,phase:0.404,sweep:1.07}, // twisted glacier spear
    FxAuthor{id:30,name:"Snowblind",pattern:Pattern::Vortex,density:72,spread:1.78,rise:1.13,twist:0.98,tempo:1.09,phase:0.138,sweep:1.2}, // layered snow funnel
    FxAuthor{id:41,name:"Gale Palm",pattern:Pattern::Sonic,density:72,spread:1.2,rise:1.75,twist:0.74,tempo:1.09,phase:0.176,sweep:0.55}, // spiral air pressure palm
    FxAuthor{id:42,name:"Razor Crescent",pattern:Pattern::Crescent,density:68,spread:1.37,rise:1.98,twist:1.1,tempo:1.2,phase:0.252,sweep:0.68}, // translucent wind crescent
    FxAuthor{id:43,name:"Updraft Well",pattern:Pattern::Vortex,density:64,spread:1.54,rise:2.21,twist:0.5,tempo:1.31,phase:0.328,sweep:0.81}, // stacked rising air rings
    FxAuthor{id:44,name:"Feather Drift",pattern:Pattern::Swarm,density:76,spread:1.71,rise:2.44,twist:0.86,tempo:0.65,phase:0.404,sweep:0.94}, // floating feathers and vortex
    FxAuthor{id:45,name:"Vacuum Pin",pattern:Pattern::Implode,density:72,spread:1.88,rise:1.75,twist:1.22,tempo:0.76,phase:0.138,sweep:1.07}, // compressed transparent needle
    FxAuthor{id:46,name:"Cyclone Stride",pattern:Pattern::Footsteps,density:68,spread:1.2,rise:1.98,twist:0.62,tempo:0.87,phase:0.214,sweep:1.2}, // ribbon slipstream footsteps
    FxAuthor{id:47,name:"Banshee Whistle",pattern:Pattern::Pulse,density:64,spread:1.37,rise:2.21,twist:0.98,tempo:0.98,phase:0.29,sweep:1.33}, // concentric vibration helix
    FxAuthor{id:48,name:"Twister Seed",pattern:Pattern::Vortex,density:76,spread:1.54,rise:2.44,twist:1.34,tempo:1.09,phase:0.366,sweep:1.46}, // narrow wind funnel
    FxAuthor{id:49,name:"Aerial Mantle",pattern:Pattern::Wings,density:72,spread:1.71,rise:1.75,twist:0.74,tempo:1.2,phase:0.1,sweep:0.55}, // flowing air wings
    FxAuthor{id:50,name:"Thunderhead Draft",pattern:Pattern::Cloud,density:68,spread:1.88,rise:1.98,twist:1.1,tempo:1.31,phase:0.176,sweep:0.68}, // rotating vapor canopy
    FxAuthor{id:61,name:"Stone Knuckle",pattern:Pattern::Claw,density:68,spread:1.3,rise:0.9,twist:0.86,tempo:1.31,phase:0.214,sweep:1.07}, // articulated stone fist
    FxAuthor{id:62,name:"Crystal Traverse",pattern:Pattern::Bridge,density:64,spread:1.47,rise:1.13,twist:1.22,tempo:0.65,phase:0.29,sweep:1.2}, // floating prismatic stairs
    FxAuthor{id:63,name:"Gravel Jaws",pattern:Pattern::Jaws,density:76,spread:1.64,rise:1.36,twist:0.62,tempo:0.76,phase:0.366,sweep:1.33}, // interlocking jagged jaws
    FxAuthor{id:64,name:"Faultline",pattern:Pattern::Crack,density:72,spread:1.81,rise:1.59,twist:0.98,tempo:0.87,phase:0.1,sweep:1.46}, // branching earth seam
    FxAuthor{id:65,name:"Shard Bulwark",pattern:Pattern::Shield,density:68,spread:1.98,rise:0.9,twist:1.34,tempo:0.98,phase:0.176,sweep:0.55}, // overlapping crystal shield petals
    FxAuthor{id:66,name:"Granite Orbit",pattern:Pattern::Orbit,density:64,spread:1.3,rise:1.13,twist:0.74,tempo:1.09,phase:0.252,sweep:0.68}, // polygonal boulder orbit
    FxAuthor{id:67,name:"Ore Sense",pattern:Pattern::Net,density:76,spread:1.47,rise:1.36,twist:1.1,tempo:1.2,phase:0.328,sweep:0.81}, // subsurface crystal resonance lattice
    FxAuthor{id:68,name:"Earthbind",pattern:Pattern::Cage,density:72,spread:1.64,rise:1.59,twist:0.5,tempo:1.31,phase:0.404,sweep:0.94}, // angular stone cuffs
    FxAuthor{id:69,name:"Pebble Barrage",pattern:Pattern::Barrage,density:68,spread:1.81,rise:0.9,twist:0.86,tempo:0.65,phase:0.138,sweep:1.07}, // helical stone sling
    FxAuthor{id:70,name:"Obsidian Needle",pattern:Pattern::Lance,density:64,spread:1.98,rise:1.13,twist:1.22,tempo:0.76,phase:0.214,sweep:1.2}, // thin black monolith
    FxAuthor{id:81,name:"Tide Needle",pattern:Pattern::Lance,density:64,spread:1.4,rise:0.9,twist:0.98,tempo:0.76,phase:0.252,sweep:0.55}, // braided liquid spear
    FxAuthor{id:82,name:"Flowstep",pattern:Pattern::Surf,density:76,spread:1.57,rise:1.13,twist:1.34,tempo:0.87,phase:0.328,sweep:0.68}, // rolling ribbon surfboard
    FxAuthor{id:83,name:"Pearl Shield",pattern:Pattern::Shield,density:72,spread:1.74,rise:1.36,twist:0.74,tempo:0.98,phase:0.404,sweep:0.81}, // layered water pearl
    FxAuthor{id:84,name:"Riptide Snare",pattern:Pattern::Tether,density:68,spread:1.91,rise:1.59,twist:1.1,tempo:1.09,phase:0.138,sweep:0.94}, // coiled liquid tether
    FxAuthor{id:85,name:"Mistcloak",pattern:Pattern::Cloud,density:64,spread:2.08,rise:0.9,twist:0.5,tempo:1.2,phase:0.214,sweep:1.07}, // billowing droplet cloak
    FxAuthor{id:86,name:"Surge Fan",pattern:Pattern::Fan,density:76,spread:1.4,rise:1.13,twist:0.86,tempo:1.31,phase:0.29,sweep:1.2}, // scalloped water fan
    FxAuthor{id:87,name:"Rainweaver",pattern:Pattern::Rain,density:72,spread:1.57,rise:1.36,twist:1.22,tempo:0.65,phase:0.366,sweep:1.33}, // suspended rain lattice
    FxAuthor{id:88,name:"Foam Cascade",pattern:Pattern::Cascade,density:68,spread:1.74,rise:1.59,twist:0.62,tempo:0.76,phase:0.1,sweep:1.46}, // stacked foaming breakers
    FxAuthor{id:89,name:"Abyss Jet",pattern:Pattern::Jet,density:64,spread:1.91,rise:0.9,twist:0.98,tempo:0.87,phase:0.176,sweep:0.55}, // cavitating liquid lance
    FxAuthor{id:90,name:"Whirlpool Ring",pattern:Pattern::Vortex,density:76,spread:2.08,rise:1.13,twist:1.34,tempo:0.98,phase:0.252,sweep:0.68}, // nested rotating liquid toroids
    FxAuthor{id:101,name:"Thorn Dart",pattern:Pattern::Lance,density:76,spread:1.5,rise:0.9,twist:1.1,tempo:0.98,phase:0.29,sweep:1.07}, // barbed spinning seed
    FxAuthor{id:102,name:"Root Snare",pattern:Pattern::Roots,density:72,spread:1.67,rise:1.13,twist:0.5,tempo:1.09,phase:0.366,sweep:1.2}, // knotted branching roots
    FxAuthor{id:103,name:"Moss Mend",pattern:Pattern::Weave,density:68,spread:1.84,rise:1.36,twist:0.86,tempo:1.2,phase:0.1,sweep:1.33}, // living moss weave
    FxAuthor{id:104,name:"Briar Loop",pattern:Pattern::Roots,density:64,spread:2.01,rise:1.59,twist:1.22,tempo:1.31,phase:0.176,sweep:1.46}, // braided thorn wreath
    FxAuthor{id:105,name:"Firefly Veil",pattern:Pattern::Swarm,density:76,spread:2.18,rise:0.9,twist:0.62,tempo:0.65,phase:0.252,sweep:0.55}, // light insect veil
    FxAuthor{id:106,name:"Seedburst",pattern:Pattern::Bloom,density:72,spread:1.5,rise:1.13,twist:0.98,tempo:0.76,phase:0.328,sweep:0.68}, // radial seed pod cluster
    FxAuthor{id:107,name:"Vine Swing",pattern:Pattern::Serpent,density:68,spread:1.67,rise:1.36,twist:1.34,tempo:0.87,phase:0.404,sweep:0.81}, // growing jointed vine rope
    FxAuthor{id:108,name:"Barkskin",pattern:Pattern::Shield,density:64,spread:1.84,rise:1.59,twist:0.74,tempo:0.98,phase:0.138,sweep:0.94}, // interlocking bark plating
    FxAuthor{id:109,name:"Bloomstep",pattern:Pattern::Footsteps,density:76,spread:2.01,rise:0.9,twist:1.1,tempo:1.09,phase:0.214,sweep:1.07}, // unfurling flower path
    FxAuthor{id:110,name:"Mushroom Lantern",pattern:Pattern::Canopy,density:72,spread:2.18,rise:1.13,twist:0.5,tempo:1.2,phase:0.29,sweep:1.2}, // bioluminescent fungus canopy
    FxAuthor{id:121,name:"Spark Fork",pattern:Pattern::Fork,density:72,spread:1.6,rise:0.9,twist:1.22,tempo:1.2,phase:0.328,sweep:0.55}, // jagged spark fork
    FxAuthor{id:122,name:"Static Halo",pattern:Pattern::Corona,density:68,spread:1.77,rise:1.13,twist:0.62,tempo:1.31,phase:0.404,sweep:0.68}, // charged spinning crown
    FxAuthor{id:123,name:"Thunder Needle",pattern:Pattern::Lance,density:64,spread:1.94,rise:1.36,twist:0.98,tempo:0.65,phase:0.138,sweep:0.81}, // electric crystal needle
    FxAuthor{id:124,name:"Arc Net",pattern:Pattern::Net,density:76,spread:2.11,rise:1.59,twist:1.34,tempo:0.76,phase:0.214,sweep:0.94}, // triangular lightning lattice
    FxAuthor{id:125,name:"Storm Coil",pattern:Pattern::Coil,density:72,spread:2.28,rise:0.9,twist:0.74,tempo:0.87,phase:0.29,sweep:1.07}, // triple helical arc torus
    FxAuthor{id:126,name:"Flashstep",pattern:Pattern::Footsteps,density:68,spread:1.6,rise:1.13,twist:1.1,tempo:0.98,phase:0.366,sweep:1.2}, // stretched light silhouette
    FxAuthor{id:127,name:"Magnet Stone",pattern:Pattern::Orbit,density:64,spread:1.77,rise:1.36,twist:0.5,tempo:1.09,phase:0.1,sweep:1.33}, // runic floating lodestone
    FxAuthor{id:128,name:"Chain Arc",pattern:Pattern::Fork,density:76,spread:1.94,rise:1.59,twist:0.86,tempo:1.2,phase:0.176,sweep:1.46}, // segmented lightning tree
    FxAuthor{id:129,name:"Surge Palm",pattern:Pattern::Claw,density:72,spread:2.11,rise:0.9,twist:1.22,tempo:1.31,phase:0.252,sweep:0.55}, // electric hand glyph
    FxAuthor{id:130,name:"Bolt Weaver",pattern:Pattern::Weave,density:68,spread:2.28,rise:1.13,twist:0.62,tempo:0.65,phase:0.328,sweep:0.68}, // stitched ground filaments
    FxAuthor{id:141,name:"Star Shard",pattern:Pattern::Starfield,density:68,spread:1.7,rise:0.9,twist:1.34,tempo:0.65,phase:0.366,sweep:1.07}, // seven pointed spectral star
    FxAuthor{id:142,name:"Gravity Thread",pattern:Pattern::Tether,density:64,spread:1.87,rise:1.13,twist:0.74,tempo:0.76,phase:0.1,sweep:1.2}, // taut gravity filament
    FxAuthor{id:143,name:"Blink Fold",pattern:Pattern::Fold,density:76,spread:2.04,rise:1.36,twist:1.1,tempo:0.87,phase:0.176,sweep:1.33}, // folded rectangular spatial ribbons
    FxAuthor{id:144,name:"Moonlit Veil",pattern:Pattern::Veil,density:72,spread:2.21,rise:1.59,twist:0.5,tempo:0.98,phase:0.252,sweep:1.46}, // curved silver phase sheets
    FxAuthor{id:145,name:"Rift Slice",pattern:Pattern::Crescent,density:68,spread:2.38,rise:0.9,twist:0.86,tempo:1.09,phase:0.328,sweep:0.55}, // broken crescent rift
    FxAuthor{id:146,name:"Comet Pearl",pattern:Pattern::Comet,density:64,spread:1.7,rise:1.13,twist:1.22,tempo:1.2,phase:0.404,sweep:0.68}, // many faceted comet sphere
    FxAuthor{id:147,name:"Astral Compass",pattern:Pattern::Compass,density:76,spread:1.87,rise:1.36,twist:0.62,tempo:1.31,phase:0.138,sweep:0.81}, // rotating star compass
    FxAuthor{id:148,name:"Weightless Field",pattern:Pattern::Swarm,density:72,spread:2.04,rise:1.59,twist:0.98,tempo:0.65,phase:0.214,sweep:0.94}, // spherical floating feather field
    FxAuthor{id:149,name:"Nightglass Mirror",pattern:Pattern::Mirror,density:68,spread:2.21,rise:0.9,twist:1.34,tempo:0.76,phase:0.29,sweep:1.07}, // black mirror prism
    FxAuthor{id:150,name:"Orbiting Moons",pattern:Pattern::Orbit,density:64,spread:2.38,rise:1.13,twist:0.74,tempo:0.87,phase:0.366,sweep:1.2}, // stone orbiting moonlets
    FxAuthor{id:161,name:"Second Hand",pattern:Pattern::Clock,density:64,spread:1.8,rise:0.9,twist:0.5,tempo:0.87,phase:0.404,sweep:0.55}, // rotating brass clock hand
    FxAuthor{id:162,name:"Moment Slip",pattern:Pattern::TimeEcho,density:76,spread:1.97,rise:1.13,twist:0.86,tempo:0.98,phase:0.138,sweep:0.68}, // curving faded movement trail
    FxAuthor{id:163,name:"Tick Pulse",pattern:Pattern::Pulse,density:72,spread:2.14,rise:1.36,twist:1.22,tempo:1.09,phase:0.214,sweep:0.81}, // nested translucent watch gears
    FxAuthor{id:164,name:"Sands of Delay",pattern:Pattern::Sandglass,density:68,spread:2.31,rise:1.59,twist:0.62,tempo:1.2,phase:0.29,sweep:0.94}, // hovering inverted sandglass
    FxAuthor{id:165,name:"Temporal Mark",pattern:Pattern::Sigil,density:64,spread:2.48,rise:0.9,twist:0.98,tempo:1.31,phase:0.366,sweep:1.07}, // hourglass target rune
    FxAuthor{id:166,name:"Echo Step",pattern:Pattern::Footsteps,density:76,spread:1.8,rise:1.13,twist:1.34,tempo:0.65,phase:0.1,sweep:1.2}, // offset footstep echoes
    FxAuthor{id:167,name:"Aging Touch",pattern:Pattern::Claw,density:72,spread:1.97,rise:1.36,twist:0.74,tempo:0.76,phase:0.176,sweep:1.33}, // cracked sundial palm
    FxAuthor{id:168,name:"Stolen Breath",pattern:Pattern::Stasis,density:68,spread:2.14,rise:1.59,twist:1.1,tempo:0.87,phase:0.252,sweep:1.46}, // fractured stopped second ring
    FxAuthor{id:169,name:"Clockwork Ward",pattern:Pattern::Shield,density:64,spread:2.31,rise:0.9,twist:0.5,tempo:0.98,phase:0.328,sweep:0.55}, // gearwork shield
    FxAuthor{id:170,name:"Rewind Stitch",pattern:Pattern::Weave,density:76,spread:2.48,rise:1.13,twist:0.86,tempo:1.09,phase:0.404,sweep:0.68}, // backward flowing gold threads
    FxAuthor{id:181,name:"Bone Thorn",pattern:Pattern::Lance,density:76,spread:1.9,rise:0.9,twist:0.62,tempo:1.09,phase:0.1,sweep:1.07}, // twisted vertebral spear
    FxAuthor{id:182,name:"Soul Lantern",pattern:Pattern::Lantern,density:72,spread:2.07,rise:1.13,twist:0.98,tempo:1.2,phase:0.176,sweep:1.2}, // floating skull lantern
    FxAuthor{id:183,name:"Grave Whisper",pattern:Pattern::Wisp,density:68,spread:2.24,rise:1.36,twist:1.34,tempo:1.31,phase:0.252,sweep:1.33}, // hollow skull wind glyph
    FxAuthor{id:184,name:"Blood Thread",pattern:Pattern::Tether,density:64,spread:2.41,rise:1.59,twist:0.74,tempo:0.65,phase:0.328,sweep:1.46}, // braided crimson cord
    FxAuthor{id:185,name:"Rib Cage",pattern:Pattern::Cage,density:76,spread:2.58,rise:0.9,twist:1.1,tempo:0.76,phase:0.404,sweep:0.55}, // interlocking giant ribs
    FxAuthor{id:186,name:"Wraith Step",pattern:Pattern::Footsteps,density:72,spread:1.9,rise:1.13,twist:0.5,tempo:0.87,phase:0.138,sweep:0.68}, // ghost footstep trail
    FxAuthor{id:187,name:"Carrion Swarm",pattern:Pattern::Swarm,density:68,spread:2.07,rise:1.36,twist:0.86,tempo:0.98,phase:0.214,sweep:0.81}, // spirit moth swarm
    FxAuthor{id:188,name:"Marrow Shard",pattern:Pattern::Barrage,density:64,spread:2.24,rise:1.59,twist:1.22,tempo:1.09,phase:0.29,sweep:0.94}, // splintering bone dagger
    FxAuthor{id:189,name:"Epitaph Mark",pattern:Pattern::Sigil,density:76,spread:2.41,rise:0.9,twist:0.62,tempo:1.2,phase:0.366,sweep:1.07}, // carved gravestone glyph
    FxAuthor{id:190,name:"Crimson Pool",pattern:Pattern::Pool,density:72,spread:2.58,rise:1.13,twist:0.98,tempo:1.31,phase:0.1,sweep:1.2}, // red mirrored ritual pool
];
pub(super) fn author_for(slot:usize)->Option<&'static FxAuthor>{
    if slot>=200{return None;}
    let school=slot/20;
    let rank=slot%20;
    if rank>=10{return None;}
    let spec=&AUTHOR_100[school*10+rank];
    debug_assert_eq!(spec.id as usize,slot+1);
    Some(spec)
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn precisely_one_hundred_named_and_individually_tuned_author_presets(){
   use std::collections::BTreeSet;
   assert_eq!(AUTHOR_100.len(),100);
   let mut unique=BTreeSet::new();
   let mut families=BTreeSet::new();
   for (i,a) in AUTHOR_100.iter().enumerate(){
     assert_eq!(a.id as usize,1+(i/10)*20+i%10);
     assert!(!a.name.is_empty());
     assert!(unique.insert(a.name));
     families.insert(format!("{:?}",a.pattern));
     assert!((64..=76).contains(&a.density));
     assert!(a.spread.is_finite()&&a.rise.is_finite()&&a.twist.is_finite());
     assert_eq!(author_for((a.id-1) as usize).unwrap().name,a.name);
   }
   assert_eq!(unique.len(),100);
   assert!(families.len()>=28,"only {} distinct physical motion families",families.len());
   assert!(author_for(10).is_none());
   assert!(author_for(200).is_none());
 }
}
