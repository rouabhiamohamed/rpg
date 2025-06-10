# RPG Game en Rust

Un jeu de rôle en mode texte développé en Rust avec un système de zones connectées et d'interactions avec des NPCs.

## Structure du Projet

```
src/
├── main.rs          # Point d'entrée du jeu
├── game.rs          # Logique principale du jeu et boucle de gameplay
├── player.rs        # Gestion du joueur et création de personnage
├── zone.rs          # Système de zones et navigation
├── npc.rs           # NPCs et leurs interactions
├── quest.rs         # Système de quêtes
├── item.rs          # Objets et inventaire
├── monster.rs       # Système de monstres et combat
└── data_loader.rs   # Chargement des données JSON

data/
├── attributes.json  # Classes de personnages
├── items.json       # Objets du jeu
├── quests.json      # Quêtes disponibles
├── npcs.json        # NPCs du jeu
├── zones.json       # Zones du monde
├── monsters.json    # Monstres et leurs statistiques
└── perso_save.json  # Sauvegarde du joueur (généré automatiquement)
```

## Fonctionnalités

### Création de Personnage
- Choix du nom
- Sélection d'une classe (Guerrier, Mage, Voleur, Paladin)
- Sélection d'objets de départ
- Sauvegarde automatique

### Système de Zones
- Navigation entre zones connectées (Nord, Sud, Est, Ouest)
- Chaque zone a une description unique
- NPCs présents dans certaines zones
- Système de connexions logiques entre zones

### Système de Combat
- Combat au tour par tour contre des monstres
- Système de dégâts basé sur Force vs Défense
- Esquive basée sur l'Agilité (2% par point, max 30%)
- 90% de chance de toucher par attaque
- Loot automatique à 100% de chance
- Expérience gagnée en vainquant des monstres
- Compteur de monstres tués sauvegardé
- Système de fuite disponible

### Système d'Inventaire et d'Équipement
- **`inv`** : Afficher l'inventaire détaillé
- **`inv, X`** : Utiliser l'objet numéro X (potions uniquement)
- **`equiper, X`** : Équiper l'arme/armure/amulette numéro X
- Trois emplacements d'équipement : Arme, Armure, Amulette
- Les objets équipés donnent des bonus de stats
- Déséquipement automatique lors du changement d'équipement
- Stats totales = Stats de base + Bonus d'équipement

### Système de Quêtes
- Interaction avec NPCs pour obtenir des quêtes
- Vérification automatique des prérequis lors du dialogue
- Quêtes automatiquement terminées si les conditions sont remplies
- Remise d'objets requis automatique
- Récompenses en or et expérience (théorique)

### Interactions
- Dialogue avec les NPCs
- Système de quêtes
- Inventaire et statistiques du joueur
- Menu d'actions numérotées

### Sauvegarde
- Sauvegarde automatique après chaque action
- Chargement de sauvegarde au démarrage

## Comment Jouer

1. **Compilez et lancez le jeu :**
   ```bash
   cargo run
   ```

2. **Première fois :**
    - Créez votre personnage en suivant les instructions
    - Choisissez votre classe et votre équipement de départ

3. **Navigation :**
    - Utilisez les numéros pour choisir vos actions
    - Déplacez-vous entre les zones avec les directions disponibles
    - Tapez `aide` pour voir les commandes disponibles

4. **Interactions :**
    - Parlez aux NPCs pour obtenir des quêtes et les terminer automatiquement
    - Combattez les monstres pour obtenir du loot et de l'XP
    - Gérez votre inventaire avec `inv` et `inv, X`
    - Équipez des armes/armures/amulettes avec `equiper, X`
    - Utilisez des potions pour vous soigner
    - Consultez vos statistiques complètes et vos victoires

5. **Commandes spéciales :**
    - `aide` : Affiche l'aide complète
    - `inv` : Affiche l'inventaire détaillé avec tous les objets
    - `inv, 3` : Utilise l'objet numéro 3 (ex: potion)
    - `equiper, 5` : Équipe l'objet numéro 5 (arme/armure/amulette)
    - `quit` : Quitte le jeu (avec sauvegarde)

## Architecture Technique

### Combat System
Le système de combat utilise des mécaniques simples mais efficaces :
- **Dégâts = Force de l'attaquant - Défense du défenseur (minimum 1)**
- **Esquive = Agilité × 2% (maximum 30%)**
- **Précision = 90% de chance de toucher**
- **Loot = 100% de chance de drop**

Exemple : Un joueur avec 12 de Force contre un monstre avec 2 de Défense inflige 10 dégâts par attaque.

### Traits et Modularité
Le code utilise des traits Rust pour définir les comportements :
- Modularité claire entre les différents systèmes
- Chargement des données depuis des fichiers JSON
- Gestion d'erreurs robuste

### Système de Zones
Les zones sont connectées via un système de directions :
- Zone 1 : Place du Village (centre) - Sûre, avec NPCs
- Zone 2 : Forêt du Nord - Loups et Gobelins
- Zone 11 : Maison d'Elara (Est de la place) - Sûre
- Zone 12 : Ferme de Tom (Nord-Est) - Bandits occasionnels
- Zone 21 : Clairière Mystique (Nord de la ferme) - Ours et Squelettes

### Données JSON
Toutes les données du jeu sont externalisées en JSON :
- Facilite la modification du contenu sans recompilation
- Structure claire et extensible
- Références par ID entre les différents éléments

## Exemples de Gameplay

### Gestion d'Inventaire et Équipement
```
🎮 Votre choix : inv

🎒 Inventaire détaillé :
  [1] Potion de Soin - Restaure 50 points de vie [Consommable]
      Stats: Santé: +50
  [2] Épée en Fer - Une épée solide en fer forgé [Arme]
      Stats: Force: +8
  [3] Amulette de Vitesse - Une amulette qui augmente l'agilité [Amulette]
      Stats: Agilité: +5

⚔️ Équipement actuel :
  Arme: Épée d'Entraînement (Force: +3)
  Armure: Aucune  
  Amulette: Aucune

🎮 Votre choix : equiper, 2
🔄 Vous déséquipez Épée d'Entraînement et équipez Épée en Fer
📊 Vos stats totales: Force: 18, Défense: 7, Agilité: 8, Santé Max: 110

🎮 Votre choix : inv, 1
🍶 Vous utilisez Potion de Soin et récupérez 35 points de vie !
💚 Santé: 110/110
```

### Combat et Récompenses
```
💥 Vous frappez Gobelin Voleur pour 12 dégâts !
👹 Gobelin Voleur : [████░░░░░░░░░░░░░░░░] 8/20 HP

🎉 Victoire ! Vous avez vaincu Gobelin Voleur !
💰 Butin obtenu :
  📦 Clé Rouillée - Une vieille clé qui pourrait ouvrir quelque chose
  📦 Amulette de Vitesse - Une amulette qui augmente l'agilité
✨ Vous gagnez 10 points d'expérience !
🏆 Vous avez maintenant tué 3 Gobelin Voleur au total !
```

### Interaction avec NPC et Quêtes
```
💬 Gérard le Marchand dit : 'Bienvenue dans ma boutique !'
✅ Vous avez l'objet requis ! Quête 'Herbes Rares' terminée !
📤 Vous donnez Herbes Médicinales à Gérard le Marchand
🎁 Récompense: 50 pièces d'or et 25 XP !
```

## Extension du Jeu

Pour ajouter du contenu :

1. **Nouvelles zones :** Modifiez `data/zones.json`
2. **Nouveaux NPCs :** Ajoutez dans `data/npcs.json`
3. **Nouveaux monstres :** Complétez `data/monsters.json`
4. **Nouvelles quêtes :** Enrichissez `data/quests.json`
5. **Nouveaux objets :** Ajoutez dans `data/items.json`
6. **Nouvelles classes :** Étendez `data/attributes.json`
