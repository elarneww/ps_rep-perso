```rust
fn print_str(&s: &String) {
    println!("{}", s)
}

```

equivaut exactement à :
```rust
fn print_str(tmp: &String) {
    let s = *tmp; // <-- move implicite ici !
    println!("{}", s)
}

```
On peux empêcher le move via clone

```rust
fn print_str(&s: &String) {
    let s = s.clone(); // clone du contenu de la String
    println!("{}", s)
}
```
