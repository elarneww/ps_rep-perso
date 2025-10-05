fn affiche(v: Variant) {
    match v {
        Variant::Rien => println!("Le variant est vide"),
        Variant::Nombre(n) => println!("Le variant contient le nombre {}", n),
        Variant::Texte(s) => println!("Le variant contient le texte {}", s),
    }
}
    affiche(v1);
    affiche(v2);
    affiche(v3);