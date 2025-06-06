#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    Consommable,
    Arme,
    Armure,
    Cle,
    ObjetDeQuete,
    Autre,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: u32,
    pub nom: String,
    pub description: String,
    pub item_type: ItemType,
    pub utilisable: bool,
}
