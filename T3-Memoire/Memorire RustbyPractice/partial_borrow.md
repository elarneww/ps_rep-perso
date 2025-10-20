
fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (ref s1,ref s2) = (t.0,t.1);// marche pas
    // 👉 Ici, tu déstructures une nouvelle paire construite à partir de (t.0, t.1).

    // La ligne du bas essaie d'emprunter t mais qui est partielle ment emprunté
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}


On fait donc un umprunt total par s1,S2 :

fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (ref s1,ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}


Ici, c’est totalement différent :
- Tu déstructures par emprunt (le mot-clé ref fait que tu prends une référence vers les champs).

- Donc tu ne bouges rien hors de t.

- Résultat : t reste possédé par la fonction main, mais ses champs sont temporairement empruntés en lecture par s1 et s2.