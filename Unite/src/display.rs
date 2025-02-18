use std::string::*;

use crate::Mesure::*;
use crate::Unite::*;

pub struct display_mesure{

}
impl display_mesure{
    pub fn use_imperial_system()->void{}
    pub fn use_international_system()->void{}
    pub fn display(mesure:Mesure)->string;
    
}