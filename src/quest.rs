// src/quest.rs
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub completed: bool,
}
