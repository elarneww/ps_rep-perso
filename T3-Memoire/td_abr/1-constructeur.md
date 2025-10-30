# Si on veut instancier un arbre :

```rust
Tree(
    Some(
        Node {
            value: 12,
            left: Tree(
                Some(
                    Node {
                        value: 8,
                        left: Tree(None),
                        right: Tree(None),
                    },
                ),
            ),
            right: Tree(
                Some(
                    Node {
                        value: 27,
                        left: Tree(None),
                        right: Tree(None),
                    },
                ),
            ),
        },
    ),
)

```
## ne compile même pas

# 1- constructeur, methodes et tests
## a) -constructeur qui renvoie arbre vide...
```rust
pub fn (<valeur:i32> )-> Self{
    Struct Tree(Some(Node {
        valeur = valeur,
        left : Tree(None),
        right : Tree(None),
    })) // pas ';' on renvoie le node//Tree 
    
}
```
## a) Correction
```rust
impl Tree {
    /// Construit un arbre vide
    pub fn new() -> Self {
        Tree(None)
    }
}
```

## b) Construire une feuille
Je l'ai fait maladroitement précédemment
mais l'arbre renvoyé n'était pas vide, l'uttilisateur
pouvait rentrer une valeur.

## b) Correction
```rust
    /// Construit une feuille avec une valeur donnée
    pub fn leaf(value: i32) -> Self {
        Tree(Some(Box::new(Node {
            value,
            left: Tree::new(),
            right: Tree::new(),
        })))
    }

```
