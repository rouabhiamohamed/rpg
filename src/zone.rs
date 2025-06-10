use crate::npc::Npc;
use crate::monster::Monster;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ZoneRaw {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub connections: Vec<String>, // Les directions seront converties
    pub npcs: Vec<u32>, // IDs des NPCs
    pub monsters: Option<Vec<u32>>, // IDs des monstres (optionnel)
}

#[derive(Debug, Clone)]
pub struct Zone {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub connections: Vec<Direction>,
    pub npcs: Vec<Npc>,
    pub monsters: Vec<Monster>,
}

impl Zone {
    pub fn from_raw(raw: ZoneRaw, all_npcs: &[Npc], all_monsters: &[Monster]) -> Self {
        // Convertir les strings en directions
        let connections = raw.connections
            .iter()
            .filter_map(|s| match s.to_lowercase().as_str() {
                "north" | "nord" => Some(Direction::North),
                "south" | "sud" => Some(Direction::South),
                "east" | "est" => Some(Direction::East),
                "west" | "ouest" => Some(Direction::West),
                _ => None,
            })
            .collect();

        // Récupérer les NPCs correspondants
        let npcs = raw.npcs
            .iter()
            .filter_map(|npc_id| all_npcs.iter().find(|npc| npc.id == *npc_id).cloned())
            .collect();

        // Récupérer les monstres correspondants
        let monsters = raw.monsters
            .unwrap_or_default()
            .iter()
            .filter_map(|monster_id| all_monsters.iter().find(|monster| monster.id == *monster_id).cloned())
            .collect();

        Zone {
            id: raw.id,
            name: raw.name,
            description: raw.description,
            connections,
            npcs,
            monsters,
        }
    }

    pub fn afficher(&self) {
        println!("🗺️  Zone [{}] : {}", self.id, self.name);
        println!("📍 {}", self.description);

        if self.connections.is_empty() {
            println!("🚫 Aucune sortie disponible.");
        } else {
            println!("🧭 Sorties disponibles :");
            for dir in &self.connections {
                let emoji = match dir {
                    Direction::North => "⬆️",
                    Direction::South => "⬇️",
                    Direction::East => "➡️",
                    Direction::West => "⬅️",
                };
                println!("   {} {:?}", emoji, dir);
            }
        }

        if self.npcs.is_empty() {
            println!("👤 Aucun personnage dans cette zone.");
        } else {
            println!("👥 Personnages présents :");
            for npc in &self.npcs {
                println!("   🧙 {} : {}", npc.name(), npc.description());
            }
        }

        if self.monsters.is_empty() {
            println!("🕊️  Aucun monstre dans cette zone.");
        } else {
            println!("⚔️  Monstres présents :");
            for monster in &self.monsters {
                println!("   👹 {} : {}", monster.name, monster.description);
                if monster.is_alive() {
                    println!("     {}", monster.health_bar());
                } else {
                    println!("     💀 Vaincu");
                }
            }
        }
    }
}