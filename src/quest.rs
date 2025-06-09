use serde::{Deserialize, Serialize};

/// Représente une quête du jeu
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quest {
    /// Identifiant unique de la quête
    pub id: u32,
    /// Titre de la quête
    pub name: String,
    /// Description détaillée
    pub description: String,
    /// Si la quête requiert un objet spécifique (par son ID)
    pub objet_requis_id: Option<u32>,
    /// Statut d'achèvement de la quête
    pub completed: bool,
}

impl Quest {
    /// Crée une nouvelle quête (non complétée)
    /// Passez `None` si aucun objet n'est requis.
    pub fn new(
        id: u32,
        name: String,
        description: String,
        objet_requis_id: Option<u32>,
    ) -> Self {
        Quest {
            id,
            name,
            description,
            objet_requis_id,
            completed: false,
        }
    }
}
