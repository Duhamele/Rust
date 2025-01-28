use std::ops::{Add, Div, Mul, Sub};

use crate::mesure::Mesure;
use crate::unite::Unite;

impl<T> Mesure<T>
where T: Add<Output = T> +
Sub<Output = T> +
Mul<Output = T> +
Div<Output = T> +
Copy,{
    pub const NOMBRE_AVOGADRO: Mesure<f64> =Mesure{
        value:6.02214076e23,
        unite:Unite{masse:0,
            longueur:0,
            temps:0,
            temperature:0,
            intensité_electrique:0,
            quantite_matiere:-1,
            intensité_lumineuse:0
        }
    };
    pub const GRAVITATIONAL_CONSTANT:Mesure<f64>=Mesure{
        value:6.674e-11,
        unite:Unite{
            masse:-1,
            temps:-2,
            longueur:3,
            temperature:0,
            intensité_electrique:0,
            quantite_matiere:0,
            intensité_lumineuse:0
        }
    };
}