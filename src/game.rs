use std::collections::HashMap;
use std::io::{self, Write};
use crate::player::Player;
use crate::zone::{Zone, Direction};
use crate::npc::Npc;
use crate::item::Item;
use crate::quest::Quest;
use crate::monster::{Monster, CombatResult, AttackResult, calculate_damage, check_dodge, calculate_hit_chance};
use crate::data_loader::{load_items, load_quests, load_zones, load_npcs, load_monsters};

pub struct Game {
    player: Player,
    zones: HashMap<u32, Zone>,
    all_npcs: Vec<Npc>,
    all_items: Vec<Item>,
    all_quests: Vec<Quest>,
    all_monsters: Vec<Monster>,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("🔄 Initialisation du jeu...");

        // Charger les données depuis les fichiers JSON
        let all_items = load_items("data/items.json")?;
        let all_quests = load_quests("data/quests.json")?;
        let all_npcs = load_npcs("data/npcs.json", &all_quests)?;
        let all_monsters = load_monsters("data/monsters.json")?;
        let zones = load_zones("data/zones.json", &all_npcs, &all_monsters)?;

        // Charger ou créer un personnage
        let player = if std::path::Path::new("data/perso_save.json").exists() {
            println!("📁 Sauvegarde trouvée !");
            let choix = Self::lire_input("Voulez-vous charger votre sauvegarde ? (o/n) : ");
            if choix.to_lowercase() == "o" || choix.to_lowercase() == "oui" {
                Player::load_character("data/perso_save.json")?
            } else {
                Player::create_character()?
            }
        } else {
            println!("📝 Aucune sauvegarde trouvée, création d'un nouveau personnage...");
            Player::create_character()?
        };

        Ok(Game {
            player,
            zones,
            all_npcs,
            all_items,
            all_quests,
            all_monsters,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🎯 Début de l'aventure !");
        println!("Tapez 'aide' pour voir les commandes disponibles.");

        loop {
            // Afficher la zone actuelle
            self.afficher_zone_actuelle();

            // Afficher le menu d'interactions
            let choix = self.afficher_menu_interactions();

            // Traiter le choix du joueur
            if !self.traiter_choix(choix)? {
                break; // Le joueur veut quitter
            }

            // Sauvegarder après chaque action
            self.player.save_character("data/perso_save.json")?;
        }

        println!("👋 Merci d'avoir joué ! À bientôt !");
        Ok(())
    }

    fn afficher_zone_actuelle(&self) {
        if let Some(zone) = self.zones.get(&self.player.current_zone_id) {
            println!("\n{}", "=".repeat(50));
            zone.afficher();
            println!("{}", "=".repeat(50));
        } else {
            println!("❌ Erreur : Zone {} introuvable !", self.player.current_zone_id);
        }
    }

    fn afficher_menu_interactions(&self) -> String {
        println!("\n📋 Actions disponibles :");
        let mut compteur = 1;

        // Mouvements
        if let Some(zone) = self.zones.get(&self.player.current_zone_id) {
            for direction in &zone.connections {
                println!("  [{}] Aller vers {:?}", compteur, direction);
                compteur += 1;
            }
        }

        // Interactions avec les NPCs
        if let Some(zone) = self.zones.get(&self.player.current_zone_id) {
            for npc in &zone.npcs {
                println!("  [{}] Parler à {}", compteur, npc.name());
                compteur += 1;
            }
        }

        // Combat avec les monstres
        if let Some(zone) = self.zones.get(&self.player.current_zone_id) {
            for monster in &zone.monsters {
                if monster.is_alive() {
                    println!("  [{}] Combattre {}", compteur, monster.name);
                    compteur += 1;
                }
            }
        }

        // Options système
        println!("  [inv] Voir inventaire");

        println!("  [stat] Voir statistiques");
        //compteur += 1;
        println!("  [aide] Aide");
        println!("  [quit] Quitter");

        Self::lire_input("\n🎮 Votre choix : ")
    }

    fn traiter_choix(&mut self, choix: String) -> Result<bool, Box<dyn std::error::Error>> {
        // Vérifier les commandes spéciales d'abord
        if choix.to_lowercase().starts_with("inv") {
            return self.gerer_commande_inventaire(&choix);
        }

        if choix.to_lowercase().starts_with("stat") {
            self.afficher_statistiques();
            return Ok(true);
        }

        if choix.to_lowercase().starts_with("equiper") {
            return self.gerer_commande_equiper(&choix);
        }

        match choix.to_lowercase().as_str() {
            "aide" => {
                self.afficher_aide();
                Ok(true)
            },
            "quit" => Ok(false),
            _ => {
                if let Ok(num) = choix.parse::<usize>() {
                    self.traiter_choix_numerique(num)
                } else {
                    println!("❌ Choix invalide. Tapez 'aide' pour voir les commandes.");
                    Ok(true)
                }
            }
        }
    }



    fn traiter_choix_numerique(&mut self, choix: usize) -> Result<bool, Box<dyn std::error::Error>> {
        let mut compteur = 1;

        // Collecter les données nécessaires d'abord pour éviter les conflits de borrow
        let zone_id = self.player.current_zone_id;
        let zone_data = self.zones.get(&zone_id).cloned();

        if let Some(zone) = zone_data {
            // Vérifier les mouvements
            for direction in &zone.connections {
                if compteur == choix {
                    return self.deplacer_joueur(direction);
                }
                compteur += 1;
            }

            // Vérifier les interactions avec les NPCs
            for npc in &zone.npcs {
                if compteur == choix {
                    // Cloner le NPC pour éviter les problèmes de borrow
                    let npc_clone = npc.clone();
                    self.interagir_avec_npc(&npc_clone);
                    return Ok(true);
                }
                compteur += 1;
            }

            // Vérifier les combats avec les monstres
            for (i, monster) in zone.monsters.iter().enumerate() {
                if monster.is_alive() && compteur == choix {
                    return self.combattre_monstre(i);
                }
                compteur += 1;
            }
        }

        /*
        // Options système
        if compteur == choix {
            self.afficher_inventaire();
            return Ok(true);
        }
        compteur += 1;
        */

        /*
        if compteur == choix {
            self.afficher_statistiques();
            return Ok(true);
        }
        */
        
        println!("❌ Choix invalide.");
        Ok(true)
    }


    fn deplacer_joueur(&mut self, direction: &Direction) -> Result<bool, Box<dyn std::error::Error>> {
        // Pour l'instant, implémentation simple : chaque direction mène à une zone spécifique
        let nouvelle_zone = match direction {
            Direction::North => self.player.current_zone_id + 1,
            Direction::South => if self.player.current_zone_id > 1 { self.player.current_zone_id - 1 } else { 1 },
            Direction::East => self.player.current_zone_id + 10,
            Direction::West => if self.player.current_zone_id >= 10 { self.player.current_zone_id - 10 } else { self.player.current_zone_id },
        };

        if self.zones.contains_key(&nouvelle_zone) {
            self.player.current_zone_id = nouvelle_zone;
            println!("🚶 Vous vous dirigez vers {:?}...", direction);
        } else {
            println!("🚫 Il n'y a rien dans cette direction.");
        }

        Ok(true)
    }

    fn interagir_avec_npc(&mut self, npc: &Npc) {
        println!("\n💬 {}", npc.interact());

        // Vérifier les quêtes
        for quest in &npc.quests {
            if quest.completed {
                continue; // Skip les quêtes déjà complétées
            }

            let mut can_complete = true;
            let mut completion_message = String::new();

            // Vérifier les prérequis de la quête
            if let Some(required_item_id) = quest.objet_requis_id {
                // Chercher l'objet requis dans l'inventaire
                if let Some(_item) = self.player.inventaire.iter().find(|item| item.id == required_item_id) {
                    completion_message = format!("✅ Vous avez l'objet requis ! Quête '{}' terminée !", quest.name);
                    // Dans une vraie implémentation, on marquerait la quête comme complétée
                    // et on retirerait l'objet si nécessaire
                } else {
                    can_complete = false;
                    if let Some(required_item) = self.all_items.iter().find(|item| item.id == required_item_id) {
                        completion_message = format!("📋 Quête '{}': Apportez-moi {} pour terminer cette quête.",
                                                     quest.name, required_item.name);
                    } else {
                        completion_message = format!("📋 Quête '{}': {}", quest.name, quest.description);
                    }
                }
            } else {
                // Pas d'objet requis, la quête peut être complétée automatiquement
                completion_message = format!("✅ Quête '{}' terminée ! Merci pour votre aide !", quest.name);
            }

            println!("ℹ️  {}", completion_message);

            if can_complete && quest.objet_requis_id.is_some() {
                // Retirer l'objet de l'inventaire si c'est une quête avec objet requis
                if let Some(required_item_id) = quest.objet_requis_id {
                    if let Some(pos) = self.player.inventaire.iter().position(|item| item.id == required_item_id) {
                        let removed_item = self.player.inventaire.remove(pos);
                        println!("📤 Vous donnez {} à {}", removed_item.name, npc.name());

                        // Donner une récompense (exemple simple)
                        println!("🎁 Récompense: 50 pièces d'or et 25 XP !");
                    }
                }
            }
        }

        if npc.quests.is_empty() || npc.quests.iter().all(|q| q.completed) {
            println!("ℹ️  {} : 'Je n'ai pas de quête pour toi en ce moment.'", npc.name());
        }
    }

    fn afficher_inventaire(&self) {
        println!("\n🎒 Inventaire :");
        if self.player.inventaire.is_empty() {
            println!("  Vide");
        } else {
            for (i, item) in self.player.inventaire.iter().enumerate() {
                println!("  [{}] {}", i + 1, item.name);
            }
        }
        println!("\n💡 Utilisez 'inv' pour plus de détails ou 'inv, X' pour utiliser un objet");
    }

    fn afficher_statistiques(&self) {
        let total_stats = self.player.get_total_stats();

        println!("\n📊 Statistiques de {} :", self.player.nom);
        println!("  💚 Santé: {}/{}", self.player.current_health, self.player.get_max_health());
        println!("  ⚔️  Force: {} (base: {} + équipement: {})",
                 total_stats.strength,
                 self.player.base_stats.strength,
                 total_stats.strength - self.player.base_stats.strength
        );
        println!("  🛡️  Défense: {} (base: {} + équipement: {})",
                 total_stats.defense,
                 self.player.base_stats.defense,
                 total_stats.defense - self.player.base_stats.defense
        );
        println!("  💨 Agilité: {} (base: {} + équipement: {})",
                 total_stats.agility,
                 self.player.base_stats.agility,
                 total_stats.agility - self.player.base_stats.agility
        );
        println!("  🗺️  Zone actuelle: {}", self.player.current_zone_id);

        // Afficher les statistiques de monstres tués
        if !self.player.monster_kills.is_empty() {
            println!("\n🏆 Monstres vaincus :");
            for (monster_id, count) in &self.player.monster_kills {
                if let Some(monster) = self.all_monsters.iter().find(|m| m.id == *monster_id) {
                    println!("  - {} : {} fois", monster.name, count);
                }
            }
        }
    }

    fn afficher_aide(&self) {
        println!("\n📖 Aide :");
        println!("  - Utilisez les numéros pour choisir une action");
        println!("  - Déplacez-vous entre les zones avec les directions");
        println!("  - Parlez aux NPCs pour obtenir des quêtes");
        println!("  - Combattez les monstres pour obtenir du loot");
        println!("  - 'inv' : Voir l'inventaire");
        println!("  - 'inv, X' : Utiliser l'objet numéro X");
        println!("  - 'equiper, X' : Équiper l'objet numéro X");
        println!("  - 'quit' : Quitter le jeu");
        println!("  - Le jeu sauvegarde automatiquement");
    }

    fn gerer_commande_inventaire(&mut self, commande: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = commande.split(',').map(|s| s.trim()).collect();

        if parts.len() == 1 {
            // Juste "inv" - afficher l'inventaire
            self.afficher_inventaire_detaille();
        } else if parts.len() == 2 {
            // "inv, X" - utiliser l'objet numéro X
            if let Ok(index) = parts[1].parse::<usize>() {
                if index > 0 && index <= self.player.inventaire.len() {
                    self.utiliser_objet(index - 1)?;
                } else {
                    println!("❌ Numéro d'objet invalide !");
                }
            } else {
                println!("❌ Format invalide ! Utilisez : inv, numéro");
            }
        } else {
            println!("❌ Format invalide ! Utilisez : 'inv' ou 'inv, numéro'");
        }

        Ok(true)
    }

    fn gerer_commande_equiper(&mut self, commande: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = commande.split(',').map(|s| s.trim()).collect();

        if parts.len() == 2 {
            if let Ok(index) = parts[1].parse::<usize>() {
                if index > 0 && index <= self.player.inventaire.len() {
                    self.equiper_objet(index - 1)?;
                } else {
                    println!("❌ Numéro d'objet invalide !");
                }
            } else {
                println!("❌ Format invalide ! Utilisez : equiper, numéro");
            }
        } else {
            println!("❌ Format invalide ! Utilisez : 'equiper, numéro'");
        }

        Ok(true)
    }

    fn afficher_inventaire_detaille(&self) {
        println!("\n🎒 Inventaire détaillé :");
        if self.player.inventaire.is_empty() {
            println!("  Vide");
        } else {
            for (i, item) in self.player.inventaire.iter().enumerate() {
                println!("  [{}] {} - {} [{}]",
                         i + 1,
                         item.name,
                         item.description,
                         item.get_type_name()
                );
                if item.is_equipable() || item.is_consumable() {
                    println!("      Stats: {}", item.get_stats_description());
                }
            }
        }

        println!("\n⚔️ Équipement actuel :");
        if let Some(ref arme) = self.player.equipment.arme {
            println!("  Arme: {} ({})", arme.name, arme.get_stats_description());
        } else {
            println!("  Arme: Aucune");
        }

        if let Some(ref armure) = self.player.equipment.armure {
            println!("  Armure: {} ({})", armure.name, armure.get_stats_description());
        } else {
            println!("  Armure: Aucune");
        }

        if let Some(ref amulette) = self.player.equipment.amulette {
            println!("  Amulette: {} ({})", amulette.name, amulette.get_stats_description());
        } else {
            println!("  Amulette: Aucune");
        }
    }

    fn utiliser_objet(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let item = self.player.inventaire[index].clone();

        if !item.utilisable {
            println!("❌ Cet objet ne peut pas être utilisé !");
            return Ok(());
        }

        match item.item_type {
            crate::item::ItemType::Consommable => {
                if item.health > 0 {
                    let old_health = self.player.current_health;
                    self.player.heal(item.health);
                    let healed = self.player.current_health - old_health;

                    println!("🍶 Vous utilisez {} et récupérez {} points de vie !",
                             item.name, healed);
                    println!("💚 Santé: {}/{}",
                             self.player.current_health,
                             self.player.get_max_health());

                    // Retirer l'objet de l'inventaire
                    self.player.inventaire.remove(index);
                } else {
                    println!("❌ Cette potion n'a aucun effet !");
                }
            },
            _ => {
                println!("❌ Cet objet ne peut pas être consommé ! Essayez de l'équiper avec 'equiper, {}'", index + 1);
            }
        }

        Ok(())
    }

    fn equiper_objet(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let item = self.player.inventaire[index].clone();

        if !item.is_equipable() {
            println!("❌ Cet objet ne peut pas être équipé !");
            return Ok(());
        }

        // Retirer l'objet de l'inventaire
        let item_to_equip = self.player.inventaire.remove(index);

        match item_to_equip.item_type {
            crate::item::ItemType::Arme => {
                if let Some(old_weapon) = self.player.equipment.arme.take() {
                    println!("🔄 Vous déséquipez {} et équipez {}", old_weapon.name, item_to_equip.name);
                    self.player.inventaire.push(old_weapon);
                } else {
                    println!("⚔️  Vous équipez {}", item_to_equip.name);
                }
                self.player.equipment.arme = Some(item_to_equip);
            },
            crate::item::ItemType::Armure => {
                if let Some(old_armor) = self.player.equipment.armure.take() {
                    println!("🔄 Vous déséquipez {} et équipez {}", old_armor.name, item_to_equip.name);
                    self.player.inventaire.push(old_armor);
                } else {
                    println!("🛡️  Vous équipez {}", item_to_equip.name);
                }
                self.player.equipment.armure = Some(item_to_equip);
            },
            crate::item::ItemType::Amulette => {
                if let Some(old_amulet) = self.player.equipment.amulette.take() {
                    println!("🔄 Vous déséquipez {} et équipez {}", old_amulet.name, item_to_equip.name);
                    self.player.inventaire.push(old_amulet);
                } else {
                    println!("💎 Vous équipez {}", item_to_equip.name);
                }
                self.player.equipment.amulette = Some(item_to_equip);
            },
            _ => {
                // Remettre l'objet dans l'inventaire si ce n'est pas équipable
                self.player.inventaire.insert(index, item_to_equip);
                println!("❌ Cet objet ne peut pas être équipé !");
            }
        }

        // Afficher les nouvelles stats
        let total_stats = self.player.get_total_stats();
        println!("📊 Vos stats totales: Force: {}, Défense: {}, Agilité: {}, Santé Max: {}",
                 total_stats.strength, total_stats.defense, total_stats.agility, total_stats.health);

        Ok(())
    }

    fn combattre_monstre(&mut self, monster_index: usize) -> Result<bool, Box<dyn std::error::Error>> {
        println!("\n⚔️ ===== COMBAT ! =====");

        let zone_id = self.player.current_zone_id;
        let zone = self.zones.get_mut(&zone_id).unwrap();
        let monster = &mut zone.monsters[monster_index].clone();

        println!("🥊 Vous engagez le combat contre {} !", monster.name);
        println!("👹 {} : {}", monster.name, monster.health_bar());
        println!("🧑‍⚔️ {} : {}/{} HP",
                 self.player.nom,
                 self.player.current_health,
                 self.player.get_max_health()
        );

        loop {
            println!("\n{}", "─".repeat(40));
            println!("🎯 Votre tour !");
            println!("  [1] Attaquer");
            println!("  [2] Fuir");

            let choix = Self::lire_input("Votre action : ");

            match choix.as_str() {
                "1" => {
                    // Attaque du joueur
                    let player_stats = self.player.get_total_stats().clone();
                    let attack_result = self.attaque_joueur(&player_stats, monster);
                    match attack_result {
                        AttackResult::Hit(damage) => {
                            println!("💥 Vous frappez {} pour {} dégâts !", monster.name, damage);
                            monster.take_damage(damage);
                            println!("👹 {} : {}", monster.name, monster.health_bar());
                        },
                        AttackResult::Miss => {
                            println!("😅 Votre attaque rate sa cible !");
                        },
                        AttackResult::Dodge => {
                            // Impossible pour le joueur d'esquiver en attaquant
                        }
                    }

                    // Vérifier si le monstre est vaincu
                    if !monster.is_alive() {
                        println!("\n🎉 Victoire ! Vous avez vaincu {} !", monster.name);

                        // Ajouter le kill au compteur
                        self.player.add_monster_kill(monster.id);
                        return self.gerer_victoire(monster);
                    }
                },
                "2" => {
                    println!("🏃 Vous fuyez le combat !");
                    return Ok(true);
                },
                _ => {
                    println!("❌ Choix invalide !");
                    continue;
                }
            }

            // Tour du monstre
            println!("\n🔥 {} attaque !", monster.name);
            let player_stats = self.player.get_total_stats();
            let attack_result = self.attaque_monstre(monster, &player_stats);
            match attack_result {
                AttackResult::Hit(damage) => {
                    println!("💢 {} vous frappe pour {} dégâts !", monster.name, damage);
                    self.player.take_damage(damage);
                    println!("🧑‍⚔️ Votre santé : {}/{} HP",
                             self.player.current_health,
                             self.player.get_max_health()
                    );
                },
                AttackResult::Dodge => {
                    println!("💨 Vous esquivez l'attaque de {} !", monster.name);
                },
                AttackResult::Miss => {
                    println!("😌 L'attaque de {} vous rate !", monster.name);
                }
            }

            // Vérifier si le joueur est vaincu
            if !self.player.is_alive() {
                println!("\n💀 Défaite ! Vous avez été vaincu par {} !", monster.name);
                println!("🏥 Vous vous réveillez au village avec 1 HP...");
                self.player.current_health = 1;
                self.player.current_zone_id = 1; // Retour au village
                return Ok(true);
            }
        }
    }

    fn attaque_joueur(&self, player_stats: &crate::player::Attributes, monster: &Monster) -> AttackResult {
        if calculate_hit_chance() {
            let damage = calculate_damage(player_stats.strength, monster.defense);
            AttackResult::Hit(damage)
        } else {
            AttackResult::Miss
        }
    }

    fn attaque_monstre(&self, monster: &Monster, player_stats: &crate::player::Attributes) -> AttackResult {
        if calculate_hit_chance() {
            if check_dodge(player_stats.agility) {
                AttackResult::Dodge
            } else {
                let damage = calculate_damage(monster.strength, player_stats.defense);
                AttackResult::Hit(damage)
            }
        } else {
            AttackResult::Miss
        }
    }

    fn gerer_victoire(&mut self, monster: &Monster) -> Result<bool, Box<dyn std::error::Error>> {
        println!("💰 Butin obtenu :");

        for item_id in &monster.loot {
            if let Some(item) = self.all_items.iter().find(|i| i.id == *item_id) {
                println!("  📦 {} - {}", item.name, item.description);
                self.player.inventaire.push(item.clone());
            }
        }

        if monster.experience > 0 {
            println!("✨ Vous gagnez {} points d'expérience !", monster.experience);
        }

        // Afficher les stats de kill
        let kills = self.player.get_monster_kills(monster.id);
        println!("🏆 Vous avez maintenant tué {} {} au total !", kills, monster.name);

        Ok(true)
    }

    fn lire_input(msg: &str) -> String {
        print!("{}", msg);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}