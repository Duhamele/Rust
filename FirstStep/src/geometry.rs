pub struct Carre{
    pub cote: f64
}
pub struct Rectangle{
    largeur: f64,
    longuer: f64
}

pub fn created_carre(cote:f64)->Carre{
    return Carre{cote:cote};
}

pub trait Geo{
    fn perimetre(&self)->f64;
    fn aire(&self)->f64;
    fn affiche(&self);
}
impl Geo for Carre{
    fn perimetre(&self)->f64{
        return self.cote*4.0;
    }
    fn aire(&self)->f64{
        return self.cote*self.cote;
    }
    fn affiche(&self){
        println!("Figure de Type carre:\n \tcote : {}",self.cote);
    }
}
impl Geo for Rectangle{
    fn perimetre(&self)->f64{
        return (self.largeur+self.longuer)*2.0;
    }
    fn aire(&self)->f64{
        return self.largeur*self.longuer;
    }
    fn affiche(&self){
        println!("Figure de Type rectanglee:\n\tlargeur : {}\n\tlongeur : {}",self.largeur,self.longuer);
    }
}