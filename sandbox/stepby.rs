pub fn crible(n:u32) -> Vec<u32>{
    // tableau de n+1 true (tous premier par defaut)
    let mut is_prime = vec![true;(n+1)as usize];

    // mutiple jusq'à sqrt(n)
    /* 
    Les nombres dans l'intervalle [0..n]
    Au pire sont multiples de racine de n...
    */
    for multiple in 0..=((n as f64).sqrt() as usize){
        if is_prime[multiple]{
            let mut pas = multiple*multiple;
            while (pas<=n as usize){
                is_prime[pas]=false;
                pas+=multiple ;
            }
        }
    }

    let mut premier = vec![]; 
    for i in 0..=n as usize {
        if is_prime[i]==false{
            premier.push(i as u32)
        }
    }
    premier
}

fn main(){
    let tableau = crible(10);
    for i in tableau {print!("{},",i);}
}