#[derive(Debug)]
pub struct Tree(pub Option<Box<Node>>);

#[derive(Debug)]
pub struct Node {
    value: i32,
    left: Tree,
    right: Tree,
}

impl Tree {
    /// Construit un arbre vide
    pub fn new() -> Self {
        Tree(None)
    }

    /// Construit une feuille avec une valeur donnée
    pub fn leaf(value: i32) -> Self {
        Tree(Some(Box::new(Node {
            value,
            left: Tree::new(),
            right: Tree::new(),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let t = Tree::new();
        assert!(t.0.is_none());
    }

    #[test]
    fn test_leaf_tree() {
        let t = Tree::leaf(42);
        assert!(t.0.is_some());
        let node = t.0.as_ref().unwrap();
        assert_eq!(node.value, 42);
        assert!(node.left.0.is_none());
        assert!(node.right.0.is_none());
    }
}

fn main() {
    let t1 = Tree::new();
    let t2 = Tree::leaf(12);

    println!("Arbre vide : {t1:#?}");
    println!("Feuille : {t2:#?}");
}
