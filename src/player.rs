use std::collections::HashMap;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use crate::item::Item;

// Assuming we have an attributes module in the rpg crate
// For now, I'll define a simple Attributes struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    pub health: i32,
    pub strength: i32,
    pub defense: i32,
    pub agility: i32,
}

impl Attributes {
    pub fn new(health: i32, strength: i32, defense: i32, agility: i32) -> Self {
        Attributes { health, strength, defense, agility }
    }

    pub fn apply_delta(&mut self, delta: &Attributes) {
        self.health += delta.health;
        self.strength += delta.strength;
        self.defense += delta.defense;
        self.agility += delta.agility;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub arme: Option<Item>,
    pub armure: Option<Item>,
    pub amulette: Option<Item>,
}

impl Equipment {
    pub fn new() -> Self {
        Equipment {
            arme: None,
            armure: None,
            amulette: None,
        }
    }

    pub fn get_total_stats(&self) -> Attributes {
        let mut total = Attributes::new(0, 0, 0, 0);

        if let Some(ref arme) = self.arme {
            total.health += arme.health;
            total.strength += arme.strength;
            total.defense += arme.defense;
            total.agility += arme.agility;
        }
        if let Some(ref armure) = self.armure {
            total.health += armure.health;
            total.strength += armure.strength;
            total.defense += armure.defense;
            total.agility += armure.agility;
        }
        if let Some(ref amulette) = self.amulette {
            total.health += amulette.health;
            total.strength += amulette.strength;
            total.defense += amulette.defense;
            total.agility += amulette.agility;
        }

        total
    }
}

/// Structure principale du personnage
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub nom: String,
    pub base_stats: Attributes, // Stats de base
    pub current_health: i32,    // Santé actuelle
    pub inventaire: Vec<Item>,  // Changé de Vec<String> à Vec<Item>
    pub equipment: Equipment,   // Équipement
    pub current_zone_id: u32,
    pub monster_kills: HashMap<u32, u32>, // monster_id -> nombre de kills
}

/// Structure pour lire les profils depuis attributes.json
#[derive(Debug, Deserialize)]
struct AttributesProfile {
    profile: String,
    stats: Attributes,
}

impl Player {
    /// Calcule les stats totales (base + équipement)
    pub fn get_total_stats(&self) -> Attributes {
        let mut total = self.base_stats.clone();
        let equipment_bonus = self.equipment.get_total_stats();
        total.apply_delta(&equipment_bonus);
        total
    }

    /// Calcule la santé maximale
    pub fn get_max_health(&self) -> i32 {
        self.get_total_stats().health
    }

    /// Soigne le joueur
    pub fn heal(&mut self, amount: i32) {
        let max_health = self.get_max_health();
        self.current_health = (self.current_health + amount).min(max_health);
    }

    /// Inflige des dégâts au joueur
    pub fn take_damage(&mut self, damage: i32) {
        self.current_health = (self.current_health - damage).max(0);
    }

    /// Vérifie si le joueur est vivant
    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }

    /// Ajoute un kill de monstre
    pub fn add_monster_kill(&mut self, monster_id: u32) {
        *self.monster_kills.entry(monster_id).or_insert(0) += 1;
    }

    /// Obtient le nombre de kills d'un monstre
    pub fn get_monster_kills(&self, monster_id: u32) -> u32 {
        *self.monster_kills.get(&monster_id).unwrap_or(&0)
    }

    /// Crée un nouveau joueur via le processus de création de personnage
    pub fn create_character() -> Result<Player, Box<dyn std::error::Error>> {
        // Étape 1 : Charger les profils de classes
        let profils = Self::charger_profils("data/attributes.json")?;
        let noms_profils: Vec<&str> = profils.keys().map(|k| k.as_str()).collect();

        // Étape 2 : Nom du joueur
        let nom = Self::lire_input("Entrez votre nom de personnage (laisser vide pour 'Inconnu') : ");
        let nom = if nom.is_empty() { "Inconnu".to_string() } else { nom };

        // Étape 3 : Choix du profil
        let profil_choisi = Self::choisir_parmi("Choisissez une classe", &noms_profils);
        let base_stats = profils.get(&profil_choisi).unwrap().clone();

        // Étape 4 : Choix de l'inventaire (objets de départ)
        let items_depart = vec![
            Item {
                id: 1,
                name: "Potion de Soin".to_string(),
                description: "Restaure 30 points de vie".to_string(),
                value: 25,
                item_type: crate::item::ItemType::Consommable,
                utilisable: true,
                health: 30,
                strength: 0,
                defense: 0,
                agility: 0,
            },
            Item {
                id: 2,
                name: "Épée d'Entraînement".to_string(),
                description: "Une épée basique pour débuter".to_string(),
                value: 50,
                item_type: crate::item::ItemType::Arme,
                utilisable: true,
                health: 0,
                strength: 3,
                defense: 0,
                agility: 0,
            },
        ];

        println!("Objets de départ :");
        for item in &items_depart {
            println!("  - {} : {}", item.name, item.description);
        }

        // Étape 5 : Zone de départ (toujours zone 1 pour commencer)
        let current_zone_id = 1;
        let current_health = base_stats.health;

        // Étape 6 : Création du personnage
        let player = Player {
            nom,
            base_stats,
            current_health,
            inventaire: items_depart,
            equipment: Equipment::new(),
            current_zone_id,
            monster_kills: HashMap::new(),
        };

        println!("\n✅ Personnage créé avec succès :");
        println!("Nom: {}", player.nom);
        println!("Stats de base: {:?}", player.base_stats);
        println!("Santé: {}/{}", player.current_health, player.get_max_health());

        // Sauvegarde
        let json = serde_json::to_string_pretty(&player)?;
        std::fs::write("data/perso_save.json", json)?;
        println!("💾 Sauvegarde effectuée dans data/perso_save.json !");

        Ok(player)
    }

    /// Charge un personnage depuis un fichier de sauvegarde
    pub fn load_character(path: &str) -> Result<Player, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let player: Player = serde_json::from_str(&data)?;
        Ok(player)
    }

    /// Sauvegarde le personnage
    pub fn save_character(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Fonction pour charger tous les profils depuis attributes.json
    fn charger_profils(path: &str) -> Result<HashMap<String, Attributes>, Box<dyn std::error::Error>> {
        let contenu = std::fs::read_to_string(path)?;
        let profils: Vec<AttributesProfile> = serde_json::from_str(&contenu)?;

        let mut map = HashMap::new();
        for p in profils {
            map.insert(p.profile, p.stats);
        }
        Ok(map)
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
            let choix = Self::lire_input("Entrez le numéro de votre choix : ");
            if let Ok(index) = choix.parse::<usize>() {
                if index >= 1 && index <= options.len() {
                    return options[index - 1].to_string();
                }
            }
            println!("❌ Choix invalide, réessayez.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::{ItemType, Item};

    fn dummy_item(name: &str, health: i32, strength: i32, defense: i32, agility: i32) -> Item {
        Item {
            id: 0,
            name: name.to_string(),
            description: "".to_string(),
            value: 0,
            item_type: ItemType::Consommable,
            utilisable: false,
            health,
            strength,
            defense,
            agility,
        }
    }

    fn create_test_player() -> Player {
        let base_stats = Attributes::new(50, 10, 5, 3);
        Player {
            nom: "Testeur".to_string(),
            base_stats: base_stats.clone(),
            current_health: base_stats.health,
            inventaire: vec![],
            equipment: Equipment::new(),
            current_zone_id: 1,
            monster_kills: HashMap::new(),
        }
    }

    #[test]
    fn test_total_stats_with_equipment() {
        let mut player = create_test_player();
        player.equipment.arme = Some(dummy_item("Épée", 0, 5, 0, 0));
        player.equipment.armure = Some(dummy_item("Armure", 10, 0, 3, 0));
        player.equipment.amulette = Some(dummy_item("Amulette", 5, 0, 0, 2));

        let total = player.get_total_stats();
        assert_eq!(total.health, 50 + 10 + 5);
        assert_eq!(total.strength, 10 + 5);
        assert_eq!(total.defense, 5 + 3);
        assert_eq!(total.agility, 3 + 2);
    }

    #[test]
    fn test_heal_and_damage() {
        let mut player = create_test_player();
        player.take_damage(30);
        assert_eq!(player.current_health, 20);

        player.heal(5);
        assert_eq!(player.current_health, 25);

        player.heal(50); // Ne doit pas dépasser la santé max
        assert_eq!(player.current_health, player.get_max_health());
    }

    #[test]
    fn test_is_alive_logic() {
        let mut player = create_test_player();
        assert!(player.is_alive());

        player.take_damage(999);
        assert_eq!(player.current_health, 0);
        assert!(!player.is_alive());
    }

    #[test]
    fn test_monster_kills_tracking() {
        let mut player = create_test_player();
        assert_eq!(player.get_monster_kills(42), 0);

        player.add_monster_kill(42);
        assert_eq!(player.get_monster_kills(42), 1);

        player.add_monster_kill(42);
        assert_eq!(player.get_monster_kills(42), 2);
    }
}

