pub enum Stack { 
    Empty,
    Elem(i32, Stack) 
}
// INCORRECT

enum Pile{
    Rien,
    Lien(i32,Option<Box<Pile>>),
}



fn main() {
    let p1 = Pile(5,Some(Box::<p2>));
    let p2:Pile = Pile(4,None);
}


// CORRECT 
// p1 fait appel à la Pile 'p2' alors que la pile 'p2' est pas créé
// (creation d'une box via Box::new(p2)> au lieu de de Box::<p2>... pas très explicite...
// Appel au constructeur naturel de Box !

// 1️⃣ « Le premier élément se trouve sur la pile, les autres sur le tas »
enum Pile {
    Rien,
    Lien(i32, Option<Box<Pile>>), // i32 sur pile, Box sur le tas
}

fn main() {
    let p3 = Pile::Lien(1, None);
    let p2 = Pile::Lien(4, p3); // élément de base
    let p1 = Pile::Lien(5, Some(Box::new(p2))); // élément qui pointe vers p2

    // p1 représente la pile [5 -> 4]
}

//2️⃣ « Les manipulations seront plus complexes »
/*  */
fn push(s: Stack, val: i32) -> Stack {
    Stack::Elem(val, Box::new(s))
}

/*Tu dois toujours penser à déférencer 
(*rest) pour récupérer l’élément suivant.*/
fn pop(s: Stack) -> (Option<i32>, Stack) {
    match s {
        Stack::Empty => (None, Stack::Empty),
        Stack::Elem(v, rest) => (Some(v), *rest), // *rest déférencé 
        // *rest déréf la Box et renvoie la Stack et non le ptr vers la stack...
    }
}

fn increment_all(s: &mut Stack) {
    match s {
        Stack::Empty => (),
        Stack::Elem(ref mut val, ref mut next) => {
            *val += 1;
            increment_all(next); // traverse le Box
        }
    }
}

