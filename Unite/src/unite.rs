use std::ops::{Add, Sub};

#[derive(Debug, PartialEq,Eq,Clone, Copy)]
pub struct Unite{
    masse:i8,
    temps:i8,
    longueur:i8,
    temperature:i8,
    intensité_electrique:i8,
    quantite_matiere:i8,
    intensité_lumineuse:i8

}
impl Unite{
    pub const NO_UNITE: Unite=Unite{
        masse: 0,
        temps: 0,
        longueur: 0,
        temperature:0,
        intensité_electrique:0,
        quantite_matiere:0,
        intensité_lumineuse:0
    };
    pub const VITESSE:Unite=Unite{
        masse:0,
        temps:-1,
        longueur:1,
        temperature:0,
        intensité_electrique:0,
        quantite_matiere:0,
        intensité_lumineuse:0
    };
    pub const DISTANCE:Unite=Unite{
        masse:0,
        temps:0,
        longueur:1,
        temperature:0,
        intensité_electrique:0,
        quantite_matiere:0,
        intensité_lumineuse:0
    };
    pub const SURFACE:Unite=Unite{
        masse:0,
        temps:0,
        longueur:2,
        temperature:0,
        intensité_electrique:0,
        quantite_matiere:0,
        intensité_lumineuse:0
    };
    pub const VOLUME:Unite=Unite{
        masse:0,
        temps:0,
        longueur:3,
        temperature:0,
        intensité_electrique:0,
        quantite_matiere:0,
        intensité_lumineuse:0
    };
    

}
impl Add for Unite {
    type Output = Unite;
    fn add(self, rhs: Self) -> Self::Output {
        Self{masse:self.masse+rhs.masse,
        temps:self.temps+rhs.temps,
        longueur:self.longueur+rhs.longueur,
        temperature:self.temperature+rhs.temperature,
        intensité_electrique:self.intensité_electrique+rhs.intensité_electrique,
        quantite_matiere:self.quantite_matiere+rhs.quantite_matiere,
        intensité_lumineuse:self.intensité_lumineuse+rhs.intensité_lumineuse}
    }
    
}
impl Sub for Unite {
    type Output = Unite;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{masse:self.masse-rhs.masse,
            temps:self.temps-rhs.temps,
            longueur:self.longueur-rhs.longueur,
            temperature:self.temperature-rhs.temperature,
            intensité_electrique:self.intensité_electrique-rhs.intensité_electrique,
            quantite_matiere:self.quantite_matiere-rhs.quantite_matiere,
            intensité_lumineuse:self.intensité_lumineuse-rhs.intensité_lumineuse}
    }
    
}