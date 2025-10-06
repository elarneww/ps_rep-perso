## Toujours ecrire pseudo-code pour pas se gourer
```
Fonction mark_multiples(tableau is_prime)
    taille <- longueur de is_prime

    // Boucle sur tous les diviseurs candidats à être premiers
    Pour diviseur allant de 2 à racine(taille) inclus
        // Si le diviseur est encore marqué comme premier
        Si is_prime[diviseur] est vrai alors

            // On commence à marquer les multiples à partir de diviseur*diviseur
            // Pourquoi ? Parce que tous les multiples plus petits ont déjà été marqués
            multiple <- diviseur * diviseur

            // Tant que le multiple ne dépasse pas la taille du tableau
            Tant que multiple < taille
                // Marquer ce multiple comme non premier
                is_prime[multiple] <- faux

                // Passer au multiple suivant
                multiple <- multiple + diviseur
            Fin Tant que
        Fin Si
    Fin Pour
Fin Fonction

```
## Boucle infinie

```rust
for multiple in 0..=((n as f64).sqrt() as usize) {
    if is_prime[multiple] {
        let mut pas = multiple * multiple;
        while pas <= n as usize {
            is_prime[pas] = false;
            pas += multiple;
        }
    }
}
```
Si le multiple vaut 0 alors le pas
vaut lui aussi 0

Il faut clarifier la boucle
```rust
fn mark_multiples(is_prime: &mut Vec<bool>) {
    let taille = is_prime.len();

    // On teste tous les pass de 2 jusqu'à racine de taille
    for pas in 2..=((taille as f64).sqrt() as usize) {
        if is_prime[pas] {
            // Marquer tous les multiples de 'pas', à partir de pas*pas
            let mut multiple = pas * pas;
            while multiple < taille {
                is_prime[multiple] = false;
                multiple += pas; // passer au multiple suivant
            }
        }
    }
}


```