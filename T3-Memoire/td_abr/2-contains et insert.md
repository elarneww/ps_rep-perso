 # 2- contains et insert
Pour chaque nœud l’on compare la valeur cherchée à la valeur courante. Si elle sont identiques alors nous retournons true. Si ce n’est pas le cas nous récursivons soit sur l’arbre gauche lorsque la valeur est inférieure ou l’arbre droit lorsque la valeur est supérieure.

 ## a) Insertion essai

/// Inserts `value` into the tree.
/// Returns `TreeOpError::ValueAlreadyExists` iff the `value` was already
/// contained in the tree.
Algorithme récursif :
structure tree t.

Si t.valeur = n
    valeur déja présente, aucune insertion n
Sinon si t.valeur < n :
    mettre dans l'arbre droit
Sinon 
    mettre dans l'arbres gauche
## b) Details techniques
On veut insérer une valeur... donc on se balade recursivement
dans l'arbre...
puis on trouve le pointeur vers l'emplacement de du noeuds ou on veut insérer... Cela doit être une référence mutable.
Vers l'emplacement où on l'on veut insérer

## c) Correction Insert
```rust
pub fn insert(&mut self, value: T) -> Result<(), TreeOpError> {
        match self.0 {
            Some(ref mut n) => match value.cmp(&n.value) {
                Ordering::Equal => Err(TreeOpError::ValueAlreadyExists),
                Ordering::Less => n.left.insert(value),
                Ordering::Greater => n.right.insert(value),
            },
            None => {
                *self = Tree::leaf(value);
                Ok(())
            }
        }
    
```
## d) Correction Contain
```rust
/// Returns true iff `value` belongs to the tree.
    pub fn contains(&self, target: T) -> bool {
        match self.0 {
            Some(ref n) => match target.cmp(&n.value) {
                Ordering::Equal => true,
                Ordering::Less => n.left.contains(target),
                Ordering::Greater => n.right.contains(target),
            },
            None => false,
        }
    }
```