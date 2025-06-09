use crate::npc::Npc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Direction {
    North,
    South,
    East,
    West,
}
 

#[derive(Debug, Deserialize)]
pub struct ZoneRaw {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub connections: Vec<String>,
    pub npcs: Vec<u32>,
}

#[derive(Debug)]
pub struct Zone {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub connections: Vec<Direction>,
    pub npcs: Vec<Npc>,
}

impl Zone {
    pub fn afficher(&self) {
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

