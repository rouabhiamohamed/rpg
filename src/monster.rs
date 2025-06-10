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