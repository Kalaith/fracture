//! Game configuration constants and balance values
//!
//! This module centralizes all game constants including supply limits, combat parameters,
//! camera settings, and victory conditions. Modify these values to balance the game.

use macroquad::prelude::*;

/// Game configuration constants
pub struct Config;

impl Config {
    // Supply and balance
    pub const SUPPLY_MAX: u32 = 100;
    pub const STARTING_SUPPLY: u32 = 20;

    // Battlefield dimensions
    pub const WORLD_WIDTH: f32 = 2000.0;
    pub const WORLD_HEIGHT: f32 = 1500.0;

    // Sector configuration
    pub const NUM_SECTORS: u32 = 4;
    pub const SECTOR_RADIUS: f32 = 150.0;
    pub const SECTOR_CAPTURE_RATE: f32 = 0.1; // Per second per unit nearby

    // Unit spawn configuration
    pub const SPAWN_COOLDOWN: f32 = 2.0; // Seconds between spawns
    pub const SPAWN_OFFSET: f32 = 100.0; // Distance from spawn point

    // Combat configuration
    pub const ATTACK_COOLDOWN: f32 = 1.0; // Seconds between attacks
    pub const DAMAGE_VARIANCE: f32 = 0.2; // +/- 20% damage randomness

    // AI behavior
    pub const AI_UPDATE_INTERVAL: f32 = 0.2; // How often AI recalculates targets
    pub const SKIRMISH_RETREAT_HEALTH: f32 = 0.5; // Retreat when below 50% health
    pub const FORTIFY_BUILD_TIME: f32 = 5.0; // Seconds to build defenses

    // Doctrine modifiers
    pub const AGGRESSIVE_SPEED_MULT: f32 = 1.5;
    pub const AGGRESSIVE_DAMAGE_MULT: f32 = 0.8; // Takes more damage
    pub const ENTRENCHED_SPEED_MULT: f32 = 0.7;
    pub const ENTRENCHED_DEFENSE_MULT: f32 = 1.4; // Takes less damage
    pub const SHOCK_DAMAGE_BONUS: f32 = 1.5;
    pub const SHOCK_DURATION: f32 = 5.0; // Seconds after spawn
    pub const RAPID_SPAWN_MULT: f32 = 0.6; // 40% faster spawning
    pub const EFFICIENCY_COST_MULT: f32 = 0.8; // 20% cheaper units

    // Camera configuration
    pub const CAMERA_MIN_ZOOM: f32 = 0.3;
    pub const CAMERA_MAX_ZOOM: f32 = 2.0;
    pub const CAMERA_ZOOM_SPEED: f32 = 0.1;
    pub const CAMERA_PAN_SPEED: f32 = 500.0;

    // UI configuration
    pub const UI_MARGIN: f32 = 10.0;
    pub const UI_PANEL_WIDTH: f32 = 300.0;
    pub const UI_BUTTON_HEIGHT: f32 = 40.0;
    pub const UI_FONT_SIZE: f32 = 20.0;

    // Victory conditions
    pub const VICTORY_SECTOR_CONTROL: u32 = 7; // Control all 7 sectors to win
    pub const VICTORY_TIME_REQUIRED: f32 = 10.0; // Hold for 10 seconds
}

/// Faction spawn positions (3-faction layout) - closer together for more action
pub fn get_spawn_position(faction_id: crate::types::FactionId) -> Vec2 {
    let center_x = Config::WORLD_WIDTH / 2.0;
    let center_y = Config::WORLD_HEIGHT / 2.0;
    let spawn_distance = 400.0; // Distance from center

    match faction_id {
        crate::types::FactionId::Faction1 => vec2(center_x - spawn_distance, center_y), // Left
        crate::types::FactionId::Faction2 => vec2(center_x + spawn_distance, center_y), // Right
        crate::types::FactionId::Faction3 => vec2(center_x, center_y - spawn_distance), // Top
    }
}

/// Sector positions on battlefield (arranged for 3-faction play)
pub fn get_sector_positions() -> Vec<Vec2> {
    vec![
        // Central contested sector
        vec2(Config::WORLD_WIDTH * 0.5, Config::WORLD_HEIGHT * 0.5),
        // Faction 1 side sectors (left)
        vec2(Config::WORLD_WIDTH * 0.25, Config::WORLD_HEIGHT * 0.35),
        vec2(Config::WORLD_WIDTH * 0.25, Config::WORLD_HEIGHT * 0.65),
        // Faction 2 side sectors (right)
        vec2(Config::WORLD_WIDTH * 0.75, Config::WORLD_HEIGHT * 0.35),
        vec2(Config::WORLD_WIDTH * 0.75, Config::WORLD_HEIGHT * 0.65),
        // Faction 3 side sectors (top)
        vec2(Config::WORLD_WIDTH * 0.4, Config::WORLD_HEIGHT * 0.25),
        vec2(Config::WORLD_WIDTH * 0.6, Config::WORLD_HEIGHT * 0.25),
    ]
}
