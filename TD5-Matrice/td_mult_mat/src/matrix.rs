/*


    Implémentez pour la structure Matrix un constructeur 
    fn new(n: usize, values: Vec<Element>) → Self. 
    Attention à bien vérifier grace à une assertion 
    que le vecteur  values contient bien \(n^2\) valeurs.


*/
use rand::{thread_rng, Rng};
use std::ops::{Index,IndexMut};

type Element = f64;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    n: usize,
    values: Vec<Element>,
}
impl Matrix{
    // constructeur publique
    pub fn new(n: usize, values: Vec<Element>)-> Self{
        assert!(
            n * n == values.len()
        );
        Matrix {n,values}
    }
    /// un deuxième constructeur fn id(n:usize) → Self qui retourne la matrice identité de dimension \(n\).
    pub fn id(n:usize) -> Self {
        let mut values = vec![0.0;n*n];
        for i in 0..n  {
            // OU  m[(i, i)] = 1.0;
            values[i*n+i] = 1.0 ;
        }
        Matrix{n, values}
    }
    /// retourne une Matrice aléatoire d’une distribution uniforme sur [-1.0, 1.0]
    

    pub fn random(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let values = (0..n*n)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
        Matrix { n, values }
    }
}

/*
Implémentez les traits std::ops::Index<(usize, usize)> et std::ops::IndexMut<(usize, usize)> pour la structure Matrix de manière à
 ce que l’on puisse accéder aux éléments de la matrice avec
 la syntaxe m[(1,0)] = 5.0; m[(0, 0)] (m[(1, 0)]  

Soit un élement x qui appartient à la matrice M de coord (i,j).
sur une ligne... bah...
[l1,l2,l3,_,li,_,ln]... c'est l'elemetn à l'indice
n x (i-1) +j
Si on commence à 0 : n x (i)  +j

 */

 //use std::ops::{Index,IndexMut};

/// COrrection cipié du repo de Gabriel je n'arrivais pas 
// à impl l'index (doc du type Index::A,(...)T,C,G)
// J'enlève la généricité de la correction
impl Index<(usize, usize)> for Matrix { // Index <T> Matrix<T>
    // indiques ce que renvoies quand on indexe → ici Element (f64)
    type Output = Element;
    //On choisis le type d’index que ton objet devra accepter → ici (usize, usize)
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j)= index ;
        // comment récupérer l’adresse → le mapping i,j → n*i + j
        &self.values[self.n * index.0 + index.1]
    }
}


impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index ;
        
        &mut self.values[self.n * index.0 + index.1]
    }
}
