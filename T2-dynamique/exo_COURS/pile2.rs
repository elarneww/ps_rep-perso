pub enum Pile {
    Empty,
    Elem(Box<Node>) // Node privé
}
struct Node { 
    elem: i32,
    next: Stack,
}

fn main() {
    let p3 = Pile::Lien(1, None);
    let p2 = Pile::Lien(4, p3); // élément de base
    let p1 = Pile::Lien(5, Some(Box::new(p2))); // élément qui pointe vers p2

    // p1 représente la pile [5 -> 4]
}
 


