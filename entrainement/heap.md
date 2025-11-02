```rust
fn main() {
    let b = Box::new(10);
    let v = vec![1, 2, 3];
    let s = String::from("abc");
    use std::collections::HashMap;
    let mut h = HashMap::new();
    h.insert("clé", 42);
}
```

📦 Exemple visuel simplifié
PILE                              TAS
─────────────────────────         ───────────────────────────────
b ─────────────┐             ┌──> [10]
v.ptr ─────────┼────────┐    ├──> [1][2][3]
s.ptr ─────────┼──────┐ │    ├──> ['a']['b']['c']
h.ptr ─────────┘      │ │    └──> {"clé": 42}
─────────────────────────
