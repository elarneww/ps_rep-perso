// Définition de la "classe"
struct Cercle {
    rayon: u16,
}

// Implémentation des méthodes
impl Cercle {
    // "Constructeur"
    fn new(rayon: u16) -> Self {
        Self { rayon }
    }

    // Méthode d'instance
    fn diametre(&self) -> u16 {
        self.rayon * 2
    }
}

fn main() {
    unsafe {
        let cercle1 = Cercle::new(5);
        println!("Rayon cercle 1: {}", cercle1.rayon);
        println!("Diamètre : {}", cercle1.diametre());

        let mut cercle2 = Cercle::new(6);
        println!("Rayon cercle2 : {}", cercle2.rayon);

        cercle2.rayon *= 10;

        println!("\nRayon cercle2 après *10: {}", cercle2.rayon);

        let addresse = &mut cercle2;
        addresse.rayon = (addresse.rayon) / 2;
        println!("rayon de addresse={} !", addresse.rayon);
        println!("rayon de cercle2={} !", cercle2.rayon);

        let addresse1 = &mut cercle2;
        let addresse2 = &mut cercle2;

        addresse1.rayon += 1;
        addresse2.rayon += 1;
    }
}
