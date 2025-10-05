fn main() {
    // Partie 1 : tableaux et slices
    tableau_et_slices();
}

fn tableau_et_slices() {
    let mut tableau = [1, 2, 3, 4, 5];
    assert_eq!(tableau, [1, 2, 3, 4, 5]);
    // Comparaison tableau à tableau
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    for x in array {
        print!("{} ", x);
    }
    println!();


}

// ===========================
// Partie 3 : Vecteurs et accès sécurisé
// ===========================

fn exemples_vecteurs() {
    // Création d'un vecteur mutable
    let mut v = vec![1, 2, 3, 4, 5];

    // Accès direct (peut panique si l'indice n'existe pas)
    let x: &i32 = &v[2];
    println!("v[2] (accès direct) = {}", x);

    // Accès sûr avec get()
    match v.get(2) {
        Some(y) => println!("v[2] (get) == {}", y),
        None => println!("indice non-existant"),
    }

    // Modification du vecteur (mutable)
    v[2] = 42;
    println!("v après modification : {:?}", v);
}

enum Variant { 
    Rien,
    Nombre(i32),
    Texte(String),
}

fn extraction_enum() {
    let v1: Variant = Variant::Rien;
    let v2: Variant = Variant::Nombre(42);
    let v3: Variant = Variant::Texte(String::from("Hello"));

    // Avec if let
    if let Variant::Nombre(nombre) = v2 {
        println!("Le nombre est : {}", nombre);
    } else {
        println!("Ce n'est pas un nombre");
    }
    // OU ON UTTILISE LE PATERN MATCHING
}
