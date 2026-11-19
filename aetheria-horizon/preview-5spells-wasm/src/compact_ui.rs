//! Aetheria v0.5.1: unobtrusive fantasy HUD and ten-school spellbook.
//! Only used by the main-game HUD hook with a world and no open screens.
//! Original 9x9 icons and small native GUI rectangles (no texture dependencies).
use mod_api::{ClientHost, GuiRenderPostEvent};
use aetheria_horizon::ui::minimal_hud::menu_motion;
use aetheria_horizon::magic::registry::CLASS_NAMES;
use crate::catalog30::{self,SPELL_COUNT,SCHOOL_COLORS};

const WHITE:u32=0xFFE6F2F6;
const MUTED:u32=0xFF94A7B5;
const DARK:u32=0xB808111C;
const DARKER:u32=0xDB0B1722;
const BLUE:u32=0xFF6DAFC9;
const SCHOOL_TAGS:[&str;10]=["EMBER","FROST","GALE","STONE","TIDE","WILD","STORM","ASTRAL","TIME","SOUL"];

pub struct UiView {
    pub tick:u64,
    pub chosen:usize,
    pub quick_open:bool,
    pub constellation:bool,
    pub mana:f32,
    pub cooldowns:[u64;SPELL_COUNT],
    pub cast_variant:u8,
}
fn fill(e:&mut GuiRenderPostEvent<'_>,rect:[f32;4],color:u32) {
    if rect[2]>0.0 && rect[3]>0.0 {e.batch.fill(rect,color);}
}
fn soft_box(e:&mut GuiRenderPostEvent<'_>,x:f32,y:f32,w:f32,h:f32,color:u32){
    if w<7.||h<7.{return;}
    fill(e,[x+3.,y,w-6.,h],color);
    fill(e,[x+1.,y+1.,w-2.,h-2.],color);
    fill(e,[x,y+3.,w,h-6.],color);
}
// Hand-drawn 9×9 pixel glyphs. Every one of the 20 local spell slots gets
// its own distinct pattern; these are not colored circles/placeholder quads.
// Drawn by the game itself, no AI-generated sprites or copied assets.
const GLYPHS:[&str;20]=[
    "....#..../...#.#.../..#...#../.#.....#./#########/.#.....#./..#...#../...#.#.../....#....",
    "......#../.....##../....###../...####../..#####../...#...../..#....../.#......./#........",
    ".#######./.#.....#./.#.#.#.#./.#.###.#./.##...##./.#.###.#./.#.#.#.#./.#.....#./.#######.",
    "...###.../..#...#../.#.....#./#...#...#/....#..../...###.../..#...#../.#.....#./#.......#",
    "#.......#/##.....##/.##...##./..##.##../...###.../...###.../..##.##../.##...##./##.....##",
    "...###.../..#####../.#######./##.....##/##..#..##/##..#..##/.#######./..#####../...###...",
    "..##.##../.###.###./..#####../...###.../....#..../...###.../..#####../.##...##./##.....##",
    "..#####../.##...##./##.#.#.##/##.....##/.##.#.##./..##.##../...###.../...###.../....#....",
    "#########/#.......#/#.#####.#/#.#...#.#/#.#.#.#.#/#.#...#.#/#.#####.#/#.......#/#########",
    "....#..../...###.../..#####../.#######./#########/...###.../...###.../..#...#../.#.....#.",
    "###...###/.###.###./..#####../...###.../....#..../...###.../..#####../.###.###./###...###",
    ".##...##./..##.##../...###.../....#..../#########/....#..../...###.../..##.##../.##...##.",
    "....#..../...###.../..#####../.###.###./##..#..##/.###.###./..#####../...###.../....#....",
    ".#.....#./.##...##./..##.##../...###.../...###.../..##.##../.##...##./.#.....#./#.......#",
    "...###.../..#...#../.#.###.#./#.#...#.#/#.#.#.#.#/#.#...#.#/.#.###.#./..#...#../...###...",
    ".##...##./.##...##./.##...##./..##.##../...###.../..##.##../.##...##./.##...##./.##...##.",
    "...###.../..#####../.##.#.##./##..#..##/.##.#.##./.#######./..#...#../.#.....#./#.......#",
    "##.....##/.##...##./..##.##../...###.../....#..../...###.../..##.##../.##...##./##.....##",
    "#########/#..###..#/#.#...#.#/##.#.#.##/##..#..##/##.#.#.##/#.#...#.#/#..###..#/#########",
    "...###.../..#####../.##...##./###...###/####.####/###...###/.##...##./..#####../...###...",
];
fn glyph(e:&mut GuiRenderPostEvent<'_>,local:usize,class:usize,x:f32,y:f32){
    let art=GLYPHS[local%20];
    let primary=SCHOOL_COLORS[class.min(9)];
    for (row,line) in art.split('/').enumerate(){
        // Coalesce consecutive on-pixels into one GPU GUI rectangle.
        // This bounds the 20-node menu's shape draw cost.
        let mut start=None;
        for (col,ch) in line.bytes().chain(std::iter::once(b'.')).enumerate(){
            if ch==b'#' {
                if start.is_none(){start=Some(col);}
            } else if let Some(begin)=start.take(){
                fill(e,[x+begin as f32,y+row as f32,(col-begin) as f32,1.],primary);
            }
        }
    }
}


pub fn draw(host:&dyn ClientHost,e:&mut GuiRenderPostEvent<'_>,view:UiView) {
    // Belt-and-braces protection: GUI events fire on title/options screens!
    if !host.playing() || host.local_player().is_none()
        || host.observed_screen_id().is_some()
        || e.gui_hidden || !e.focused {return;}
    let [w,h]=e.batch.size();
    if w<300.||h<180.{return;}
    if view.constellation {
        draw_constellation(e,&view);
        return;
    }
    let selected=view.chosen.min(SPELL_COUNT-1);
    let school=catalog30::school(selected);
    let accent=SCHOOL_COLORS[school];
    let spell=catalog30::spec(selected);
    let mana=if view.mana.is_finite(){view.mana.clamp(0.,100.)}else{0.};
    let cd=view.cooldowns[selected].saturating_sub(view.tick);
    // Tiny (262x34 GUI px) piece of enchanted glass above the vanilla hotbar.
    // No gold border, large permanently open wheel or duplicated health bars.
    let x=(w-262.)*0.5;
    let y=(h-82.).max(49.);
    soft_box(e,x,y,262.,33.,DARK);
    fill(e,[x+1.,y+6.,2.,20.],accent);
    glyph(e,selected,school,x+10.,y+8.);
    e.batch.text(spell.name,[x+27.,y+5.],WHITE,true);
    e.batch.text(&format!("{:02}/30",selected+1),[x+217.,y+5.],accent,true);
    let cd_label=if cd==0 {"READY".to_owned()} else {
        format!("{:.1}s",cd as f32/20.0)
    };
    e.batch.text(&cd_label,[x+27.,y+16.],if cd==0{MUTED}else{accent},false);
    e.batch.text(&format!("{:03} MP",mana.round() as u16),
        [x+214.,y+16.],MUTED,false);
    fill(e,[x+78.,y+23.,129.,3.],0xFF203442);
    let mw=129.*mana/100.;
    fill(e,[x+78.,y+23.,mw,3.],BLUE);
    if mw>4. {fill(e,[x+78.,y+23.,mw,1.],0xFFB1DDE8);}

    // Compact five-neighbour picker visible only while holding R or briefly
    // after switching; no permanent block across the middle of the view.
    if view.quick_open {
        let px=(w-300.)*0.5;
        let py=(y-54.).max(8.);
        soft_box(e,px,py,300.,48.,DARKER);
        e.batch.text("R + SCROLL   /   N-M    |    G CAST",
            [px+11.,py+5.],MUTED,false);
        for offset in 0..5 {
            let i=(selected+SPELL_COUNT+offset-2)%SPELL_COUNT;
            let school_i=catalog30::school(i);
            let bx=px+7.+offset as f32*58.;
            let by=py+18.;
            let active=i==selected;
            if active {
                soft_box(e,bx,by,54.,24.,0xEF1B2F3B);
                fill(e,[bx+3.,by+1.,48.,2.],SCHOOL_COLORS[school_i]);
            }
            glyph(e,i,school_i,bx+5.,by+8.);
            let name=catalog30::spec(i).name.chars().take(6).collect::<String>();
            e.batch.text(&name,[bx+16.,by+6.],
                if active{WHITE}else{MUTED},false);
            e.batch.text(&format!("{:02}",i+1),[bx+37.,by+15.],
                SCHOOL_COLORS[school_i],false);
        }
    }
}

fn draw_constellation(e:&mut GuiRenderPostEvent<'_>,view:&UiView) {
    let [w,h]=e.batch.size();
    if w<370.||h<270.{return;}
    // Translucent readable book instead of full-screen opaque blue table.
    fill(e,[0.,0.,w,h],0xB7070C15);
    let pw=(w-30.).min(550.);
    let ph=(h-24.).min(330.);
    let x=(w-pw)*0.5;
    let y=(h-ph)*0.5;
    soft_box(e,x,y,pw,ph,0xF10B1420);
    let selected=view.chosen.min(SPELL_COUNT-1);
    let school=catalog30::school(selected);
    let accent=SCHOOL_COLORS[school];
    fill(e,[x+12.,y+14.,3.,32.],accent);
    e.batch.text("A E T H E R I A",[x+24.,y+11.],WHITE,true);
    e.batch.text("SPELLBOOK  /  30 PREVIEW SPELLS",[x+24.,y+25.],MUTED,false);
    e.batch.text("K CLOSE  /  N-M SELECT",[x+24.,y+40.],MUTED,false);
    // A restrained slow star drift behind the ten class choices.
    let motion=menu_motion(view.tick,0.0,school as u16,false);
    for star in 0..24u32 {
        let px=x+16.+(star.wrapping_mul(89)%503) as f32/503.*(pw-34.);
        let py=y+58.+(star.wrapping_mul(193)%221) as f32/221.*(ph-78.);
        fill(e,[px+motion.star_x*(star%3) as f32,py+motion.star_y,1.,1.],
            0xFF30495B);
    }
    // Every class has its own visible tab; only three demo spells per class.
    let tab_width=(pw-31.)/5.;
    for school_i in 0..10 {
        let cx=x+13.+(school_i%5) as f32*tab_width;
        let cy=y+64.+(school_i/5) as f32*36.;
        let active=school_i==school;
        soft_box(e,cx,cy,tab_width-4.,30.,
            if active{0xEC233947}else{0x9E111F2A});
        if active {
            fill(e,[cx+3.,cy+1.,tab_width-10.,2.],SCHOOL_COLORS[school_i]);
        }
        glyph(e,school_i,school_i,cx+7.,cy+9.);
        e.batch.text(SCHOOL_TAGS[school_i],[cx+23.,cy+10.],
            if active{WHITE}else{MUTED},false);
    }
    let heading_y=y+145.;
    fill(e,[x+17.,heading_y,pw-34.,1.],0xFF304A5C);
    e.batch.text(CLASS_NAMES[school],[x+18.,heading_y+10.],accent,true);
    e.batch.text("3 TEST SPELLS",[x+pw-128.,heading_y+10.],MUTED,false);
    let card_width=(pw-40.)/3.;
    for rank in 0..3 {
        let i=school*3+rank;
        let bx=x+15.+rank as f32*(card_width+5.);
        let by=heading_y+32.;
        let active=i==selected;
        soft_box(e,bx,by,card_width,83.,
            if active{0xEF1D3140}else{0xCE111F2A});
        fill(e,[bx+7.,by+8.,2.,16.],
            if active{accent}else{0xFF476075});
        glyph(e,i,school,bx+15.,by+11.);
        e.batch.text(&format!("{:02}",i+1),[bx+card_width-28.,by+11.],accent,true);
        let spec=catalog30::spec(i);
        e.batch.text(spec.name,[bx+9.,by+33.],if active{WHITE}else{MUTED},true);
        e.batch.text(&format!("{} MP",spec.mana_cost as u16),
            [bx+9.,by+54.],MUTED,false);
        let cd=view.cooldowns[i].saturating_sub(view.tick);
        let status=if cd==0{"READY".to_owned()}
            else{format!("{:.1}s",cd as f32/20.)};
        e.batch.text(&status,[bx+card_width-51.,by+67.],
            if active{accent}else{MUTED},false);
    }
    e.batch.text("R + WHEEL TO CHOOSE     G TO CAST     K TO EXIT",
        [x+18.,y+ph-31.],WHITE,false);
    e.batch.text("CLIENT VISUAL TEST  /  NO WORLD DAMAGE OR SAVE SYNC",
        [x+18.,y+ph-18.],MUTED,false);
}
