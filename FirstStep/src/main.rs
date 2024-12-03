fn main() {
    let rayon=5.2;
    let perimetre=perimetre_cercle(rayon);
    println!("Le périmétre d'un cercle de rayon {} est égale à {}",rayon,perimetre)
}

fn perimetre_cercle(rayon: f64)->f64{
    assert!(rayon>=0.0);
    let perimetre:f64=2.0*std::f64::consts::PI *rayon;
    return perimetre;
}
