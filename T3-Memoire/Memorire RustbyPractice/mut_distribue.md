

fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &s;// Evidemment on veut que p soit mutable
    mais le mut de la variable s d'origine
     ne se distribue pas automatiquement ici.
    
    p.push_str("world");

    println!("Success!");
}

fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s; Donc on emprunte pas s jsute pour voir.
    
    p.push_str("world");

    println!("Success!");
}