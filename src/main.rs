trait Describable {
    fn name(&self) -> &String;
    fn description(&self) -> &String;
}

trait Interactable {
    fn interact(&self) -> String;
}

// Structure Npc
struct Npc {
    id: u32,
    name: String,
    description: String,
    dialogues: Vec<String>,
}

impl Describable for Npc {
    fn name(&self) -> &String {
        &self.name  // Emprunt, pas de copie
    }

    fn description(&self) -> &String {
        &self.description  // Emprunt
    }
}

impl Interactable for Npc {
    fn interact(&self) -> String {
        if self.dialogues.is_empty() {
            format!("{} n'a rien à dire.", self.name)
        } else {
            // prendre le premier dialogue pour simplifier
            format!("{} dit : '{}'", self.name, &self.dialogues[0])
        }
    }
}

impl Npc {
    fn new(id: u32, name: String, description: String, dialogues: Vec<String>) -> Npc {
        Npc {
            id,
            name,
            description,
            dialogues,
        }
    }
}

fn main() {
    let marchand = Npc::new(
        1,
        "Gérard le Marchand".to_string(),
        "Un vieux marchand aux yeux pétillants".to_string(),
        vec![
            "Bienvenue dans ma boutique !".to_string(),
            "J'ai de très bonnes affaires aujourd'hui.".to_string(),
        ]
    );

    let garde = Npc::new(
        2,
        "Garde Royal".to_string(),
        "Un garde imposant en armure brillante".to_string(),
        vec![]  
    );

    println!("=== Informations des NPCs ===");
    println!("Nom: {}", marchand.name());
    println!("Description: {}", marchand.description());
    println!();
    println!("Nom: {}", garde.name());
    println!("Description: {}", garde.description());

    println!("\n=== Interactions ===");
    println!("{}", marchand.interact());
    println!("{}", garde.interact());

    println!("\n=== Test d'emprunts multiples ===");
    let nom_marchand = marchand.name();
    let desc_marchand = marchand.description();
    println!("Le {} : {}", nom_marchand, desc_marchand);
    println!("{}", marchand.interact());
}