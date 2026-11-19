//! M27 catalog acceptance: every ID must dispatch through a deliberately
//! authored renderer. This is not a graphical quality or world gameplay test.
use crate::{
    catalog200, standard_forms, charged_standard_forms,
    standard_channelled, standard_instant,
    standard_combo, standard_charged2, standard_m26,
    standard_m27a, standard_m27b
};
#[test]
fn all_200_slots_have_explicit_non_template_artist_routing(){
    let mut seen=[false;200];
    let mut standard=0;
    let mut curated=0;
    let mut awaken=0;
    let mut ultimate=0;
    let mut count_m27=0;
    for slot in 0..200 {
        let id=catalog200::id(slot);
        let rank=catalog200::rank(slot);
        assert!(!seen[slot],"duplicate slot");
        seen[slot]=true;
        let routes=[
            standard_forms::dedicated(id),
            charged_standard_forms::dedicated(id),
            standard_channelled::dedicated(id),
            standard_instant::dedicated(id),
            standard_combo::dedicated(id),
            standard_charged2::dedicated(id),
            standard_m26::dedicated(id),
            standard_m27a::dedicated(id),
            standard_m27b::dedicated(id)
        ];
        let num=routes.iter().filter(|&&x|x).count();
        match rank {
            0..=2 => {
                curated+=1;
                assert!(catalog200::curated_slot(slot).is_some());
                assert_eq!(num,0,"unexpected standard override for curated id={id}");
            },
            3..=13 => {
                standard+=1;
                assert_eq!(num,1,
                    "standard spell {id} must have exactly one bespoke renderer");
                assert!(!catalog200::spec(slot).mesh.is_empty());
                if (11..=13).contains(&rank){
                    count_m27+=1;
                    assert!(routes[7] ^ routes[8],
                       "M27 art scene missing or duplicated for id={id}");
                }
            },
            14..=17 => {
                awaken+=1;
                assert_eq!(num,0,
                    "Awakening class should use bespoke awakening scene {id}");
            },
            18..=19 =>{
                ultimate+=1;
                assert_eq!(num,0,
                    "Ultimate should use uniquely authored hero scene {id}");
            },
            _=> unreachable!()
        }
    }
    assert_eq!((curated,standard,awaken,ultimate),(30,110,40,20));
    assert_eq!(count_m27,30);
    assert_eq!(seen.iter().filter(|&&x|x).count(),200);
}
