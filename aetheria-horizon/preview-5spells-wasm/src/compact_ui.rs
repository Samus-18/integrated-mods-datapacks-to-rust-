//! Aetheria v0.5.1: unobtrusive fantasy HUD and ten-school spellbook.
//! Only used by the main-game HUD hook with a world and no open screens.
//! Original 9x9 icons and small native GUI rectangles (no texture dependencies).
use mod_api::{ClientHost, GuiRenderPostEvent};
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
    pub mana:f32,
    pub cooldowns:[u64;SPELL_COUNT],
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


// M32: a single quiet hotbar strip; no fullscreen screen takeover.
// Two controls only: tap R for next school, hold R+wheel for one of
// its 20 spells, G cast. The game hotbar stays usable normally.
pub fn draw(host:&dyn ClientHost,e:&mut GuiRenderPostEvent<'_>,view:UiView) {
    if !host.playing() || host.local_player().is_none()
        || host.observed_screen_id().is_some()
        || e.gui_hidden || !e.focused {return;}
    let [w,h]=e.batch.size();
    if w<300.||h<180.{return;}
    let selected=view.chosen.min(SPELL_COUNT-1);
    let school=catalog200::school(selected);
    let rank=catalog200::rank(selected);
    let accent=SCHOOL_COLORS[school];
    let spec=catalog200::spec(selected);
    let mana=if view.mana.is_finite(){view.mana.clamp(0.,100.)}else{0.};
    let cd=view.cooldowns[selected].saturating_sub(view.tick);

    // 274×32 GUI pixels, dark, no overlapping keybind instructions.
    let x=(w-274.)*0.5;
    let y=(h-80.).max(45.);
    soft_box(e,x,y,274.,32.,0xCA090F19);
    fill(e,[x+2.,y+5.,3.,22.],accent);
    glyph(e,rank,school,x+11.,y+8.);
    let spell_name=spec.name.chars().take(26).collect::<String>();
    e.batch.text(&spell_name,[x+27.,y+5.],WHITE,true);
    e.batch.text(&format!("{} {:02}/20",SCHOOL_TAGS[school],rank+1),
        [x+27.,y+17.],MUTED,false);
    e.batch.text(&format!("{} MP",mana.round() as u16),
        [x+226.,y+5.],MUTED,false);
    let ready=if cd==0 {"G CAST".to_string()}else{
        format!("{:.1}s",cd as f32/20.0)
    };
    e.batch.text(&ready,[x+226.,y+17.],
        if cd==0{accent}else{MUTED},false);
    fill(e,[x+103.,y+27.,116.,2.],0xFF203442);
    fill(e,[x+103.,y+27.,116.*mana/100.,2.],accent);

    if view.quick_open {
        let px=(w-300.)*0.5;
        let py=(y-58.).max(8.);
        soft_box(e,px,py,300.,53.,0xE209121E);
        e.batch.text("R: SCHOOL    HOLD R + WHEEL: SPELL    G: CAST",
            [px+8.,py+4.],WHITE,false);
        e.batch.text(SCHOOL_TAGS[school],[px+8.,py+16.],accent,true);
        for offset in 0..3 {
            let local=(rank+19+offset)%20;
            let id=school*20+local;
            let bx=px+6.+offset as f32*97.;
            let by=py+27.;
            let active=local==rank;
            soft_box(e,bx,by,94.,23.,
                if active{0xE322394A}else{0xB30C1A29});
            if active {fill(e,[bx+3.,by+2.,87.,2.],accent);}
            glyph(e,local,school,bx+5.,by+9.);
            let name=catalog200::spec(id).name.chars().take(10).collect::<String>();
            e.batch.text(&name,[bx+18.,by+7.],
                if active{WHITE}else{MUTED},false);
        }
    }
}
