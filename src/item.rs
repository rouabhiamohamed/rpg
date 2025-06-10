use serde::{Deserialize, Serialize};

/// Type d'objet dans le jeu
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ItemType {
    Consommable,
    Arme,
    Armure,
    Amulette,
    Cle,
    ObjetDeQuete,
    Autre,
}

/// Représente un objet ramassable dans le jeu
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub value: u32,
    pub item_type: ItemType,
    pub utilisable: bool,
    // Stats fournis par l'équipement
    pub health: i32,
    pub strength: i32,
    pub defense: i32,
    pub agility: i32,
}

impl Item {
    pub fn is_equipable(&self) -> bool {
        matches!(self.item_type, ItemType::Arme | ItemType::Armure | ItemType::Amulette)
    }

    pub fn is_consumable(&self) -> bool {
        matches!(self.item_type, ItemType::Consommable)
    }

    pub fn get_type_name(&self) -> &str {
        match self.item_type {
            ItemType::Arme => "Arme",
            ItemType::Armure => "Armure",
            ItemType::Amulette => "Amulette",
            ItemType::Consommable => "Consommable",
            ItemType::Cle => "Clé",
            ItemType::ObjetDeQuete => "Objet de Quête",
            ItemType::Autre => "Autre",
        }
    }

    pub fn get_stats_description(&self) -> String {
        let mut stats = Vec::new();
        if self.health != 0 {
            stats.push(format!("Santé: {:+}", self.health));
        }
        if self.strength != 0 {
            stats.push(format!("Force: {:+}", self.strength));
        }
        if self.defense != 0 {
            stats.push(format!("Défense: {:+}", self.defense));
        }
        if self.agility != 0 {
            stats.push(format!("Agilité: {:+}", self.agility));
        }

        if stats.is_empty() {
            "Aucun bonus".to_string()
        } else {
            stats.join(", ")
        }
    }
}