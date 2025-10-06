boucle_infinie
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


2️⃣ Séparer les responsabilités

Ta fonction fait trois choses à la fois : 
1) initialiser le tableau, 
2) marquer les multiples, 
3) collecter les premiers.