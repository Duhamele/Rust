use crate::mesure;
use crate::mesure::Mesure;
use crate::unite::Unite;

impl<T> Mesure<T>{
    pub const NOMBRE_AVOGADRO: Mesure<T> =Mesure{value:6.022e23,
        unite:Unite{masse:0,
            longueur:0,
            temps:0,
            temperature:0,
            intensité_electrique:0,
            quantite_matiere:-1,
            intensité_lumineuse:0
        }};
}