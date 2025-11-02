/// Une pile définie récursivement.
/// 
/// Problème de compilation
/// error[E0446]: private type `Node` in public interface
/// 
#[derive(Debug,PartialEq)]
pub enum Stack {
    Empty,
    Elem(Box<Node>) // can't leak private type
                    // struct Node {
                    // ----------- `Node` declared as private

}

#[derive(Debug,PartialEq)]
struct Node { 
    elem: i32,
    next: Stack,
}