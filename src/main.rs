mod npc;
mod quest;
mod zone;

use npc::{Npc, NpcRaw};
use quest::Quest;
use zone::{Direction, Zone, ZoneRaw};

use std::fs;

fn main() {
    // Chargement des quêtes
    let quests_json = fs::read_to_string("quests.json")
        .expect("Erreur lecture quests.json");
    let quests: Vec<Quest> = serde_json::from_str(&quests_json)
        .expect("Erreur parsing quests.json");

    // Chargement des PNJ
    let npcs_json = fs::read_to_string("npcs.json")
        .expect("Erreur lecture npcs.json");
    let raw_npcs: Vec<NpcRaw> = serde_json::from_str(&npcs_json)
        .expect("Erreur parsing npcs.json");

    let npcs: Vec<Npc> = raw_npcs
        .into_iter()
        .map(|raw| Npc::from_raw(raw, &quests))
        .collect();

    // Chargement des zones
    let zones_json = fs::read_to_string("zones.json")
        .expect("Erreur lecture zones.json");
    let raw_zones: Vec<ZoneRaw> = serde_json::from_str(&zones_json)
        .expect("Erreur parsing zones.json");

    let zones: Vec<Zone> = raw_zones
        .into_iter()
        .map(|rz| {
            let connections = rz.connections.iter().filter_map(|dir| match dir.as_str() {
                "North" => Some(Direction::North),
                "South" => Some(Direction::South),
                "East" => Some(Direction::East),
                "West" => Some(Direction::West),
                _ => None,
            }).collect();

            let zone_npcs: Vec<Npc> = rz.npcs.iter()
                .filter_map(|nid| npcs.iter().find(|n| n.id == *nid).cloned())
                .collect();

            Zone {
                id: rz.id,
                name: rz.name,
                description: rz.description,
                connections,
                npcs: zone_npcs,
            }
        })
        .collect();

    // Affichage de la première zone
    if let Some(zone) = zones.first() {
        zone.afficher();
    }
}
