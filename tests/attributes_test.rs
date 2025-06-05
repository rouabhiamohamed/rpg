use rpg::model::attributes::Attributes;
use std::fs;

#[derive(Debug, serde::Deserialize)]
struct AttributesProfile {
    profile: String,
    stats: Attributes,
}

#[test]
fn test_load_attributes_profiles() {
    // Lecture du fichier JSON
    let json_content = fs::read_to_string("data/attributes.json")
        .expect("Impossible de lire le fichier attributes.json");

    // Parsing
    let profils: Vec<AttributesProfile> = serde_json::from_str(&json_content)
        .expect("Erreur de parsing JSON");

    // Vérifications de base
    assert!(!profils.is_empty(), "Aucun profil trouvé dans le fichier");

    for profil in &profils {
        println!("Profil: {} → {:?}", profil.profile, profil.stats);
        assert!(profil.stats.health > 0);
    }
}

#[test]
fn test_mage_stats_are_correct() {
    let json = std::fs::read_to_string("data/attributes.json").unwrap();
    let profils: Vec<AttributesProfile> = serde_json::from_str(&json).unwrap();

    let mage = profils.iter().find(|p| p.profile == "mage").expect("Mage non trouvé");

    assert_eq!(mage.stats.intelligence, 18);
    assert_eq!(mage.stats.health, 80);
}


