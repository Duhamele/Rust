pub mod unite;
use unite::Unite;
pub mod mesure;
use mesure::Mesure;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_constance(){
        let mesure=Mesure::constance(5.0);
        println!("Unite Test");
        assert_eq!(5.0,mesure.get_value());
    }
    #[test]
    fn test_mult() {
        let mesure=Mesure::distance(5.0);
        let mesure1=Mesure::distance(8.0)*mesure;
        assert_eq!(mesure1.get_unite(),Unite::SURFACE);
        
    }
    #[test]
    fn test_display(){
        let vitesse=Mesure::vitesse(956.645);
        println!("{}", vitesse);
    }
    #[test]
    #[should_panic]
    fn test_erreur(){
        let mesure=Mesure::vitesse(5.0);
        let mesure1=Mesure::distance(8.0)+mesure;
    }
}