fn main() {
    let rayon=5.2;
    let perimetre=perimetre_cercle(rayon);
    let surface=surface_cercle(rayon);
    let surface_s:f64=surface_sphere(rayon);
    let volume:f64=volume_sphere(rayon);
    println!("Pour un cercle de rayon {} cm",rayon);
    println!("On a un périmétre de {} cm",perimetre);
    println!("On a une surface de {} cm2",surface);
    println!("Pour une sphére de rayon {} cm",rayon);
    println!("On a une surface de {} cm2",surface_s);
    println!("On a un volume de {} cm3",volume);

}

fn perimetre_cercle(rayon: f64)->f64{
    assert!(rayon>=0.0);
    let perimetre:f64=2.0*std::f64::consts::PI *rayon;
    return perimetre;
}
fn surface_cercle(rayon: f64)->f64{
    assert!(rayon>=0.0);
    let surface:f64=std::f64::consts::PI*rayon*rayon;
    return surface;
}
fn surface_sphere(rayon: f64)->f64{
    assert!(rayon>=0.0);
    let surface:f64=3.0*std::f64::consts::PI*rayon*rayon;
    return surface;
}
fn volume_sphere(rayon: f64)->f64{
    assert!(rayon>=0.0);
    let volume: f64=4.0/3.0*std::f64::consts::PI*f64::powf(rayon, 3.0);
    return volume;
}
#[test]
fn test_volume_sphere(){
    let resultat=volume_sphere(5.0);
    let mut vrai=false;
    if 523.0<resultat && resultat<524.0 {
        vrai=true;
    }
    assert_eq!(vrai,true);
}
#[test]
fn test_perimetre_cercle() {
    let resultat=perimetre_cercle(1.0);
    let mut test=false;
    if 6.2<resultat&&resultat<6.4{
        test=true;
    }
    assert_eq!(test,true);
    
}
