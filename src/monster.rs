use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct MonsterRaw {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub health: i32,
    pub strength: i32,
    pub defense: i32,
    pub agility: i32,
    pub loot: Vec<u32>, // IDs des objets droppés
    pub experience: u32, // XP donnée quand vaincu
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub max_health: i32,
    pub current_health: i32,
    pub strength: i32,
    pub defense: i32,
    pub agility: i32,
    pub loot: Vec<u32>,
    pub experience: u32,
}

impl Monster {
    pub fn from_raw(raw: MonsterRaw) -> Self {
        Monster {
            id: raw.id,
            name: raw.name.clone(),
            description: raw.description,
            max_health: raw.health,
            current_health: raw.health,
            strength: raw.strength,
            defense: raw.defense,
            agility: raw.agility,
            loot: raw.loot,
            experience: raw.experience,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.current_health = (self.current_health - damage).max(0);
    }

    pub fn health_percentage(&self) -> f32 {
        if self.max_health == 0 {
            0.0
        } else {
            (self.current_health as f32 / self.max_health as f32) * 100.0
        }
    }

    pub fn health_bar(&self) -> String {
        let percentage = self.health_percentage();
        let bar_length = 20;
        let filled = ((percentage / 100.0) * bar_length as f32) as usize;
        let empty = bar_length - filled;

        let bar = format!(
            "[{}{}]",
            "█".repeat(filled),
            "░".repeat(empty)
        );

        format!("{} {}/{} HP", bar, self.current_health, self.max_health)
    }
}

#[derive(Debug)]
pub enum CombatResult {
    PlayerWins(Vec<u32>), // Loot obtenu
    PlayerLoses,
    Ongoing,
}

#[derive(Debug)]
pub enum AttackResult {
    Hit(i32),      // Dégâts infligés
    Dodge,         // Attaque esquivée
    Miss,          // Attaque ratée
}

pub fn calculate_damage(attacker_strength: i32, defender_defense: i32) -> i32 {
    (attacker_strength - defender_defense).max(1)
}

pub fn check_dodge(agility: i32) -> bool {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    // Simple RNG basé sur le temps et l'agilité
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    agility.hash(&mut hasher);
    let random = hasher.finish() % 100;

    // Chance d'esquive : agility * 2% (max 30%)
    let dodge_chance = (agility * 2).min(30) as u64;
    random < dodge_chance
}

pub fn calculate_hit_chance() -> bool {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    let random = hasher.finish() % 100;

    // 90% de chance de toucher
    random < 90
}
#[cfg(test)]
mod tests {
    use super::*;

    fn creer_monstre_test() -> Monster {
        Monster {
            id: 1,
            name: "Squelette".to_string(),
            description: "Un squelette animé".to_string(),
            max_health: 80,
            current_health: 80,
            strength: 15,
            defense: 8,
            agility: 5,
            loot: vec![10, 11],
            experience: 45,
        }
    }

    #[test]
    fn test_creation_depuis_raw() {
        let raw = MonsterRaw {
            id: 2,
            name: "Orc Guerrier".to_string(),
            description: "Un orc massif avec une hache".to_string(),
            health: 120,
            strength: 18,
            defense: 10,
            agility: 3,
            loot: vec![15, 16, 17],
            experience: 60,
        };

        let monstre = Monster::from_raw(raw);
        assert_eq!(monstre.name, "Orc Guerrier");
        assert_eq!(monstre.max_health, 120);
        assert_eq!(monstre.current_health, 120);
        assert_eq!(monstre.strength, 18);
        assert_eq!(monstre.loot.len(), 3);
    }

    #[test]
    fn test_systeme_vie() {
        let mut monstre = creer_monstre_test();

        assert!(monstre.is_alive());
        assert_eq!(monstre.health_percentage(), 100.0);

        monstre.take_damage(30);
        assert_eq!(monstre.current_health, 50);
        assert_eq!(monstre.health_percentage(), 62.5); // 50/80 * 100
        assert!(monstre.is_alive());

        monstre.take_damage(100); // Plus que la santé restante
        assert_eq!(monstre.current_health, 0);
        assert!(!monstre.is_alive());
    }

    #[test]
    fn test_barre_de_vie() {
        let mut monstre = creer_monstre_test();

        let barre_pleine = monstre.health_bar();
        assert!(barre_pleine.contains("█"));
        assert!(barre_pleine.contains("80/80 HP"));

        monstre.current_health = 40; // 50%
        let barre_moitie = monstre.health_bar();
        assert!(barre_moitie.contains("█"));
        assert!(barre_moitie.contains("░"));
        assert!(barre_moitie.contains("40/80 HP"));

        monstre.current_health = 0;
        let barre_vide = monstre.health_bar();
        assert!(barre_vide.contains("░"));
        assert!(barre_vide.contains("0/80 HP"));
    }

    #[test]
    fn test_calcul_degats() {
        assert_eq!(calculate_damage(20, 5), 15);    // Attaque normale
        assert_eq!(calculate_damage(10, 12), 1);    // Défense supérieure → dégâts min
        assert_eq!(calculate_damage(25, 25), 1);    // Force égale défense → dégâts min
    }

    #[test]
    fn test_systeme_esquive() {
        // Test avec agilité 0 - aucune esquive
        let mut esquives = 0;
        for _ in 0..50 {
            if check_dodge(0) {
                esquives += 1;
            }
        }
        assert_eq!(esquives, 0);

        // Test avec agilité très élevée
        let mut esquives_elevees = 0;
        for _ in 0..200 {
            if check_dodge(20) { // 20 * 2 = 40%, mais plafonné à 30%
                esquives_elevees += 1;
            }
        }
        // Environ 30% d'esquives attendues
        assert!(esquives_elevees > 40 && esquives_elevees < 80);
    }

    #[test]
    fn test_precision_attaque() {
        let mut touches = 0;
        for _ in 0..100 {
            if calculate_hit_chance() {
                touches += 1;
            }
        }
        // Environ 90% de touches attendues
        assert!(touches > 80 && touches <= 100);
    }
}