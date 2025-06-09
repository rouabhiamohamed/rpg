use crate::{item::Item, quest::Quest};
use serde_json;               // 📌 Ajouté
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
