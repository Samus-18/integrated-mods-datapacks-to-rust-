//! M29: fair submission of intact source geometry *components*.
//!
//! M28 kept the last source faces visible by selecting widely spaced quads.
//! That punched holes into closed sculptures: e.g. 3 out of 6 faces of a
//! boulder, or alternating sides of an orb. M29 introduces atomic component
//! ownership: only whole boxes/crystals/membranes/tubes/slabs/orbs/curtains
//! may be selected at once. A reduced-tessellation source orb/curtain is
//! constructed up front in crowded scenes instead of losing random panels.
//! Plain authored single quads remain individually selectable; future source
//! shapes must explicitly mark ALL connected panels as a component.
//! This is CPU world-quad scheduling, NOT GPU mesh LOD or verified Horizon FPS.
use super::Scene;
use std::{collections::VecDeque,ops::Range};

/// Return an ordered list of contiguous atomic component face ranges.
/// Group IDs are assigned sequentially by Scene::quad/begin_component.
fn spans(scene:&Scene)->Vec<Range<usize>> {
    debug_assert_eq!(scene.group_ids.len(),scene.facets.len());
    let mut result:Vec<Range<usize>>=Vec::new();
    for (i,id) in scene.group_ids.iter().enumerate(){
        if i==0 || *id!=scene.group_ids[i-1]{
            result.push(i..i+1);
        }else{
            result.last_mut().unwrap().end=i+1;
        }
    }
    result
}

/// Endpoint-first, then recursively bisect source indices. This ensures
/// that crowns, roots, wings and the LAST source elements remain eligible,
/// rather than consuming all capacity with the first authored components.
fn spread_order(n:usize)->Vec<usize>{
    if n==0{return Vec::new();}
    let mut result=vec![0];
    if n==1{return result;}
    result.push(n-1);
    let mut ranges=VecDeque::from([(0,n-1)]);
    while let Some((low,high))=ranges.pop_front(){
        if high<=low+1{continue;}
        let middle=low+(high-low)/2;
        result.push(middle);
        ranges.push_back((low,middle));
        ranges.push_back((middle,high));
    }
    debug_assert_eq!(result.len(),n);
    result
}

pub(super) fn sample_fair(scenes:&[Scene],budget:usize)
    ->Vec<([[f64;3];4],u32)>
{
    if scenes.is_empty() || budget==0{return Vec::new();}
    let groups=scenes.iter().map(spans).collect::<Vec<_>>();
    let order=groups.iter().map(|g|spread_order(g.len())).collect::<Vec<_>>();

    // First pass: equal FACE quotas, redistributing spare capacity from
    // genuinely tiny spells. No single new effect should monopolize draws.
    let mut quotas=vec![0usize;scenes.len()];
    let mut left=budget;
    loop{
        let mut progress=false;
        for (i,scene) in scenes.iter().enumerate(){
            if left==0{break;}
            if quotas[i]<scene.facets.len(){
                quotas[i]+=1;left-=1;progress=true;
            }
        }
        if left==0||!progress{break;}
    }

    // An atomic group is either selected completely or not at all.
    // Commit groups in source order later so render layering is stable.
    let mut selected=groups.iter().map(|g|vec![false;g.len()])
        .collect::<Vec<_>>();
    let mut used=vec![0usize;scenes.len()];
    let mut submitted=0usize;
    for i in 0..scenes.len(){
        for &j in &order[i]{
            let faces=groups[i][j].len();
            if faces<=quotas[i].saturating_sub(used[i]){
                selected[i][j]=true;
                used[i]+=faces;
                submitted+=faces;
            }
        }
    }

    // Whole components can leave a few unused faces in an initial quota.
    // Return that headroom to other spells in a deterministic round-robin
    // pass, without ever breaking a closed primitive to fill the last slot.
    let mut cursor=vec![0usize;scenes.len()];
    loop{
        let mut progress=false;
        for i in 0..scenes.len(){
            while cursor[i]<order[i].len(){
                let j=order[i][cursor[i]];
                cursor[i]+=1;
                if selected[i][j]{continue;}
                let faces=groups[i][j].len();
                if faces<=budget.saturating_sub(submitted){
                    selected[i][j]=true;
                    submitted+=faces;
                    progress=true;
                    break;
                }
            }
        }
        if !progress{break;}
    }

    let mut result=Vec::with_capacity(submitted);
    for (i,scene) in scenes.iter().enumerate(){
        for (j,group) in groups[i].iter().enumerate(){
            if selected[i][j]{
                result.extend_from_slice(&scene.facets[group.clone()]);
            }
        }
    }
    debug_assert_eq!(result.len(),submitted);
    debug_assert!(result.len()<=budget);
    result
}

#[cfg(test)]
mod tests{
    use super::*;
    fn filled(count:usize,school:u32)->Scene{
        let mut scene=Scene::new(count);
        for i in 0..count{
            let x=i as f64;
            scene.quad([[x,0.,0.],[x,1.,0.],[x,1.,1.],[x,0.,1.]],
                ((school&255)<<16)|(i as u32));
        }
        scene
    }
    fn components(parts:usize,faces:usize,school:u32)->Scene{
        let mut scene=Scene::new(parts*faces);
        for p in 0..parts{
            let prev=scene.begin_component(faces).unwrap();
            for k in 0..faces{
                let x=(p*faces+k) as f64;
                scene.quad([[x,0.,0.],[x,1.,0.],[x,1.,1.],[x,0.,1.]],
                    (school<<16)|p as u32);
            }
            scene.end_component(prev);
        }
        scene
    }

    #[test]
    fn shares_budget_with_spare_redistribution(){
        let scenes=[filled(2,1),filled(10,2),filled(10,3)];
        let result=sample_fair(&scenes,8);
        assert_eq!(result.len(),8);
        let counts=(1..=3).map(|s|result.iter()
            .filter(|(_,color)|(*color>>16)==s).count()).collect::<Vec<_>>();
        assert_eq!(counts,[2,3,3]);
        assert_eq!(result.iter().filter(|(_,c)|(*c>>16)==2)
            .map(|(_,c)|*c&65535).collect::<Vec<_>>(),[0,4,9]);
    }
    #[test]
    fn eight_large_spells_preserve_edges_and_never_exceed_900(){
        let scenes=(1..=8).map(|id|filled(500,id)).collect::<Vec<_>>();
        let result=sample_fair(&scenes,900);
        assert_eq!(result.len(),900);
        for school in 1..=8{
            let indices=result.iter().filter(|(_,c)|(*c>>16)==school)
                .map(|(_,c)|*c&65535).collect::<Vec<_>>();
            assert!((112..=113).contains(&indices.len()));
            assert_eq!(indices[0],0);
            assert_eq!(indices[indices.len()-1],499);
        }
        assert_eq!(result,sample_fair(&scenes,900));
    }
    #[test]
    fn atomic_components_are_never_stripped_or_cut_in_half(){
        let scenes=[components(9,6,1),components(9,6,2)];
        let result=sample_fair(&scenes,39);
        assert!(!result.is_empty());
        assert!(result.len()<=39);
        for school in 1..=2{
            for part in 0..9{
                let kept=result.iter().filter(|(_,c)|(*c>>16)==school
                    &&(*c&65535)==part).count();
                assert!(kept==0||kept==6,
                    "school {school} closed part {part} stripped to {kept}/6 faces");
            }
        }
        // Selecting only 39 of 108 source quads must not break a 6-face box.
        assert_eq!(result.len()%6,0);
        assert_eq!(result,sample_fair(&scenes,39));
    }
    #[test]
    fn eight_busy_spells_keep_whole_3d_components_and_both_ends(){
        let scenes=(1..=8).map(|id|components(50,10,id)).collect::<Vec<_>>();
        let result=sample_fair(&scenes,900);
        assert_eq!(result.len(),900);
        for school in 1..=8{
            let parts=(0..50).map(|p|result.iter().filter(|(_,c)|
                (*c>>16)==school&&(*c&65535)==p).count()).collect::<Vec<_>>();
            assert!(parts.iter().all(|&x|x==0||x==10));
            assert!(parts[0]>0&&parts[49]>0,
                "first/last authored sculpture components missing for {school}");
            let count=parts.iter().sum::<usize>();
            assert!((110..=120).contains(&count),
                "unfair scheduling: school {school} has {count} faces");
        }
    }
    #[test]
    fn oversized_atomic_form_is_hidden_instead_of_punched_through(){
        let scenes=[components(1,126,1),components(30,6,2)];
        let result=sample_fair(&scenes,50);
        assert!(result.iter().all(|(_,c)|(*c>>16)==2),
            "an orb larger than the total capacity must not become a wire mesh");
        assert_eq!(result.len()%6,0);
    }
    #[test]
    fn empty_and_tiny_frames_are_safe(){
        let empty=Scene::new(0);
        assert!(sample_fair(&[],900).is_empty());
        assert!(sample_fair(&[filled(5,1)],0).is_empty());
        assert!(sample_fair(&[empty],900).is_empty());
        let scenes=[filled(5,1),filled(2,2)];
        assert_eq!(sample_fair(&scenes,999).len(),7);
        assert_eq!(sample_fair(&scenes,1).len(),1);
        assert!(spread_order(0).is_empty());
        for n in 1..128{
            let order=spread_order(n);
            assert_eq!(order.len(),n);
            let mut sorted=order;sorted.sort_unstable();
            assert_eq!(sorted,(0..n).collect::<Vec<_>>());
        }
    }
}
