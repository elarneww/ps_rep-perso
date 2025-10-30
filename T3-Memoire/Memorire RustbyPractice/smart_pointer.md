
    // Box = smart poitner qui possède sa valeur sur le heap.
    // Rust gère automatiquement la libération mémoire quand le box sort du scope.
Possession (ownership) : le Box possède la valeur.

Peut être déplacé ou assigné à une autre variable → transfert complet de l’ownership.

Allocation dynamique sur le heap, donc utile si la valeur doit vivre longtemps ou être très grosse.

Exemple :
``` rust
let x = Box::new([0u8; 1_000_000]);
let y = x; // move rapide
```

``` rust
```
Sans box on a un problème
x est trop gros et n’implémente pas Copy.
Rust doit déplacer la valeur → x n’est plus utilisable après.
Si tu voulais garder x → .clone() obligatoire, ce qui serait très coûteux (copie de 1 million d’octets).
``` rust
let x = [0u8; 1_000_000]; // tableau d’un million d’octets
let y = x; // Erreur : move obligatoire

```
```rust
let x = Box::new(5); // alloue 5 sur le heap
let mut y = x;        // move : y possède maintenant 5
*y = 10;              // on peut modifier la valeur
```
```rust
fn main() {


    let x = Box::new(5); // alloue sur le tas
    
    let mut y =Box::new(2) ;      // update this line, don't change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
```

