#[derive(Debug)]
struct Plat {
    nom: String,
}

impl Plat {
    fn new(nom: &str) -> Self {
        Plat {
            nom: nom.to_string(),
        }
    }
}

#[derive(Debug)]
struct Phase {
    nom: String,
    options: Vec<Plat>,
}

impl Phase {
    fn new(nom: &str) -> Self {
        Phase {
            nom: nom.to_string(),
            options: Vec::new(),
        }
    }

    fn add_plat(&mut self, plat: Plat) {
        self.options.push(plat);
    }

    fn print(&self) {
        println!("{}", self.nom);
        for plat in &self.options {
            println!("  - {}", plat.nom);
        }
    }
}

fn main() {
    let mut entrees = Phase::new("Entrées");
    entrees.add_plat(Plat::new("Salade verte"));
    entrees.add_plat(Plat::new("Soupe à l'oignon"));
    entrees.add_plat(Plat::new("Carpaccio de bœuf"));

    let mut plats = Phase::new("Plats");
    plats.add_plat(Plat::new("Steak frites"));
    plats.add_plat(Plat::new("Poisson grillé"));
    plats.add_plat(Plat::new("Risotto aux champignons"));

    entrees.print();
    plats.print();
}
