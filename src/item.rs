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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(
        item_type: ItemType,
        health: i32,
        strength: i32,
        defense: i32,
        agility: i32,
    ) -> Item {
        Item {
            id: 1,
            name: "Test Item".to_string(),
            description: "Un objet pour test".to_string(),
            value: 100,
            item_type,
            utilisable: true,
            health,
            strength,
            defense,
            agility,
        }
    }

    #[test]
    fn test_is_equipable() {
        assert!(make_item(ItemType::Arme, 0, 0, 0, 0).is_equipable());
        assert!(make_item(ItemType::Armure, 0, 0, 0, 0).is_equipable());
        assert!(make_item(ItemType::Amulette, 0, 0, 0, 0).is_equipable());
        assert!(!make_item(ItemType::Consommable, 0, 0, 0, 0).is_equipable());
        assert!(!make_item(ItemType::Cle, 0, 0, 0, 0).is_equipable());
    }

    #[test]
    fn test_is_consumable() {
        assert!(make_item(ItemType::Consommable, 0, 0, 0, 0).is_consumable());
        assert!(!make_item(ItemType::Arme, 0, 0, 0, 0).is_consumable());
    }

    #[test]
    fn test_get_type_name() {
        let item = make_item(ItemType::ObjetDeQuete, 0, 0, 0, 0);
        assert_eq!(item.get_type_name(), "Objet de Quête");

        let item2 = make_item(ItemType::Amulette, 0, 0, 0, 0);
        assert_eq!(item2.get_type_name(), "Amulette");
    }

    #[test]
    fn test_get_stats_description_none() {
        let item = make_item(ItemType::Arme, 0, 0, 0, 0);
        assert_eq!(item.get_stats_description(), "Aucun bonus");
    }

    #[test]
    fn test_get_stats_description_some() {
        let item = make_item(ItemType::Arme, 5, 0, -2, 3);
        let desc = item.get_stats_description();
        assert!(desc.contains("Santé: +5"));
        assert!(desc.contains("Défense: -2"));
        assert!(desc.contains("Agilité: +3"));
        assert!(!desc.contains("Force")); // car force = 0
    }
}

