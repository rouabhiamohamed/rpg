use serde::{Deserialize, Serialize};

/// Type d'objet dans le jeu
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ItemType {
    Consommable,
    Arme,
    Armure,
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
}
