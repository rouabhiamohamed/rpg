mod item;
mod quest;
mod npc;
mod zone;
mod player;
mod game;
mod data_loader;
mod monster;

use std::error::Error;
use game::Game;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎮 Bienvenue dans le RPG !");
    println!("========================");

    let mut game = Game::new()?;
    game.run()?;

    Ok(())
}