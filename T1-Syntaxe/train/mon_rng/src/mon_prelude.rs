//importer la bibliothèque avec use ( Penser à Cargo.toml)
use rand::prelude::*;
fn main(){
    //Prendre random number Generator (rng()) :
    let mut rng = rand::rng() // on va le modifier plus tard
    
    println!("char: '{}'",rng.random::<char>()) ;

}
