mod item;
mod quest;

use item::{Item, ItemType};
use quest::Quest;

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
            // on prend toujours le premier dialogue, par souci de simplicité
            format!("{} dit : '{}'", self.name, &self.dialogues[0])
        }
    }
}

impl QuestGiver for Npc {
    fn has_quest(&self) -> bool {
        !self.quests.is_empty()
    }
    fn quest_info(&self) -> String {
        if self.quests.is_empty() {
            format!("{} : 'Je n'ai pas de quête pour toi.'", self.name)
        } else {
            // on affiche la première quête disponible
            format!("{} : 'J'ai une quête : {}'", self.name, &self.quests[0].name)
        }
    }
}

impl Npc {
    fn new(
        id: u32,
        name: String,
        description: String,
        dialogues: Vec<String>,
        quests: Vec<Quest>,
    ) -> Npc {
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
    // --- Création d'une quête "marchand" (sans objet requis) ---
    let quete_marchand = Quest::new(
        1,
        "Trouver des herbes rares".to_string(),
        "Le marchand a besoin d'herbes pour ses potions".to_string(),
        None, // pas d'objet précis à ramener
    );

    // --- NPC "marchand" avec une quête ---
    let marchand = Npc::new(
        1,
        "Gérard le Marchand".to_string(),
        "Un vieux marchand aux yeux pétillants".to_string(),
        vec![
            "Bienvenue dans ma boutique !".to_string(),
            "J'ai de très bonnes affaires aujourd'hui.".to_string(),
        ],
        vec![quete_marchand],
    );

    // --- NPC "garde" sans quête ni dialogue ---
    let garde = Npc::new(
        2,
        "Garde Royal".to_string(),
        "Un garde imposant en armure brillante".to_string(),
        vec![],
        vec![],
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

    /////////////////////////////////////////////////////////////////////////////
    // --- Exemple d’Item et d’une quête liée à un objet ---
    let potion = Item {
        id: 1,
        nom: "Potion de soin".to_string(),
        description: "Rend 50 PV".to_string(),
        item_type: ItemType::Consommable,
        utilisable: true,
    };

    // Ici, on crée une quête qui demande spécifiquement l'objet d'id = 1
    let quete_objet = Quest::new(
        2,
        "Trouver la potion".to_string(),
        "Donner une potion de soin au villageois.".to_string(),
        Some(1), // l'ID de l'objet "Potion de soin"
    );

    // On peut afficher les deux structures grâce au trait Debug
    println!("{:?}\n{:?}", potion, quete_objet);
}
