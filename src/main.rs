use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
struct Quest {
    id: u32,
    name: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct NpcRaw {
    id: u32,
    name: String,
    description: String,
    dialogues: Vec<String>,
    quests: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Npc {
    id: u32,
    name: String,
    description: String,
    dialogues: Vec<String>,
    quests: Vec<Quest>,
}

impl Npc {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn interact(&self) -> String {
        if self.dialogues.is_empty() {
            format!("{} n'a rien à dire.", self.name)
        } else {
            format!("{} dit : '{}'", self.name, &self.dialogues[0])
        }
    }

    fn has_quest(&self) -> bool {
        !self.quests.is_empty()
    }

    fn quest_info(&self) -> String {
        if self.quests.is_empty() {
            format!("{} : 'Je n'ai pas de quête pour toi.'", self.name)
        } else {
            format!("{} : 'J'ai une quête : {}'", self.name, self.quests[0].name)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Deserialize)]
struct ZoneRaw {
    id: u32,
    name: String,
    description: String,
    connections: Vec<String>,
    npcs: Vec<u32>,
}

#[derive(Debug)]
struct Zone {
    id: u32,
    name: String,
    description: String,
    connections: Vec<Direction>,
    npcs: Vec<Npc>,
}

impl Zone {
    fn afficher(&self) {
        println!("== Zone [{}] : {} ==", self.id, self.name);
        println!("{}", self.description);

        if self.connections.is_empty() {
            println!("Pas de sorties.");
        } else {
            println!("Sorties disponibles :");
            for dir in &self.connections {
                println!("- {:?}", dir);
            }
        }

        if self.npcs.is_empty() {
            println!("Personne dans cette zone.");
        } else {
            println!("PNJs présents :");
            for npc in &self.npcs {
                println!("-> {} : {}", npc.name(), npc.description());
                println!("   {}", npc.interact());
                println!("   {}", npc.quest_info());
            }
        }
    }
}

fn main() {
    let quests_json = fs::read_to_string("quests.json").expect("Erreur lecture quests.json");
    let npcs_json = fs::read_to_string("npcs.json").expect("Erreur lecture npcs.json");
    let zones_json = fs::read_to_string("zones.json").expect("Erreur lecture zones.json");

    let all_quests: Vec<Quest> = serde_json::from_str(&quests_json).unwrap();
    let npc_raw: Vec<NpcRaw> = serde_json::from_str(&npcs_json).unwrap();
    let zone_raw: Vec<ZoneRaw> = serde_json::from_str(&zones_json).unwrap();

    let npcs: Vec<Npc> = npc_raw
        .into_iter()
        .map(|n| {
            let quests = n
                .quests
                .iter()
                .filter_map(|qid| all_quests.iter().find(|q| q.id == *qid).cloned())
                .collect();
            Npc {
                id: n.id,
                name: n.name,
                description: n.description,
                dialogues: n.dialogues,
                quests,
            }
        })
        .collect();

    let zones: Vec<Zone> = zone_raw
        .into_iter()
        .map(|z| {
            let zone_npcs = z
                .npcs
                .iter()
                .filter_map(|nid| npcs.iter().find(|n| n.id == *nid).cloned())
                .collect();
            let connections = z
                .connections
                .iter()
                .filter_map(|d| match d.as_str() {
                    "North" => Some(Direction::North),
                    "South" => Some(Direction::South),
                    "East" => Some(Direction::East),
                    "West" => Some(Direction::West),
                    _ => None,
                })
                .collect();
            Zone {
                id: z.id,
                name: z.name,
                description: z.description,
                connections,
                npcs: zone_npcs,
            }
        })
        .collect();

    if let Some(zone) = zones.first() {
        zone.afficher();
    }
}
