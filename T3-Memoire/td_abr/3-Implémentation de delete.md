# 3-Implémentation de delete
    Implémentez la méthode pub fn delete(&mut self, value: i32) en suivant l’algorithme précédent. La méthode retourne true lorsque la suppression est possible et retourne false lorsque la valeur n’est pas trouvée dans l’ABR.
## a) Reflexion
pub fn delete(&mut self, value i32){
    match self.0{
        Some(ref mut n) {
            Ordering::Equal => on doit supp ce noeud... mais quid de ses fils ?
            Ordering::Lesser => faut supprimmer le fils de gauche
            Ordering::Greater => faut supprimmer le fils de droite
        }
        None {le noeud vide... Ne rien faire...}
    }
}
## b) Correc avancées Gabriel
```rust
/// Deletes `value` from the tree.
    /// When the value is not found the tree, `TreeOpError::NoValue` is returned.
    pub fn delete(&mut self, target: &T) -> Result<(), TreeOpError> {
        // Destructure `n` thanks to default binding modes and get mutable references
        // on each field of `n`
        //
        // # Example:
        // ```
        // Some(n) = n,          // `n` is of type `&mut Box<Node<T>>`
        // Some(n) = n.as_mut(), // `n` is of type `&mut Node<T>`
        // ```
        // We can then destructure `n` into its fiels: value, left and right
        let Node { value, left, right } = match &mut self.0 {
            Some(n) => n.as_mut(),
            None => return Err(TreeOpError::NoValue),
        };

        match target.cmp(value) {
            Ordering::Equal => {
                match left.inorder_predecessor() {
                    Some(predecessor) => *value = predecessor,
                    None => self.0 = right.0.take(),
                }
                Ok(())
            }
            Ordering::Less => left.delete(target),
            Ordering::Greater => right.delete(target),
        }
    }

```