
pub fn erathos(n:u32)-> Vec<u32> {
    // Si n < 2 : Retourner un vecteur vide
    if n < 2 {
        return vec![]
    }
    
    // # 1️⃣ Initialiser un tableau de booléens
    let mut is_prime = vec![true;(n+1) as usize] ;
    //is_prime[0..n] = true
    
    let mut count = 0; 
    for i in &is_prime {
        if *i == true {
            count +=1 ;
        }
    }
    println!("Nombre d'entier premiers : {}",count) ;
    
    // La fonction DOIT retourner quelquechose même si vecteur vide
    vec![]
}

fn main(){
    erathos(5);
    // >>> Nombre d'entier premiers : 6
}
