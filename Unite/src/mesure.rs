use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};

use crate::unite::Unite;
pub struct Mesure<T> where 
T: Add<Output = T> +
Sub<Output = T> +
Mul<Output = T> + 
Div<Output = T> + 
Copy,{
    pub(crate) value:T,
    pub(crate) unite:Unite
}

impl<T> Mesure<T> where 
T: Add<Output = T> +
Sub<Output = T> +
Mul<Output = T> + 
Div<Output = T> + 
Copy,
{
    pub fn constance(valeur:T)->Mesure<T>{
        return Mesure{
            value:valeur,
            unite:Unite::NO_UNITE
        }
    }
    pub fn vitesse(valeur:T)->Mesure<T>{
        Mesure { value: valeur, unite: Unite::VITESSE }
    }
    pub fn distance(valeur:T)->Mesure<T> {
        Mesure { value: valeur, unite: Unite::DISTANCE }
    }
    pub fn temps(valeur:T)->Mesure<T> {
        Mesure{value:valeur,unite: Unite::TEMPS }
    }
    pub fn acceleration(valeur:T)->Mesure<T> {
        Mesure{value:valeur,unite:Unite::ACCELERATION }
    }
    pub fn get_value(&self)->T{
        return self.value;
    }
    pub fn get_unite(&self)->Unite{
        self.unite
    }
}
impl<T> Add for Mesure<T>
where T: Add<Output = T> +
Sub<Output = T> +
Mul<Output = T> + 
Div<Output = T> + 
Copy,
    {
        type Output = Mesure<T>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.unite!=rhs.unite{
            panic!("Addition impossible pour deux unite différent");
        }
        Self{value:self.value+rhs.value,unite:self.unite}
    }
}
impl <T> Sub for Mesure<T>
where T: Add<Output = T> +
Sub<Output = T> +
Mul<Output = T> + 
Div<Output = T> + 
Copy,  {
    type Output=Mesure<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.unite!=rhs.unite{
            panic!("Soustration impossible pour deux unite différent");
        }
        Self{value:self.value-rhs.value,unite:self.unite}
    }
}
impl <T> Mul for Mesure<T>
where T: Add<Output = T> +
Sub<Output = T> +
Mul<Output = T> + 
Div<Output = T> + 
Copy,  {
    type Output = Mesure<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self{value:self.value*rhs.value,unite:self.unite+rhs.unite}
    }
}
impl <T> Div for Mesure<T> 
where T: Add<Output = T> +
Sub<Output = T> +
Mul<Output = T> + 
Div<Output = T> + 
Copy, {
    type Output = Mesure<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Self{value:self.value/rhs.value,unite:self.unite-rhs.unite}
    }
}
impl <T> Display for Mesure<T>
where T:Display +
Add<Output = T> +
Sub<Output = T> +
Mul<Output = T> + 
Div<Output = T> + 
Copy,{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {}",self.value,self.unite)
    }
}
