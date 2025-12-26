mod matrix;
use crate::matrix::Matrix; // <-- import de la struct

/*
 N’hésitez pas à implémenter des tests automatiques 
 pour vous aider à valider l’implémentation 
 de chaque méthode tout au long du TP. 
*/

fn main() {
    println!("Hello, world!");
    let m = Matrix::new(2, vec![1.0, 2.0, 3.0, 4.0]);
    println!("{:?}", m);

    let id4 = Matrix::id(4);
    println!("{:?}", id4);

    let r3 = Matrix::random(3);
    println!("{:?}", r3);

}


#[test]
fn indexes() {
    let mut m = Matrix::new(2, vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(1, 0)], 3.0);

    m[(1,0)] = 5.0;
    assert_eq!(m[(1, 0)], 5.0);
}