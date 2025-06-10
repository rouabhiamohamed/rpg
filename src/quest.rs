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
        Quest { id, name, description, objet_requis_id, completed: false }
    }

    /// Marque la quête comme complétée
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Réinitialise la quête à non complétée
    pub fn reset(&mut self) {
        self.completed = false;
    }

    /// Indique si la quête est complétée
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Indique si un objet est requis pour cette quête
    pub fn requires_object(&self) -> bool {
        self.objet_requis_id.is_some()
    }

    /// Récupère l'ID de l'objet requis ou erreur si aucun
    pub fn required_object_id(&self) -> Option<u32> {
        self.objet_requis_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_new_without_object() {
        let quest = Quest::new(1, "TestQuest".into(), "Une quête de test".into(), None);
        assert_eq!(quest.id, 1);
        assert_eq!(quest.name, "TestQuest");
        assert_eq!(quest.description, "Une quête de test");
        assert_eq!(quest.objet_requis_id, None);
        assert!(!quest.is_completed());
        assert!(!quest.requires_object());
    }

    #[test]
    fn test_new_with_object() {
        let quest = Quest::new(2, "ObjetQuest".into(), "Quête avec objet".into(), Some(42));
        assert_eq!(quest.id, 2);
        assert_eq!(quest.objet_requis_id, Some(42));
        assert!(quest.requires_object());
        assert_eq!(quest.required_object_id(), Some(42));
    }

    #[test]
    fn test_complete_and_reset() {
        let mut quest = Quest::new(3, "StateQuest".into(), "Test état".into(), None);
        assert!(!quest.is_completed());
        quest.complete();
        assert!(quest.is_completed());
        quest.reset();
        assert!(!quest.is_completed());
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut quest = Quest::new(4, "SerializeQuest".into(), "Test sérialisation".into(), Some(7));
        quest.complete();
        let json = serde_json::to_string(&quest).expect("Serialization failed");
        let deserialized: Quest = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(quest, deserialized);
    }
}
