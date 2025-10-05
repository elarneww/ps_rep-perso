# Mémo mémoire et CPU

## 32/64 bits
- 32 bits → 0 .. 2**32-1
- 64 bits → 0 .. 2**48-1 (certaines zones réservées)

## Stack / Heap
- Stack : grandit vers le bas, contient variables locales et frames de fonction, accès très rapide
- Heap : grandit vers le haut, contient objets dynamiques, allocations plus lentes, nécessite free/malloc
- Collision stack/heap possible mais rare
- Pile est souvent dans le cache CPU → accès ultra rapide
- Heap moins souvent dans cache → accès plus lent

## Cache CPU
- Stocke les zones les plus fréquemment accédées
- Accès à la mémoire locale = rapide grâce au cache

# -- Syntaxe rust

## Déreferencement du pointeur nul = erreur 
- Option<T> Some(r) pour checker, force à écrire un IF ELSE
- On peux enlever l'Option via unwrap, plus rapide, no IF/ELSE
    mais panique facilement
- Result similaire au type Option<T>, contient Ok et E qui représente l'erreur au lieu d'avoir : Some et None..
- Result permet personnaliser via match le msg d'err
- Conversion possible d'un type Option en Result (see ex CM)
