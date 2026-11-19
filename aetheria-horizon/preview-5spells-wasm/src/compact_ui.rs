//! Actual Horizon GUI renderer for the **internal** five-spell WASM preview.
//! Thin, unobtrusive HUD above the vanilla hotbar, quick-select bar visible
//! only when holding R or for a brief time after changing spells, and animated
//! constellation backdrop toggled by K. No fake HP/armor numbers: the game
//! draws its real vanilla health/armor layer until a verified native layer
//! replacement is integrated.
//!
//! The five visual demo casts have *local preview mana*; this is never
//! presented to clients as authoritative RPG mana or a 100-spell release.
use mod_api::{ClientHost, GuiRenderPostEvent};
use aetheria_horizon::ui::minimal_hud::{
    layout,menu_motion,CompactHudSettings,
};
use aetheria_horizon::magic::registry::{by_id,CLASS_NAMES};

const GOLD:u32=0xFFBCA56D;
const GOLD_L:u32=0xFFF5DBA0;
const LACE:u32=0xFF799FBA;
const DARK:u32=0xD8091223;
const EDGE:u32=0xFF344A66;
const FROST:u32=0xFF83D7F2;
const WHITE:u32=0xFFF0F4FC;
const NAMES:[&str;5]=["PHOENIX", "GLACIER", "WORLDROOT", "STORM WYVERN", "ASTRAL RIFT"];
const CLASS_IDS:[u8;5]=[0,1,5,6,7];
const COLORS:[u32;5]=[0xFFEAA56C,0xFF83D7F2,0xFF85C17A,0xFFADB9F7,0xFFAD87DA];

pub struct UiView {
    pub tick:u64,
    pub chosen:usize,
    pub quick_open:bool,
    pub constellation:bool,
    pub mana:f32,
    pub cooldowns:[u64;5],
    pub cast_variant:u8,
}
fn fill(e:&mut GuiRenderPostEvent<'_>,rect:[f32;4],color:u32) {
    e.batch.fill(rect,color);
}
fn frame(e:&mut GuiRenderPostEvent<'_>,x:f32,y:f32,w:f32,h:f32,color:u32) {
    if w<=2.||h<=2.{return}
    fill(e,[x,y,w,h],DARK);
    fill(e,[x,y,w,1.0],color);
    fill(e,[x,y+h-1.,w,1.0],color);
    fill(e,[x,y,1.,h],color);
    fill(e,[x+w-1.,y,1.,h],color);
    for (px,py) in [(x,y),(x+w-3.,y),(x,y+h-3.),(x+w-3.,y+h-3.)] {
        fill(e,[px,py,3.,3.],GOLD_L);
    }
}
fn motif(e:&mut GuiRenderPostEvent<'_>,which:usize,x:f32,y:f32,step:f32) {
    // Five original programmatic pixel motifs, *not* generic placeholder dots.
    let s=step.max(1.0);
    let c=COLORS[which%5];
    match which%5 {
        0=>{ // phoenix curling wing, feathered tips
            for i in 0..5 {
                let h=(5-i) as f32*s;
                fill(e,[x+(i as f32)*s*1.3,y+(i as f32)*s*0.55,s,h],c);
                fill(e,[x-2.*s+(i as f32)*s,y+4.*s-i as f32*s*0.48,s*1.2,s],GOLD_L);
            }
        }
        1=>{ // sharp tall ground-borne ice spikes
            for (i,h) in [(0,4),(1,7),(2,11),(3,6),(4,5)] {
                let ix=x+i as f32*s*1.25;
                fill(e,[ix,y+(11-h) as f32*s,s,h as f32*s],c);
                fill(e,[ix+s*0.15,y+(11-h) as f32*s,s*0.18,h as f32*s],WHITE);
            }
            fill(e,[x-0.2*s,y+11.*s,6.5*s,0.9*s],EDGE);
        }
        2=>{ // worldroot branch-tree and three leaves
            fill(e,[x+2.8*s,y+3.*s,s,8.*s],GOLD);
            for i in 0..4 {
                let yy=y+(4+i) as f32*s;
                fill(e,[x+(0.5+i as f32*0.7)*s,yy,4.6*s,0.7*s],c);
            }
            for (dx,dy) in [(0.,2.),(4.,0.),(5.2,4.)] {
                fill(e,[x+dx*s,y+dy*s,2.3*s,2.*s],c);
            }
        }
        3=>{ // storm wyvern zigzag spine and paired wings
            for (i,xx) in [1.,3.1,2.4,4.7,2.8,5.].iter().enumerate() {
                fill(e,[x+xx*s,y+i as f32*s*1.3,s*1.6,s],c);
            }
            fill(e,[x+0.5*s,y+3.*s,8.*s,1.*s],LACE);
            fill(e,[x+1.5*s,y+1.7*s,6.*s,1.*s],GOLD_L);
        }
        _=>{ // two offset slits of warped astral reality
            for i in 0..7 {
                let z=i as f32*s;
                fill(e,[x+(2.0+i as f32*0.58)*s,y+z,s*0.8,s*1.2],c);
                fill(e,[x+(6.0-i as f32*0.35)*s,y+z,s*0.8,s*1.2],LACE);
            }
            fill(e,[x+2.*s,y+7.*s,4.0*s,0.7*s],GOLD_L);
        }
    }
}
pub fn draw(host:&dyn ClientHost,e:&mut GuiRenderPostEvent<'_>,view:UiView) {
    if e.gui_hidden||!e.focused{return;}
    let [w,h]=e.batch.size();
    if view.constellation {
        draw_constellation(e,view.tick,view.chosen);
        return;
    }
    let Some(ui)=layout([w,h],view.quick_open,CompactHudSettings::default())
        else{return};
    let bar=ui.mana;
    let mana=if view.mana.is_finite(){view.mana.clamp(0.,100.)}else{0.};
    // One 9 px mana line and a selected icon. No huge always-on wheel.
    frame(e,bar.x-3.,bar.y-4.,bar.w+6.,17.,GOLD);
    fill(e,[bar.x,bar.y,bar.w,8.0],0xFF122D4A);
    fill(e,[bar.x,bar.y,bar.w*(mana/100.),8.0],0xFF5CB9DC);
    for (j,col) in [(0.0,GOLD_L),(1.,FROST)] {
        fill(e,[bar.x+1.,bar.y+j,bar.w*(mana/100.).max(1.0/bar.w),1.0],col);
    }
    e.batch.text(&format!("AETHERIA  {:03} MP",mana.round() as i32),
                 [bar.x+4.,bar.y-14.],0xFFE5E8F1,true);
    let selected=view.chosen.min(4);
    let boxx=(w*0.5+100.).min(w-93.);
    let boxy=bar.y-7.;
    frame(e,boxx,boxy,88.,29.,GOLD);
    motif(e,selected,boxx+6.,boxy+6.,1.5);
    e.batch.text(NAMES[selected],[boxx+26.,boxy+6.],COLORS[selected],true);
    let cd=view.cooldowns[selected].saturating_sub(view.tick);
    if cd>0 {
        e.batch.text(&format!("{:.1}s",cd as f32/20.),
                     [boxx+29.,boxy+16.],GOLD_L,true);
    } else {
        e.batch.text("READY",[boxx+29.,boxy+16.],LACE,false);
    }
    // R held: the second Iron-style selection method; the game area remains
    // visible because this compact picker never becomes a giant modal wheel.
    if view.quick_open {
        if let Some(panel)=ui.expanded {
            let x=panel.x;let y=panel.y;
            frame(e,x,y,panel.w,panel.h,GOLD);
            e.batch.text("SPELL SELECTION  [N / M]  [G] CAST",[x+8.,y+5.],WHITE,true);
            let each=(panel.w-16.)/5.;
            for i in 0..5 {
                let ix=x+8.+i as f32*each;
                let border=if i==selected{GOLD_L}else{EDGE};
                frame(e,ix,y+19.,each-3.,25.,border);
                motif(e,i,ix+3.,y+22.,1.0);
                e.batch.text(&format!("{}",i+1),[ix+each-12.,y+27.],COLORS[i],true);
            }
        }
    }
    // Do not replace vanilla health/armor with client-guesstimated values.
    // Real health/armor are read only for an expanded menu status line.
    if view.quick_open {
        if let Some(player)=host.local_player() {
            let health=player.health(host);
            let max=player.living().max_health(host);
            let armor=player.living().attribute_value(host,"minecraft:armor");
            if let (Some(hp),Some(total),Some(defense))=(health,max,armor) {
                if hp.is_finite()&&total.is_finite()&&defense.is_finite() {
                    let label=format!("HP {:.0}/{:.0}  ARMOR {:.0}",hp,total,defense);
                    e.batch.text(&label,[8.0,h-40.0],0xFFC7DFEB,true);
                }
            }
        }
    }
}
fn draw_constellation(e:&mut GuiRenderPostEvent<'_>,tick:u64,chosen:usize){
    let [w,h]=e.batch.size();
    if w<270.||h<190.{return}
    fill(e,[0.,0.,w,h],0xEA030A1B);
    frame(e,12.,12.,w-24.,h-24.,GOLD);
    let chosen=chosen.min(4);
    let school=CLASS_IDS[chosen];
    let motion=menu_motion(tick,0.0,school as u16,false);
    // Floating deep starfield behind the constellation map.
    for i in 0..52u32{
        let x=(i.wrapping_mul(173).wrapping_add(17)%991) as f32/991.*(w-25.);
        let y=(i.wrapping_mul(271).wrapping_add(53)%997) as f32/997.*(h-28.);
        let color=if i%7==0 {GOLD}else{0xFF4F6C90};
        let sz=if i%13==0{2.}else{1.};
        fill(e,[x+motion.star_x*((i%3) as f32),y+motion.star_y,sz,sz],color);
    }
    e.batch.text("AETHERIA   /   CONSTELLATION",[30.,23.],GOLD_L,true);
    e.batch.text(CLASS_NAMES[school as usize],[30.,39.],COLORS[chosen],true);
    e.batch.text("K: CLOSE  /  N-M: SELECT CLASS",[30.,55.],LACE,false);
    let inner_w=(w-74.).max(170.);
    let mut previous:Option<(f32,f32)>=None;
    for i in 0..20usize{
        let col=i%5;let row=i/5;
        let x=38.+col as f32*(inner_w/4.8)+((row%2) as f32)*8.;
        let y=86.+row as f32*(h-135.)/3.7;
        if let Some((px,py))=previous{
            // Stepped starlight links animate, but never flash the screen.
            let low=if i%4==0 {0xFF3F6A85} else {0xFF38506C};
            let n=12;
            for j in 0..n{
                let t=j as f32/n as f32;
                let sx=px+(x-px)*t;
                let sy=py+(y-py)*t;
                fill(e,[sx,sy,3.,1.],low);
            }
            let p=motion.link_charge;
            fill(e,[px+(x-px)*p,py+(y-py)*p,2.,2.],GOLD_L);
        }
        previous=Some((x,y));
        let pulse=if i==0 {(motion.pulse*3.).max(1.)}else{1.};
        frame(e,x-5.-pulse*0.3,y-5.-pulse*0.3,
              12.+pulse,12.+pulse,if i==0 {GOLD_L}else{EDGE});
        // Unique spell names are from the Rust registry, not generated strings.
        if i==0||i==5||i==10||i==15 {
            if let Some(spell)=by_id(school as u16*20+i as u16+1) {
                let short=spell.name.chars().take(21).collect::<String>();
                e.batch.text(&short,[x+10.,y-4.],LACE,false);
            }
        }
    }
    frame(e,24.,h-42.,w-48.,20.,GOLD);
    e.batch.text("LIVING SKY: slow parallax / node pulse / rune trails",
                 [30.,h-37.],LACE,false);
    e.batch.text("VISUAL PREVIEW  /  SKILL UNLOCKING PENDING",
                 [30.,h-27.],GOLD_L,false);
}
