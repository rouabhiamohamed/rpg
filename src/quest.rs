#[derive(Debug, Clone)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    /// Si la quête est liée à un objet précis (par son ID), on le stocke ici
    pub objet_requis_id: Option<u32>,
    pub completed: bool,
}

impl Quest {
    /// Crée une nouvelle quête.
    /// Passez `None` pour `objet_requis_id` si la quête n’est pas liée à un objet spécifique.
    pub fn new(
        id: u32,
        name: String,
        description: String,
        objet_requis_id: Option<u32>,
    ) -> Quest {
        Quest {
            id,
            name,
            description,
            objet_requis_id,
            completed: false,
        }
    }
}
