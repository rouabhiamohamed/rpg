use crate::quest::Quest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NpcRaw {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub dialogues: Vec<String>,
    pub quests: Vec<u32>, // IDs des quêtes
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::quest::Quest;

    fn creer_quete_test() -> Quest {
        Quest {
            id: 1,
            name: "Retrouver l'Épée Perdue".to_string(),
            description: "Une épée légendaire a été perdue dans les profondeurs".to_string(),
            objet_requis_id: Some(42),
            completed: false,
        }
    }

    #[test]
    fn test_pnj_avec_dialogue() {
        let quete = creer_quete_test();
        let pnj = Npc {
            id: 1,
            name: "Sage du Village".to_string(),
            description: "Un vieil homme sage".to_string(),
            dialogues: vec![
                "Bienvenue, jeune aventurier !".to_string(),
                "Le village a besoin de votre aide.".to_string(),
            ],
            quests: vec![quete],
        };

        assert_eq!(pnj.name(), "Sage du Village");
        assert_eq!(pnj.description(), "Un vieil homme sage");

        let interaction = pnj.interact();
        assert!(interaction.contains("Sage du Village"));
        assert!(interaction.contains("Bienvenue, jeune aventurier !"));
    }

    #[test]
    fn test_pnj_sans_dialogue() {
        let pnj = Npc {
            id: 2,
            name: "Garde Silencieux".to_string(),
            description: "Un garde qui ne parle jamais".to_string(),
            dialogues: vec![],
            quests: vec![],
        };

        let interaction = pnj.interact();
        assert!(interaction.contains("Garde Silencieux"));
        assert!(interaction.contains("n'a rien à dire"));
    }

    #[test]
    fn test_pnj_avec_quete() {
        let pnj = Npc {
            id: 3,
            name: "Marchand".to_string(),
            description: "Un marchand itinérant".to_string(),
            dialogues: vec!["J'ai des objets rares à vendre !".to_string()],
            quests: vec![creer_quete_test()],
        };

        let info_quete = pnj.quest_info();
        assert!(info_quete.contains("Marchand"));
        assert!(info_quete.contains("Retrouver l'Épée Perdue"));
    }

    #[test]
    fn test_pnj_sans_quete() {
        let pnj = Npc {
            id: 4,
            name: "Villageois".to_string(),
            description: "Un simple villageois".to_string(),
            dialogues: vec!["Belle journée, n'est-ce pas ?".to_string()],
            quests: vec![],
        };

        let info_quete = pnj.quest_info();
        assert!(info_quete.contains("pas de quête"));
    }

    #[test]
    fn test_creation_depuis_raw() {
        let quete1 = Quest {
            id: 10,
            name: "Quête 1".to_string(),
            description: "Première quête".to_string(),
            objet_requis_id: None,
            completed: false,
        };

        let quete2 = Quest {
            id: 20,
            name: "Quête 2".to_string(),
            description: "Deuxième quête".to_string(),
            objet_requis_id: Some(100),
            completed: true,
        };

        let toutes_quetes = vec![quete1, quete2];

        let pnj_raw = NpcRaw {
            id: 5,
            name: "Maître des Quêtes".to_string(),
            description: "Distribue les missions".to_string(),
            dialogues: vec!["J'ai des tâches pour vous !".to_string()],
            quests: vec![10, 20, 999], // Le 999 n'existe pas
        };

        let pnj = Npc::from_raw(pnj_raw, &toutes_quetes);
        assert_eq!(pnj.name, "Maître des Quêtes");
        assert_eq!(pnj.quests.len(), 2); // Seules les quêtes existantes
        assert_eq!(pnj.quests[0].id, 10);
        assert_eq!(pnj.quests[1].id, 20);
    }

    // Tests originaux conservés
    #[test]
    fn test_npc_has_quest() {
        let quest = Quest {
            id: 1,
            name: "Test Quest".to_string(),
            description: "Desc".to_string(),
            objet_requis_id: None,
            completed: false,
        };
        let npc = Npc {
            id: 1,
            name: "Test".to_string(),
            description: "desc".to_string(),
            dialogues: vec!["Salut".to_string()],
            quests: vec![quest],
        };
        assert!(npc.quest_info().contains("Test Quest"));
    }

    #[test]
    fn test_npc_interact_empty() {
        let npc = Npc {
            id: 2,
            name: "Muet".to_string(),
            description: "Rien à dire".to_string(),
            dialogues: vec![],
            quests: vec![],
        };
        assert!(npc.interact().contains("n'a rien à dire"));
    }
}
