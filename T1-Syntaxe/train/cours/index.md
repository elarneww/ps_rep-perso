# Plan de cours Rust / Mémoire / Structures

## 1. Types de base

- Entiers : i32, i64, u32, u64
- Plage de valeurs sur 32 et 64 bits
- Bool, char, float
- Références & mutabilité

## 2. Tableaux et slices
- Déclaration et initialisation `[1,2,3]`
- Accès direct vs slice `&array[1..], array[1] = 1 vs &array[1..]; `

- Comparaison avec `assert_eq!`
- Exemple pratique : `tableaux_et_slices()`

## 3. Vecteurs (Vec<T>)
- Création et mutabilité
- Accès direct `v[i]` vs sécurisé `v.get(i)`
- Modification et push/pop
- Exemple pratique : `exemples_vecteurs()`

## 4. Pile / Heap / Cache voir notes.md
- Stack : allocation automatique, LIFO, rapide
- Heap : allocation dynamique, malloc/free, plus lent
- Cache CPU : accélération de la mémoire locale
- Visualisation mémoire : stack ↓ / heap ↑

## 5. Structures composites et gestion d’erreurs
- 1) Structs, tuples et deplacement mémoires let (x, y, z) = tuple; et Uttilisation RGB
- 2) Constructeur
- 3) Traits
- 4) 3.1.5. Tuples
- 5) Enum = type disjoint comment récupérer l'interieur ?
- 6) 

## 6. Notes et rappels
- Comparaison tableau vs slice vs vecteur
- Quand utiliser stack vs heap
- Accès sûr vs panique
