fn racine(n : f64) -> Option<f64> {
    if n > 0.0 {
        Some(n.sqrt())
    } else {
        None
    }
}
racine(25.0);  // -> Some(5.0)
racine(-25.0); // -> None

// Vérification
if let Some(r) = racine(25.0) { 
    println!("La racine est {}", r);
} else {
    println!("Pas de racine réelle");
}

// panique si racine -> None  dans les 2 cas
let v : f64 = racine(25.0).unwrap(); 
let v : f64 = racine(25.0).expect("erreur: racine d'un nombre négatif"); 