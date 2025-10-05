let mut v = vec![1, 2, 3, 4, 5];

let n : usize = 100; 
let zero_v = vec![0; n];
let one_v = vec![1; n];
// Objet::new()
let v1 = Vec::new();             // vecteur vide

let mut v: Vec<i32> = Vec::new();
v.push(5);
v.push(6);
v.push(7);

// Objet::from()

    // crée un Vec<i32> à partir d’un tableau
let v2 = Vec::from([1, 2, 3]);   

let mut ada = Personne {
    nom : String::from("Ada Lovelace"),
    année_naissance: 0,
};

use std::path::PathBuf;

let chemin = PathBuf::from("/home/user/fichier.txt");
    //→ crée un chemin de fichier manipulable 
    // dynamiquement (comme String).

use std::time::Duration;

let d = Duration::from_secs(5);

// Objet.into()
let s: String = "Alice".into();
let v: Vec<i32> = [1, 2, 3].into();
//Objet ::default
let v = Vec::<i32>::default(); // même effet que Vec::new()
let s = String::default();     // chaîne vide
let n = i32::default();        // 0

//4️⃣ Cas bonus : ::clone() et ::copy()
let b = bool::from(1u8); // true
let i = i32::from(42u8); // conversion entière
