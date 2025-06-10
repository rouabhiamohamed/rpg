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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::npc::Npc;
    use crate::monster::Monster;
    use crate::quest::Quest;

    #[test]
    fn test_directions_francaises() {
        let raw = ZoneRaw {
            id: 1,
            name: "Place du Village".to_string(),
            description: "La place centrale du village".to_string(),
            connections: vec!["nord".to_string(), "sud".to_string(), "est".to_string(), "ouest".to_string()],
            npcs: vec![],
            monsters: None,
        };

        let zone = Zone::from_raw(raw, &[], &[]);
        assert_eq!(zone.connections.len(), 4);
        assert!(matches!(zone.connections[0], Direction::North));
        assert!(matches!(zone.connections[1], Direction::South));
        assert!(matches!(zone.connections[2], Direction::East));
        assert!(matches!(zone.connections[3], Direction::West));
    }

    #[test]
    fn test_directions_invalides_ignorees() {
        let raw = ZoneRaw {
            id: 2,
            name: "Zone Corrompue".to_string(),
            description: "Données partiellement corrompues".to_string(),
            connections: vec!["nord".to_string(), "direction_invalide".to_string(), "sud".to_string()],
            npcs: vec![],
            monsters: None,
        };

        let zone = Zone::from_raw(raw, &[], &[]);
        assert_eq!(zone.connections.len(), 2); // Seuls "nord" et "sud" sont valides
    }

    #[test]
    fn test_zone_avec_entites() {
        let quete = Quest {
            id: 1,
            name: "Sauver le Village".to_string(),
            description: "Éliminez les monstres qui menacent le village".to_string(),
            objet_requis_id: None,
            completed: false,
        };

        let pnj = Npc {
            id: 1,
            name: "Maire du Village".to_string(),
            description: "Le dirigeant du village".to_string(),
            dialogues: vec!["Aidez-nous, brave aventurier !".to_string()],
            quests: vec![quete],
        };

        let monstre = Monster {
            id: 1,
            name: "Loup Enragé".to_string(),
            description: "Un loup aux yeux rouges".to_string(),
            max_health: 40,
            current_health: 40,
            strength: 12,
            defense: 3,
            agility: 10,
            loot: vec![5],
            experience: 20,
        };

        let raw = ZoneRaw {
            id: 3,
            name: "Lisière de la Forêt".to_string(),
            description: "Où le village rencontre la nature sauvage".to_string(),
            connections: vec!["nord".to_string(), "ouest".to_string()],
            npcs: vec![1],
            monsters: Some(vec![1]),
        };

        let zone = Zone::from_raw(raw, &[pnj], &[monstre]);
        assert_eq!(zone.npcs.len(), 1);
        assert_eq!(zone.monsters.len(), 1);
        assert_eq!(zone.npcs[0].name, "Maire du Village");
        assert_eq!(zone.monsters[0].name, "Loup Enragé");
    }

    #[test]
    fn test_entites_manquantes() {
        let raw = ZoneRaw {
            id: 4,
            name: "Zone Vide".to_string(),
            description: "Aucune entité trouvée".to_string(),
            connections: vec!["est".to_string()],
            npcs: vec![999], // ID inexistant
            monsters: Some(vec![888]), // ID inexistant
        };

        let zone = Zone::from_raw(raw, &[], &[]);
        assert_eq!(zone.npcs.len(), 0);
        assert_eq!(zone.monsters.len(), 0);
    }
}