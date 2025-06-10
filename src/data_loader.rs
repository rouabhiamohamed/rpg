use crate::{item::Item, quest::Quest, npc::{Npc, NpcRaw}, zone::{Zone, ZoneRaw}, monster::{Monster, MonsterRaw}};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn load_items(path: &str) -> Result<Vec<Item>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let items = serde_json::from_str(&data)?;
    Ok(items)
}

pub fn load_quests(path: &str) -> Result<Vec<Quest>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let quests = serde_json::from_str(&data)?;
    Ok(quests)
}

pub fn load_npcs(path: &str, all_quests: &[Quest]) -> Result<Vec<Npc>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let npcs_raw: Vec<NpcRaw> = serde_json::from_str(&data)?;

    let npcs = npcs_raw
        .into_iter()
        .map(|raw| Npc::from_raw(raw, all_quests))
        .collect();

    Ok(npcs)
}

pub fn load_monsters(path: &str) -> Result<Vec<Monster>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let monsters_raw: Vec<MonsterRaw> = serde_json::from_str(&data)?;

    let monsters = monsters_raw
        .into_iter()
        .map(|raw| Monster::from_raw(raw))
        .collect();

    Ok(monsters)
}

pub fn load_zones(path: &str, all_npcs: &[Npc], all_monsters: &[Monster]) -> Result<HashMap<u32, Zone>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let zones_raw: Vec<ZoneRaw> = serde_json::from_str(&data)?;

    let mut zones = HashMap::new();
    for raw in zones_raw {
        let zone_id = raw.id;
        let zone = Zone::from_raw(raw, all_npcs, all_monsters);
        zones.insert(zone_id, zone);
    }

    Ok(zones)
}