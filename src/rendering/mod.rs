//! Rendering functions for game visuals and UI
//!
//! This module handles all drawing operations including:
//! - World rendering (units, sectors, battlefield)
//! - UI panels and HUD elements
//! - Game over and victory screens
//!
//! All rendering must be side-effect free and deterministic.

mod ui;
mod world;

use crate::config::Config;
use crate::game::GameState;
use macroquad::prelude::*;
use macroquad_toolkit::camera::Camera2D as ToolkitCamera;
use macroquad_toolkit::colors::dark;

/// Render the entire game scene
pub fn render_game(game: &GameState, camera: &ToolkitCamera) {
    clear_background(dark::BACKGROUND);

    // Set camera for world rendering
    let mq_camera = macroquad::camera::Camera2D {
        target: camera.target,
        zoom: vec2(
            camera.zoom / screen_width() * 2.0,
            -camera.zoom / screen_height() * 2.0,
        ),
        ..Default::default()
    };
    set_camera(&mq_camera);

    // Draw battlefield bounds
    draw_rectangle_lines(
        0.0,
        0.0,
        Config::WORLD_WIDTH,
        Config::WORLD_HEIGHT,
        2.0,
        dark::TEXT_DIM,
    );

    // Draw sectors
    for sector in &game.sectors {
        world::render_sector(sector);
    }

    // Draw strategic markers
    world::render_markers(&game.markers, &game.sectors);

    // Draw squads and units
    for squad in &game.squads {
        world::render_squad(squad);
    }

    // Draw combat effects
    world::render_combat_effects(&game.combat_effects);

    // Reset to screen camera for UI
    set_default_camera();

    // Draw UI
    ui::render_ui(game);
}

/// Render controls help
pub fn render_controls_help() {
    ui::render_controls_help();
}
