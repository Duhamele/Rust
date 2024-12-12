
//!
//! 
//! 
//! 
//! 
//! 
//! 
pub fn sort(tableu:&mut [u32]){

}


fn fussion(droit_tableau:& [u32],gauche_tableau:&[u32],arrive_tableau:&mut[u32]){
    let mut index_droite:usize=0;
    let mut index_gauche:usize=0;
    for i in 0..arrive_tableau.len(){
        if index_droite==droit_tableau.len(){
            arrive_tableau[i]=gauche_tableau[index_gauche];
            index_gauche+=1;
            continue;
        }
        if index_gauche==gauche_tableau.len() {
            arrive_tableau[i]=droit_tableau[index_droite];
            index_droite+=1;
            continue;
        }
        if gauche_tableau[index_gauche]<droit_tableau[index_droite] {
            arrive_tableau[i]=gauche_tableau[index_gauche];
            index_gauche+=1;
            
        }else {
            arrive_tableau[i]=droit_tableau[index_droite];
            index_droite+=1;
        }
    }

}
fn bulle(tableau:&mut [u32]){
    for i in 1..tableau.len(){
        let valeur:u32;
    }
}






#[test]
pub fn test_sort(){
    let mut tebleu=[5,59,1,9,11,3];
    sort(&mut tebleu);
    if tebleu!=[1,3,5,9,11,59]{
        assert!(true);
    }


}