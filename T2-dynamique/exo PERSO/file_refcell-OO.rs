use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Noeud {
    val: i32,
    lien: Option<Rc<RefCell<Noeud>>>,
}

#[derive(Debug)]
struct File {
    head: Option<Rc<RefCell<Noeud>>>,
    tail: Option<Rc<RefCell<Noeud>>>,
}

impl File {
    fn new() -> Self {
        File { head: None, tail: None }
    }

    fn push(&mut self, val: i32) {
        let nouveau = Rc::new(RefCell::new(Noeud { val, lien: None }));

        match self.tail.take() {
            Some(old_tail) => {
                // On relie l'ancien dernier au nouveau
                old_tail.borrow_mut().lien = Some(nouveau.clone());
                self.tail = Some(nouveau);
            }
            None => {
                // File vide : tête et queue pointent sur le nouveau noeud
                self.head = Some(nouveau.clone());
                self.tail = Some(nouveau);
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            if let Some(next) = old_head.borrow_mut().lien.take() {
                self.head = Some(next);
            🔑 En résumé
            Cette ligne :

            1)prend le lien vers le prochain noeud de la tête
            2)deconnecte l’ancien noeud de la file
            3)met à jour head pour qu’il pointe sur le suivant
            4)si la file ne contenait qu’un seul élément, head devient None
            } else {
                // La file devient vide
                self.tail = None;
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }
    
}
