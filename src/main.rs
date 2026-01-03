mod ai;
mod config;
mod data;
mod game;
mod input;
mod network;
mod rendering;
mod simulation;
mod types;

use ai::AICoordinator;
use config::Config;
use data::GameData;
use game::GameState;
use input::InputHandler;
use macroquad::prelude::*;
use macroquad_toolkit::camera::Camera2D;
use types::FactionId;

fn window_conf() -> Conf {
    Conf {
        window_title: "Fracture Command - 3-Faction Prototype".to_owned(),
        window_width: 1600,
        window_height: 900,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load game data
    println!("Loading game data...");
    let game_data = match GameData::load() {
        Ok(data) => {
            println!("Game data loaded successfully!");
            data
        }
        Err(e) => {
            eprintln!("Failed to load game data: {}", e);
            eprintln!("Make sure assets/data/ folder exists with all JSON files.");
            return;
        }
    };

    // Initialize game state (player controls Faction1)
    let mut game = GameState::new(FactionId::Faction1);
    let mut input_handler = InputHandler::new();

    // Initialize AI coordinator
    let mut ai_coordinator = AICoordinator::new();
    let ai_personalities = vec![
        "aggressive".to_string(), // Faction2
        "defensive".to_string(),  // Faction3
    ];
    ai_coordinator.initialize_from_data(&game_data, &game.commanders, &ai_personalities);

    // Initialize camera at center of world
    let camera_center = vec2(Config::WORLD_WIDTH / 2.0, Config::WORLD_HEIGHT / 2.0);
    let mut camera = Camera2D::new(camera_center, 1.0);

    // Spawn initial units for testing
    spawn_initial_units(&mut game);

    // Main game loop
    loop {
        let dt = get_frame_time();

        // Handle input (for local player)
        input_handler.handle_input(&mut game, &mut camera, dt);

        // Update AI commanders
        ai_coordinator.update(&mut game, dt);

        // Update game state
        game.update(dt);

        // Render
        rendering::render_game(&game, &camera);
        rendering::render_controls_help();

        // Exit on ESC
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

/// Spawn initial units for all three factions to test the simulation
fn spawn_initial_units(game: &mut GameState) {
    // Commented out - start with empty battlefield
    // Players can spawn their own units
    
    // use types::{FactionId, SquadOrder, UnitType};
    //
    // // Faction 1 initial forces (Player)
    // let _ = game.spawn_squad(
    //     FactionId::Faction1,
    //     UnitType::Infantry,
    //     3,
    //     SquadOrder::Advance,
    // );
    // let _ = game.spawn_squad(FactionId::Faction1, UnitType::Heavy, 2, SquadOrder::Hold);
    //
    // // Faction 2 initial forces (AI - Aggressive)
    // let _ = game.spawn_squad(
    //     FactionId::Faction2,
    //     UnitType::Infantry,
    //     3,
    //     SquadOrder::Advance,
    // );
    // let _ = game.spawn_squad(FactionId::Faction2, UnitType::Drone, 2, SquadOrder::Skirmish);
    //
    // // Faction 3 initial forces (AI - Defensive)
    // let _ = game.spawn_squad(
    //     FactionId::Faction3,
    //     UnitType::Heavy,
    //     3,
    //     SquadOrder::Fortify,
    // );
    // let _ = game.spawn_squad(FactionId::Faction3, UnitType::Artillery, 1, SquadOrder::Hold);
    //
    // // Set some initial doctrines
    // game.set_doctrine(
    //     FactionId::Faction1,
    //     0,
    //     Some(types::Doctrine::RapidDeployment),
    // );
    // game.set_doctrine(
    //     FactionId::Faction2,
    //     0,
    //     Some(types::Doctrine::AggressivePosture),
    // );
    // game.set_doctrine(
    //     FactionId::Faction3,
    //     0,
    //     Some(types::Doctrine::EntrenchedAssault),
    // );
}
