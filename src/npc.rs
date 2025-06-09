use crate::quest::Quest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NpcRaw {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub dialogues: Vec<String>,
    pub quests: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Npc {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub dialogues: Vec<String>,
    pub quests: Vec<Quest>,
}

impl Npc {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn interact(&self) -> String {
        if self.dialogues.is_empty() {
            format!("{} n'a rien à dire.", self.name)
        } else {
            format!("{} dit : '{}'", self.name, &self.dialogues[0])
        }
    }

    pub fn quest_info(&self) -> String {
        if self.quests.is_empty() {
            format!("{} : 'Je n'ai pas de quête pour toi.'", self.name)
        } else {
            format!("{} : 'J'ai une quête : {}'", self.name, self.quests[0].name)
        }
    }

    pub fn from_raw(raw: NpcRaw, all_quests: &[Quest]) -> Self {
        let quests = raw
            .quests
            .iter()
            .filter_map(|qid| all_quests.iter().find(|q| q.id == *qid).cloned())
            .collect();

        Npc {
            id: raw.id,
            name: raw.name,
            description: raw.description,
            dialogues: raw.dialogues,
            quests,
        }
    }
}
