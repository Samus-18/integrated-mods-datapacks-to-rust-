//! M28: fair, deterministic submission of whole authored spell sculptures.
//! The 500-face source draft retains every sculpted component before
//! downsampling to the actual Horizon world-quad budget. In particular,
//! the last petal/wall/impact no longer disappears simply because earlier
//! parts of the same spell consumed the frame's per-spell allocation.
//! CPU geometry only; no GPU instancing, lighting or world gameplay implied.
use super::Scene;

pub(super) fn sample_fair(scenes:&[Scene],budget:usize)
    -> Vec<([[f64;3];4],u32)>
{
    if scenes.is_empty() || budget==0 {return Vec::new();}
    let mut quotas=vec![0usize;scenes.len()];
    let mut left=budget;
    // Water-filling: spells with small forms return spare faces to complex
    // spells, while any two unfinished spells differ by at most one quad.
    loop {
        let mut allocated=false;
        for (i,scene) in scenes.iter().enumerate() {
            if left==0 {break;}
            if quotas[i]<scene.facets.len() {
                quotas[i]+=1;
                left-=1;
                allocated=true;
            }
        }
        if left==0 || !allocated {break;}
    }
    let count=budget-left;
    let mut result=Vec::with_capacity(count);
    for (scene,quota) in scenes.iter().zip(quotas) {
        let len=scene.facets.len();
        if quota==0 {continue;}
        if quota==len {
            result.extend_from_slice(&scene.facets);
        } else if quota==1 {
            result.push(scene.facets[len/2]);
        } else {
            // Uniform spatial-source sampling, including the final authored
            // part. Taking only the first 'quota' faces deletes late petals
            // and foundations in high-complexity Awakening/Ultimate scenes.
            for k in 0..quota {
                let index=k*(len-1)/(quota-1);
                result.push(scene.facets[index]);
            }
        }
    }
    debug_assert!(result.len()<=budget);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn filled(count:usize,school:u32)->Scene {
        let mut scene=Scene::new(count);
        for i in 0..count{
            let x=i as f64;
            scene.quad([[x,0.,0.],[x,1.,0.],[x,1.,1.],[x,0.,1.]],
                ((school&255)<<16)|(i as u32));
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
    fn empty_and_tiny_frames_are_safe(){
        let empty=Scene::new(0);
        assert!(sample_fair(&[],900).is_empty());
        assert!(sample_fair(&[filled(5,1)],0).is_empty());
        assert!(sample_fair(&[empty],900).is_empty());
        let scenes=[filled(5,1),filled(2,2)];
        assert_eq!(sample_fair(&scenes,999).len(),7);
        assert_eq!(sample_fair(&scenes,1).len(),1);
    }
}
