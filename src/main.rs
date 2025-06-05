use std::collections::HashMap;
use std::io::{self, Write};
use serde::Deserialize;
use serde::Serialize;

extern crate rpg; 

use rpg::model::attributes::Attributes;

/*

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
*/



/// Structure principale du personnage, conforme au diagramme de classe
#[derive(Debug, Serialize)]
struct Perso {
    nom: String,
    stats: Attributes,
    inventaire: Vec<String>,
    current_zone: String,
}

/// Structure pour lire les profils depuis attributes.json
#[derive(Debug, Deserialize)]
struct AttributesProfile {
    profile: String,
    stats: Attributes,
}

/// Fonction pour charger tous les profils depuis attributes.json
fn charger_profils(path: &str) -> HashMap<String, Attributes> {
    let contenu = std::fs::read_to_string(path)
        .expect("Erreur lors de la lecture du fichier attributes.json");

    let profils: Vec<AttributesProfile> = serde_json::from_str(&contenu)
        .expect("Erreur lors du parsing JSON");

    let mut map = HashMap::new();
    for p in profils {
        map.insert(p.profile, p.stats);
    }
    map
}

/// Lit une ligne depuis la console (avec message)
fn lire_input(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Menu pour sélectionner une valeur dans une liste
fn choisir_parmi(label: &str, options: &[&str]) -> String {
    println!("{} :", label);
    for (i, option) in options.iter().enumerate() {
        println!("  [{}] {}", i + 1, option);
    }

    loop {
        let choix = lire_input("Entrez le numéro de votre choix : ");
        if let Ok(index) = choix.parse::<usize>() {
            if index >= 1 && index <= options.len() {
                return options[index - 1].to_string();
            }
        }
        println!("❌ Choix invalide, réessayez.");
    }
}




fn main() {
/*
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
    
    println!("\n=== Test avec les attributs ===");
    
    let mut stats = Attributes::new(100, 10, 5, 7);
    let bonus = Attributes::new(10, 2, 0, 1);
    let malus = Attributes::new(-20, -3, -1, 0);

    println!("Avant modification: {:?}", stats);
    stats.apply_delta(&bonus);
    println!("Après bonus: {:?}", stats);
    stats.apply_delta(&malus);
    println!("Après malus: {:?}", stats);
    
    println!("\n ------------------------- ");
    */
    
    
    
    
    // Étape 1 : Charger les profils de classes
    let profils = charger_profils("data/attributes.json");
    let noms_profils: Vec<&str> = profils.keys().map(|k| k.as_str()).collect();

    // Étape 2 : Nom du joueur
    let nom = lire_input("Entrez votre nom de personnage (laisser vide pour 'Inconnu') : ");
    let nom = if nom.is_empty() { "Inconnu".to_string() } else { nom };

    // Étape 3 : Choix du profil
    let profil_choisi = choisir_parmi("Choisissez une classe", &noms_profils);
    let stats = profils.get(&profil_choisi).unwrap().clone();

    // Étape 4 : Choix de l'inventaire (items fictifs)
    let items_disponibles = vec!["potion_01", "arc_long", "épée_fer", "amulette_vent"];
    println!("Sélectionnez des objets (tapez les numéros séparés par des virgules, ex: 1,3) :");
    for (i, item) in items_disponibles.iter().enumerate() {
        println!("  [{}] {}", i + 1, item);
    }

    let mut inventaire = vec![];
    let choix_items = lire_input("Objets : ");
    for s in choix_items.split(',') {
        if let Ok(idx) = s.trim().parse::<usize>() {
            if idx >= 1 && idx <= items_disponibles.len() {
                inventaire.push(items_disponibles[idx - 1].to_string());
            }
        }
    }

    // Étape 5 : Choix de la zone
    let zones_disponibles = vec!["zone_foret_nord", "zone_montagne_ouest", "zone_tour_arcane"];
    let current_zone = choisir_parmi("Choisissez une zone de départ", &zones_disponibles);

    // Étape 6 : Création du personnage
    let perso = Perso {
        nom,
        stats,
        inventaire,
        current_zone,
    };

    println!("\n✅ Personnage créé avec succès :\n{:#?}", perso);

    
    
    // À la fin du main()
    let json = serde_json::to_string_pretty(&perso).unwrap();
    std::fs::write("data/perso_save.json", json).unwrap();
    println!("💾 Sauvegarde effectuée dans data/perso_save.json !");

    
    
    
}
