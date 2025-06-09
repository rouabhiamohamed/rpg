mod item;
mod quest;
mod data_loader;

use std::error::Error;
use item::Item;
use quest::Quest;
use data_loader::{load_items, load_quests};

// Traits pour le comportement
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

// Définition d'un NPC
struct Npc {
    id: u32,
    name: String,
    description: String,
    dialogues: Vec<String>,
    quests: Vec<Quest>,
}

impl Describable for Npc {
    fn name(&self) -> &String { &self.name }
    fn description(&self) -> &String { &self.description }
}

impl Interactable for Npc {
    fn interact(&self) -> String {
        if let Some(line) = self.dialogues.first() {
            format!("{} dit : '{}'", self.name, line)
        } else {
            format!("{} n'a rien à dire.", self.name)
        }
    }
}

impl QuestGiver for Npc {
    fn has_quest(&self) -> bool { !self.quests.is_empty() }
    fn quest_info(&self) -> String {
        if let Some(q) = self.quests.first() {
            format!("{} : 'J'ai une quête : {}'", self.name, q.name)
        } else {
            format!("{} : 'Je n'ai pas de quête pour toi.'", self.name)
        }
    }
}

impl Npc {
    pub fn new(
        id: u32,
        name: String,
        description: String,
        dialogues: Vec<String>,
        quests: Vec<Quest>,
    ) -> Self {
        Npc { id, name, description, dialogues, quests }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1) Charger les données
    let items = load_items("data/items.json")?;
    let mut quests = load_quests("data/quests.json")?;

    // 2) Construire la première quête pour le marchand
    let merchant_quests = if !quests.is_empty() {
        vec![quests.remove(0)]
    } else {
        Vec::new()
    };

    // 3) Créer et utiliser un NPC « marchand »
    let marchand = Npc::new(
        1,
        "Gérard le Marchand".to_string(),
        "Un vieux marchand aux yeux pétillants".to_string(),
        vec![
            "Bienvenue dans ma boutique !".to_string(),
            "J'ai de très bonnes affaires aujourd'hui !".to_string(),
        ],
        merchant_quests,
    );

    println!("=== Interaction NPC ===");
    println!("{}", marchand.interact());
    println!("A une quête ? {}", marchand.has_quest());
    println!("{}", marchand.quest_info());

    // 4) Afficher le reste des données chargées
    println!("\n=== Données chargées depuis JSON ===");
    println!("{} items chargés.", items.len());
    for item in &items {
        println!(
            "- {} (id={}): {} [valeur={}, type={:?}, utilisable={}]",
            item.name, item.id, item.description, item.value, item.item_type, item.utilisable
        );
    }

    println!("{} quêtes restantes.", quests.len());
    for q in &quests {
        println!(
            "- {} (id={}): {} [Objet requis: {:?}, terminé: {}]",
            q.name, q.id, q.description, q.objet_requis_id, q.completed
        );
    }

    Ok(())
}
