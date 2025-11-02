/// Une pile définie récursivement.
#[derive(Debug,PartialEq)]
pub struct Stack {
    head: Link,
}
// implémentation interne... privé
#[derive(Debug,PartialEq)]
enum Link {
    Empty,
    Elem(Box<Node>)
}
// 3️⃣ Node — le nœud privé
#[derive(Debug,PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

#[test]
fn should_push_an_element_on_the_stack() {
    let mut s = empty_stack();
    push(&mut s, 1);
    assert_eq!(1, top(&s).unwrap());
}

/// Empile un élément
/// Problème de compilation :
/// error[E0507]: cannot move out of `*next` which is behind a shared reference
/// 
pub fn push(s: &mut Stack, elem: i32) {
    let next = &mut s.head;
    let newtop = Box::new(Node{elem, next: *next});  
    // move occurs because `*next` has type `Link`, which does not implement the `Copy` trait
    *s = Stack{head: Link::Elem(newtop)};
}