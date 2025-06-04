// Définition des traits
trait Describable {
    fn name(&self) -> &String;
    fn description(&self) -> &String;
}

trait Interactable {
    fn interact(&self) -> String;
}

trait QuestGiver {
    fn has_quest(&self) -> bool;
    fn quest_info(&self) -> String;
}

// structure simple de quete en attendant la  vrai
struct Quest {
    id: u32,
    name: String,
    description: String,
    completed: bool,
}

impl Quest {
    fn new(id: u32, name: String, description: String) -> Quest {
        Quest {
            id,
            name,
            description,
            completed: false,
        }
    }
}

struct Npc {
    id: u32,
    name: String,
    description: String,
    dialogues: Vec<String>,
    quests: Vec<Quest>,  
}

impl Describable for Npc {
    fn name(&self) -> &String {
        &self.name  
    }

    fn description(&self) -> &String {
        &self.description  
    }
}

impl Interactable for Npc {
    fn interact(&self) -> String {
        if self.dialogues.is_empty() {
            format!("{} n'a rien à dire.", self.name)
        } else {
            //prendre le premier dialogue
            format!("{} dit : '{}'", self.name, &self.dialogues[0])
        }
    }
}

impl QuestGiver for Npc {
    fn has_quest(&self) -> bool {
        !self.quests.is_empty()  // vrai si la liste n'est pas vide
    }

    fn quest_info(&self) -> String {
        if self.quests.is_empty() {
            format!("{} : 'Je n'ai pas de quête pour toi.'", self.name)
        } else {
            // premiere quete dans la liste
            format!("{} : 'J'ai une quête : {}'", self.name, &self.quests[0].name)
        }
    }
}


impl Npc {
    fn new(id: u32, name: String, description: String, dialogues: Vec<String>, quests: Vec<Quest>) -> Npc {
        Npc {
            id,
            name,
            description,
            dialogues,
            quests,
        }
    }
}

fn main() {
    // creation d'une quete
    let quete_marchand = Quest::new(
        1,
        "Trouver des herbes rares".to_string(),
        "Le marchand a besoin d'herbes pour ses potions".to_string()
    );

    //npc avec quete
    let marchand = Npc::new(
        1,
        "Gérard le Marchand".to_string(),
        "Un vieux marchand aux yeux pétillants".to_string(),
        vec![
            "Bienvenue dans ma boutique !".to_string(),
            "J'ai de très bonnes affaires aujourd'hui.".to_string(),
        ],
        vec![quete_marchand]  
    );

    // npc sans quete
    let garde = Npc::new(
        2,
        "Garde Royal".to_string(),
        "Un garde imposant en armure brillante".to_string(),
        vec![], // pas de dialogue
        vec![]  // pas de quete
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

    println!("\n=== Quêtes ===");
    println!("Marchand a une quête ? {}", marchand.has_quest());
    println!("{}", marchand.quest_info());
    println!();
    println!("Garde a une quête ? {}", garde.has_quest());
    println!("{}", garde.quest_info());

    println!("\n=== Test d'emprunts multiples ===");
    let nom_marchand = marchand.name();
    let desc_marchand = marchand.description();
    println!("Le {} : {}", nom_marchand, desc_marchand);
    println!("{}", marchand.quest_info());
}