//V1_____________________________________________

pub enum Pile { 
    Vide,
    Elem(i32, Pile) // recursive without indirection
                     // help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `Pile` representable
                     //     Elem(i32, Box<Pile>)
}

//V2_____________________________________________

/// Le premier élément se trouve sur la pile, les autres sur le tas

/// => les manipulations seront plus complexes
enum Pile{
    Vide,
    Elem(val:i32,Option<Box<Pile>>)//lourd
}

//V3________/// error[E0446]: private type `Node` in public interface
pub enum Pile{ // full heap
    Vide,
    Elem(Option<Box<Pile>>)//lourd
}

struct Node {
    val:i32, // Pile :D
    next : Pile,
}
//V4__________________Parfait mais problème lors du push (empilement)
//___________head_______enum-link(2)__________node(val)[struct]______________
pub struct Pile{
    head: Link,}
// Pile se transofrme en struct et link prend l'enum sur la heap
enum Link {
    Vide,
    Elem(Box<Node>)
}
struct Node {
    val:i32,
    next : Link,
}

pub fn push(s: &mut Pile, elem: i32) {
    let next = &mut s.head;
    let newtop = Box::new(Node{elem, next: *next});  
// move occurs because `*next` has type `Link`, which does not implement
//  the `Copy` trait
    *s = Pile{head: Link::Elem(newtop)};
}
/*__________________________________________________________________
    let newtop = Box::new(Node{elem, next: *next})
Link n’implémente pas le trait Copy donc Rust interdit le déplacement
 du sommet de la pile HEAD !!!!
//__________________________________________________________________
📊 Schéma :
Pile
 └── head ──► Link::Elem(Box<Node>)
                     │
                     ├── elem: 42
                     └── next ──► Link::Elem(Box<Node>)
                                          ├── elem: 15
                                          └── next ──► Link::Empty
*///____________________________________________________________________

//V5__aucune différence____________Étape 5 : tous les éléments sur le tas

pub fn push(s: &mut Pile, elem: i32) {
    let next = std::mem::replace(&mut s.head, Link::Empty); 
    let newtop = Box::new(Node{elem, next});
    s.head = Link::Elem(newtop);
}

/