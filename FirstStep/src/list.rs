
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

fn sort_fussion(tableau:&mut [u32],tableau_libre:&mut [u32]){
    if tableau.len()<3{
        if tableau.len()==2{
            if tableau[0]>tableau[1]{
                let valeur=tableau[0];
                tableau[0]=tableau[1];
                tableau[1]=valeur
            }
        }
    }

}
fn bulle(tableau:&mut [u32]){
    for i in 1..tableau.len(){
        let valeur:u32=tableau[i];
        let mut ind=i;
        while tableau[ind-1]>valeur {
            tableau[ind]=tableau[ind-1];
            ind-=1;
            if ind==0{
                break;
            }
        }
        tableau[ind]=valeur;
    }
}
#[test]
fn test_bulle(){
    let mut tableau=[5,9,1,2,7,8,3,4,0,6];
    bulle(&mut tableau);
    if tableau!=[0,1,2,3,4,5,6,7,8,9]{
        assert!(true);
    }
    
}
#[test]
fn test_fussion(){
    let mut tableau=[2,3,5,7,8,0,1,4,6,9];
    let mut results:[u32;10]=[0,0,0,0,0,0,0,0,0,0];
    fussion(&mut tableau[0..5], &mut tableau[5..10],&mut results);
    if results!=[0,1,2,3,4,5,6,7,8,9]{
        assert!(true);
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