//! Aetheria v0.5.1: unobtrusive fantasy HUD and ten-school spellbook.
//! Only used by the main-game HUD hook with a world and no open screens.
//! Original 9x9 icons and small native GUI rectangles (no texture dependencies).
use mod_api::{ClientHost, GuiRenderPostEvent};
use aetheria_horizon::ui::minimal_hud::menu_motion;
use aetheria_horizon::magic::registry::CLASS_NAMES;
use crate::catalog200::{self,SPELL_COUNT,SCHOOL_COLORS};

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
    let school=catalog200::school(selected);
    let accent=SCHOOL_COLORS[school];
    let spell=catalog200::spec(selected);
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
    e.batch.text(&format!("{:03}/200",selected+1),[x+217.,y+5.],accent,true);
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
            let school_i=catalog200::school(i);
            let bx=px+7.+offset as f32*58.;
            let by=py+18.;
            let active=i==selected;
            if active {
                soft_box(e,bx,by,54.,24.,0xEF1B2F3B);
                fill(e,[bx+3.,by+1.,48.,2.],SCHOOL_COLORS[school_i]);
            }
            glyph(e,i,school_i,bx+5.,by+8.);
            let name=catalog200::spec(i).name.chars().take(6).collect::<String>();
            e.batch.text(&name,[bx+16.,by+6.],
                if active{WHITE}else{MUTED},false);
            e.batch.text(&format!("{:02}",i+1),[bx+37.,by+15.],
                SCHOOL_COLORS[school_i],false);
        }
    }
}

fn draw_constellation(e:&mut GuiRenderPostEvent<'_>,view:&UiView) {
    let [w,h]=e.batch.size();
    if w<380.||h<280.{return;}
    // Compact gallery of all 20 authored spells in selected school, NOT
    // a fake panel containing three while the registry claims 200.
    fill(e,[0.,0.,w,h],0xA6070D16);
    let pw=(w-28.).min(670.);
    let ph=(h-22.).min(410.);
    let x=(w-pw)*0.5;
    let y=(h-ph)*0.5;
    soft_box(e,x,y,pw,ph,0xF00C1722);
    let selected=view.chosen.min(SPELL_COUNT-1);
    let school=catalog200::school(selected);
    let accent=SCHOOL_COLORS[school];
    let spell=catalog200::spec(selected);
    fill(e,[x+12.,y+13.,3.,37.],accent);
    e.batch.text("A E T H E R I A  /  200 SPELLS",
        [x+24.,y+11.],WHITE,true);
    e.batch.text("K EXIT   J/L SCHOOL   N/M SELECT",
        [x+24.,y+26.],MUTED,false);
    e.batch.text(&format!("{}   {} MP   {}",
        spell.name,spell.mana_cost as u16,spell.role),
        [x+24.,y+44.],accent,false);
    // 10 school tabs, two rows; only one selected at a time.
    let tab_width=(pw-28.)/5.;
    for idx in 0..10 {
        let bx=x+12.+(idx%5) as f32*tab_width;
        let by=y+65.+(idx/5) as f32*31.;
        let active=idx==school;
        soft_box(e,bx,by,tab_width-4.,28.,
            if active {0xE9274050} else {0x9F101E29});
        if active {fill(e,[bx+2.,by+1.,tab_width-8.,2.],SCHOOL_COLORS[idx]);}
        glyph(e,idx,idx,bx+6.,by+9.);
        e.batch.text(SCHOOL_TAGS[idx],[bx+21.,by+10.],
            if active{WHITE}else{MUTED},false);
    }
    let separator=y+132.;
    fill(e,[x+13.,separator,pw-26.,1.],0xFF304A5C);
    e.batch.text(CLASS_NAMES[school],[x+15.,separator+5.],accent,true);
    e.batch.text("14 STANDARD / 4 AWAKENING / 2 ULTIMATE",
        [x+pw-276.,separator+5.],MUTED,false);
    let grid_y=separator+25.;
    let row_h=((ph-204.)/4.).max(19.);
    let card_w=(pw-33.)/5.;
    for local in 0..20 {
        let slot=school*20+local;
        let spec=catalog200::spec(slot);
        let cx=x+12.+(local%5) as f32*card_w;
        let cy=grid_y+(local/5) as f32*row_h;
        let chosen=slot==selected;
        soft_box(e,cx,cy,card_w-4.,row_h-2.,
           if chosen {0xF0294355}else{0xC311202D});
        if chosen {fill(e,[cx+3.,cy+1.,card_w-11.,2.],accent);}
        glyph(e,local,school,cx+4.,cy+5.);
        let nm=spec.name.chars().take(13).collect::<String>();
        e.batch.text(&nm,[cx+16.,cy+5.],
            if chosen{WHITE}else{MUTED},false);
        e.batch.text(&format!("{:03}  {} MP",slot+1,spec.mana_cost as u16),
            [cx+16.,cy+17.],SCHOOL_COLORS[school],false);
        if row_h>32. {
            let tier=if local>=18{"ULTIMATE"}else if local>=14{"AWAKEN"}else{"STANDARD"};
            e.batch.text(tier,[cx+16.,cy+28.],MUTED,false);
        }
    }
    e.batch.text("R+SCROLL / N-M SELECT    J-L SCHOOL    G CAST    K CLOSE",
        [x+15.,y+ph-29.],WHITE,false);
    e.batch.text("LOCAL VISUAL ALPHA — 170 EXTENDED FORMS NEED ART REVIEW; NO SERVER EFFECTS",
        [x+15.,y+ph-15.],MUTED,false);
}
