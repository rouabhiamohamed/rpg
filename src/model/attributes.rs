//! Module `attributes`
//!
//! Représente les statistiques principales d'un personnage ou d'un objet dans le jeu.
//! Permet d'appliquer des modifications positives (bonus) ou négatives (malus) sur ces stats.

use serde::{Deserialize, Serialize};

/// Structure des attributs d'un personnage (ou d'un effet).

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    pub health: i32,        // Points de vie
    pub strength: i32,      // Force physique
    pub dexterity: i32,     // Agilité / précision
    pub intelligence: i32,  // Capacité mentale / magie
}

impl Attributes {
    /// Crée un nouvel ensemble d'attributs avec des valeurs données.
    pub fn new(health: i32, strength: i32, dexterity: i32, intelligence: i32) -> Self {
        Self {
            health,
            strength,
            dexterity,
            intelligence,
        }
    }

    /// Applique un ensemble de modifications (`delta`) sur les attributs actuels.
    ///
    /// - Si `delta` contient des valeurs positives : effet bénéfique (bonus)
    /// - Si `delta` contient des valeurs négatives : effet néfaste (malus)
    ///
    pub fn apply_delta(&mut self, delta: &Attributes) {
        self.health += delta.health;
        self.strength += delta.strength;
        self.dexterity += delta.dexterity;
        self.intelligence += delta.intelligence;
    }
}

