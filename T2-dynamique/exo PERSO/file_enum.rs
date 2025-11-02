struct File {
    head : Link
    // no end: Link, on aprcourt la fiel par la tête pour arriver à la queue...
}

enum Link{
    Vide,
    Elem(Box<Noeud>),
}

struct Noeud {
    val  : i32,
    lien : Link,
}

impl File {
    fn new() -> Self {
        File { head: Link::Vide }
    }

    fn push(&mut self, val: i32) {
        let mut nouveau = Box::new(Noeud { val, lien: Link::Vide });

        match &mut self.head {
            Link::Vide => self.head = Link::Elem(nouveau),
            Link::Elem(mut courant) => {
                // Traverse jusqu'à la fin
                while let Link::Elem(next) = &mut courant.lien {
                    courant = next;
                }
                courant.lien = Link::Elem(nouveau);
            }
        }
    }
}
