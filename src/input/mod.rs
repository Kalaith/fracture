//! Input handling and command processing
//!
//! This module processes keyboard and mouse input, translating it into game commands.
//! Handles camera controls, unit spawning, doctrine selection, and squad orders.
//! Input should emit commands that are processed by the game state.

use crate::config::Config;
use crate::game::GameState;
use crate::types::*;
use macroquad::prelude::*;
use macroquad_toolkit::camera::Camera2D;

/// Input handling for game controls
pub struct InputHandler {
    pub selected_squad: Option<u32>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            selected_squad: None,
        }
    }

    /// Handle all input for the game
    pub fn handle_input(&mut self, game: &mut GameState, camera: &mut Camera2D, dt: f32) {
        // Game controls (pause, speed)
        self.handle_game_controls(game);

        // Camera controls
        self.handle_camera_input(camera, dt);

        // Unit spawning
        self.handle_spawn_input(game);

        // Doctrine selection
        self.handle_doctrine_input(game);

        // Squad selection and orders
        self.handle_squad_input(game, camera);
        
        // Strategic markers
        self.handle_marker_input(game, camera);
    }

    fn handle_game_controls(&self, game: &mut GameState) {
        // Pause/unpause with SPACE
        if is_key_pressed(KeyCode::Space) {
            game.paused = !game.paused;
        }

        // Game speed controls (using - and = keys)
        if is_key_pressed(KeyCode::Minus) {
            game.game_speed = (game.game_speed * 0.5).max(0.25); // Slow down
        }
        if is_key_pressed(KeyCode::Equal) {
            game.game_speed = (game.game_speed * 2.0).min(4.0); // Speed up
        }
        if is_key_pressed(KeyCode::Key0) {
            game.game_speed = 1.0; // Reset to normal
        }
    }

    fn handle_camera_input(&self, camera: &mut Camera2D, dt: f32) {
        let mut pan_direction = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            pan_direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            pan_direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            pan_direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            pan_direction.x += 1.0;
        }

        if pan_direction != Vec2::ZERO {
            pan_direction = pan_direction.normalize();
            camera.target += pan_direction * Config::CAMERA_PAN_SPEED * dt / camera.zoom;
        }

        // Zoom with mouse wheel (multiplicative for smooth zooming)
        let (_wheel_x, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            // Use multiplicative zoom for consistent feel across zoom range
            let zoom_factor = 1.0 + (wheel_y.signum() * Config::CAMERA_ZOOM_SPEED);
            camera.zoom =
                (camera.zoom * zoom_factor).clamp(Config::CAMERA_MIN_ZOOM, Config::CAMERA_MAX_ZOOM);
        }

        // Update camera (handles smoothing and bounds)
        camera.update(dt, false);
    }

    fn handle_spawn_input(&self, game: &mut GameState) {
        if !game.can_spawn() {
            return;
        }

        let unit_type = if is_key_pressed(KeyCode::Key1) {
            Some(UnitType::InfantryLight)
        } else if is_key_pressed(KeyCode::Key2) {
            Some(UnitType::InfantryHeavy)
        } else if is_key_pressed(KeyCode::Key3) {
            Some(UnitType::ArmorLight)
        } else if is_key_pressed(KeyCode::Key4) {
            Some(UnitType::ArmorHeavy)
        } else if is_key_pressed(KeyCode::Key5) {
            Some(UnitType::Artillery)
        } else if is_key_pressed(KeyCode::Key6) {
            Some(UnitType::DroneScout)
        } else if is_key_pressed(KeyCode::Key7) {
            Some(UnitType::DroneGunship)
        } else if is_key_pressed(KeyCode::Key8) {
            Some(UnitType::Engineer)
        } else if is_key_pressed(KeyCode::Key9) {
            Some(UnitType::AntiArmor)
        } else if is_key_pressed(KeyCode::Key0) {
            Some(UnitType::AntiAir)
        } else {
            None
        };

        if let Some(unit_type) = unit_type {
            println!("Attempting to spawn {:?} for faction {:?}", unit_type, game.local_faction);
            // Spawn a squad of 3 units with Advance order by default
            let result = game.spawn_squad(game.local_faction, unit_type, 3, SquadOrder::Advance);

            if let Err(err) = result {
                println!("✗ Spawn failed: {}", err);
            }
        }
    }

    fn handle_doctrine_input(&self, game: &mut GameState) {
        if is_key_pressed(KeyCode::F1) {
            // Cycle through doctrines for slot 0
            let current = game.get_local_commander().active_doctrines[0];
            let next = match current {
                None => Some(Doctrine::AggressivePosture),
                Some(Doctrine::AggressivePosture) => Some(Doctrine::EntrenchedAssault),
                Some(Doctrine::EntrenchedAssault) => Some(Doctrine::ShockWindows),
                Some(Doctrine::ShockWindows) => Some(Doctrine::RapidDeployment),
                Some(Doctrine::RapidDeployment) => Some(Doctrine::ResourceEfficiency),
                Some(Doctrine::ResourceEfficiency) => None,
            };
            game.set_doctrine(game.local_faction, 0, next);
            if let Some(d) = next {
                println!("Doctrine 1: {:?}", d);
            } else {
                println!("Doctrine 1: None");
            }
        }

        if is_key_pressed(KeyCode::F2) {
            // Cycle through doctrines for slot 1
            let current = game.get_local_commander().active_doctrines[1];
            let next = match current {
                None => Some(Doctrine::AggressivePosture),
                Some(Doctrine::AggressivePosture) => Some(Doctrine::EntrenchedAssault),
                Some(Doctrine::EntrenchedAssault) => Some(Doctrine::ShockWindows),
                Some(Doctrine::ShockWindows) => Some(Doctrine::RapidDeployment),
                Some(Doctrine::RapidDeployment) => Some(Doctrine::ResourceEfficiency),
                Some(Doctrine::ResourceEfficiency) => None,
            };
            game.set_doctrine(game.local_faction, 1, next);
            if let Some(d) = next {
                println!("Doctrine 2: {:?}", d);
            } else {
                println!("Doctrine 2: None");
            }
        }
    }

    fn handle_squad_input(&mut self, game: &mut GameState, camera: &Camera2D) {
        // Change order for selected squad
        if self.selected_squad.is_some() {
            if is_key_pressed(KeyCode::Q) {
                self.cycle_squad_order(game, -1);
            } else if is_key_pressed(KeyCode::E) {
                self.cycle_squad_order(game, 1);
            }
        }

        // Squad selection with mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let world_pos = camera.screen_to_world(vec2(mouse_pos.0, mouse_pos.1));

            // Find clicked squad
            let clicked_squad = game
                .get_local_squads()
                .find(|squad| {
                    squad
                        .units
                        .iter()
                        .any(|unit| unit.position.distance(world_pos) < 15.0)
                })
                .map(|squad| squad.id);

            if let Some(squad_id) = clicked_squad {
                self.selected_squad = Some(squad_id);
                println!("Selected squad {}", squad_id);
            } else {
                self.selected_squad = None;
            }
        }
    }

    fn cycle_squad_order(&self, game: &mut GameState, direction: i32) {
        if let Some(squad_id) = self.selected_squad {
            if let Some(squad) = game.squads.iter().find(|s| s.id == squad_id) {
                let current_order = squad.order;
                let orders = [
                    SquadOrder::Advance,
                    SquadOrder::Hold,
                    SquadOrder::Skirmish,
                    SquadOrder::Fortify,
                ];

                let current_index = orders.iter().position(|&o| o == current_order).unwrap_or(0);
                let new_index = if direction > 0 {
                    (current_index + 1) % orders.len()
                } else {
                    (current_index + orders.len() - 1) % orders.len()
                };

                let new_order = orders[new_index];
                game.change_squad_order(squad_id, new_order);
                println!("Changed squad {} order to {:?}", squad_id, new_order);
            }
        }
    }

    fn handle_marker_input(&self, game: &mut GameState, camera: &Camera2D) {
        // Right-click to place/cycle markers on sectors
        if is_mouse_button_pressed(MouseButton::Right) {
            let mouse_pos = mouse_position();
            let world_pos = camera.screen_to_world(vec2(mouse_pos.0, mouse_pos.1));

            // Find clicked sector (store ID to avoid borrow issues)
            let clicked_sector_id = game.sectors.iter()
                .find(|sector| sector.contains_point(world_pos))
                .map(|sector| sector.id);

            if let Some(sector_id) = clicked_sector_id {
                game.toggle_marker(sector_id);

                // Print marker status
                if let Some(marker) = game.get_marker(sector_id, game.local_faction) {
                    let marker_name = match marker.marker_type {
                        crate::types::MarkerType::Attack => "ATTACK",
                        crate::types::MarkerType::Defend => "DEFEND",
                    };
                    println!("Marked sector {} as {}", sector_id, marker_name);
                } else {
                    println!("Removed marker from sector {}", sector_id);
                }
            }
        }
    }
}
