# ```
```rust
struct Player {
    name: String,
    score: i32,
}

impl Player {
    fn new(name: &str) -> Self {
        Self { name: name.to_string(), score: 0 }
    }

    fn add_points(&mut self, pts: i32) {
        self.score += pts;
    }

    fn display(&self) {
        println!("{}: {}", self.name, self.score);
    }
}

fn main() {
    let mut p = Player::new("Alice");
    p.add_points(10);
    p.display();
}
```
# 🧠 2️⃣ Vecteur + boucle + lecture utilisateur (15 min)
```rust
use std::io;

fn main() {
    let mut numbers = Vec::new();

    println!("Tape quelques nombres (0 pour arrêter):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur lecture");
        let n: i32 = input.trim().parse().unwrap_or(0);
        if n == 0 { break; }
        numbers.push(n);
    }

    println!("Tu as tapé : {:?}", numbers);
}

```
# ⚙️ 3️⃣ Enum + Box (optionnel, 10 min si t’as le courage)

```rust
enum Stack {
    Empty,
    Elem(i32, Box<Stack>),
}

fn main() {
    let s = Stack::Elem(1, Box::new(Stack::Elem(2, Box::new(Stack::Empty))));
    println!("{:?}", s);
}

```
## Mais le ENum recursif est... chiant...
# Exemple classique
```rust
#[derive(Debug, PartialEq)]
pub struct Node {
    elem: i32,
    next: Stack,
}

#[derive(Debug, PartialEq)]
pub enum Stack {
    Empty,
    Elem(Box<Node>),
}

```
# Comparaison
## Avec struct intermédiaire
```rust

pub struct Node {
    val: i32,
    next: Stack,
}

pub enum Stack {
    Empty,
    Elem(Box<Node>),
}

fn increment_all(s: Stack) -> Stack {
    match s {
        Stack::Empty => Stack::Empty,
        Stack::Elem(node) => {
            Stack::Elem(Box::new(Node {
                val: node.val + 1,
                next: increment_all(node.next),
            }))
        }
    }
}
```
## Sans struct intermédiaire
```rust
fn increment_all(s: Stack) -> Stack {
    match s {
        Stack::Empty => Stack::Empty,
        Stack::Elem(val, rest) => {
            // Tu dois reconstruire le Box à chaque étape
            Stack::Elem(val + 1, Box::new(increment_all(*rest)))
        }
    }
}
```