#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Affiche l’arbre avec indentation
    fn print(&self, indent: usize) {
        println!("{}{}", "  ".repeat(indent), self.name);
        for child in &self.children {
            child.print(indent + 1);
        }
    }
}

fn main() {
    // Racine = Daniel (grand-parent)
    let mut daniel = Node::new("Daniel");

    // Enfants de Daniel
    let mut patricia = Node::new("Patricia");
    let mut marc = Node::new("Marc");
    let mut luc = Node::new("Luc");

    // Enfants de Patricia
    patricia.add_child(Node::new("Arnaud"));
    patricia.add_child(Node::new("Loic"));
    patricia.add_child(Node::new("Damiens"));

    // Enfants de Marc
    marc.add_child(Node::new("Izaac"));
    marc.add_child(Node::new("Nao"));
    marc.add_child(Node::new("Maë"));

    // Ajout à Daniel
    daniel.add_child(patricia);
    daniel.add_child(marc);
    daniel.add_child(luc);

    // Affichage de l’arbre
    daniel.print(0);
}
