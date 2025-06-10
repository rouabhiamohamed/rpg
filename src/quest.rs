use serde::{Deserialize, Serialize};

/// Représente une quête du jeu
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

    /// Marque la quête comme complétée
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Indique si la quête est complétée
    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_new_without_object() {
        let quest = Quest::new(1, "TestQuest".to_string(), "Une quête de test".to_string(), None);
        assert_eq!(quest.id, 1);
        assert_eq!(quest.name, "TestQuest");
        assert_eq!(quest.description, "Une quête de test");
        assert_eq!(quest.objet_requis_id, None);
        assert!(!quest.completed);
        assert!(!quest.is_completed());
    }

    #[test]
    fn test_new_with_object() {
        let quest = Quest::new(2, "ObjetQuest".to_string(), "Quête avec objet".to_string(), Some(42));
        assert_eq!(quest.id, 2);
        assert_eq!(quest.objet_requis_id, Some(42));
    }

    #[test]
    fn test_complete_method() {
        let mut quest = Quest::new(3, "CompleteQuest".to_string(), "Test complete".to_string(), None);
        assert!(!quest.is_completed());
        quest.complete();
        assert!(quest.is_completed());
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut quest = Quest::new(4, "SerializeQuest".to_string(), "Test sérialisation".to_string(), Some(7));
        quest.complete();
        let json = serde_json::to_string(&quest).expect("Serialization failed");
        let deserialized: Quest = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(quest, deserialized);
    }
}
