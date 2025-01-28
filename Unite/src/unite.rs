use std::{fmt:: Display,  ops::{Add, Sub}};

#[derive(Debug, PartialEq,Eq,Clone, Copy)]
pub struct Unite{
    pub(crate) masse:i8,
    pub(crate) temps:i8,
    pub(crate) longueur:i8,
    pub(crate) temperature:i8,
    pub(crate) intensité_electrique:i8,
    pub(crate) quantite_matiere:i8,
    pub(crate) intensité_lumineuse:i8

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
impl Display for Unite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut premier=true;
        if self.masse!=0 {
            write!(f,"kg")?;
            if self.masse!=1 {
                write!(f,"^{}",self.masse)?;
            }
            premier=false;
        }
        if self.longueur!=0{
            if !premier{
                write!(f,"*")?;
            }
            write!(f,"m")?;
            if self.longueur!=1{
                write!(f,"^{}",self.longueur)?;
            }
            premier=false;
        }
        if self.temps!=0 {
            if !premier{
                write!(f,"*")?;
            }
            write!(f,"s")?;
            if self.temps!=1 {
                write!(f,"^{}",self.temps)?;
            }
            premier=false;
        }
        if self.temperature!=0{
            if !premier{
                write!(f,"*")?;
            }
            write!(f,"K")?;
            if self.temperature!=1{
                write!(f,"^{}",self.temperature)?;
            }
            premier=false;
        }
        if self.intensité_electrique!=0{
            if !premier{
                write!(f,"*")?;
            }
            write!(f,"A")?;
            if self.intensité_electrique!=1{
                write!(f,"^{}",self.intensité_electrique)?;
            }
            premier=false;
        }
        if self.quantite_matiere!=0{
            if !premier{
                write!(f,"*")?;
            }
            write!(f,"mol")?;
            if self.quantite_matiere!=1{
                write!(f,"^{}",self.quantite_matiere)?;
            }
            premier=false;
        }
        if self.intensité_lumineuse!=0{
            if !premier{
                write!(f,"*")?;
            }
            write!(f,"cd")?;
            if self.intensité_lumineuse!=0{
                write!(f,"^{}",self.intensité_lumineuse)?;
            }
        }
        Ok(())
    }
}
