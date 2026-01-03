//! Game simulation - autonomous unit AI, combat resolution, and sector control
//!
//! This module implements the core simulation logic including:
//! - Autonomous squad AI based on orders (Advance, Hold, Skirmish, Fortify)
//! - Combat resolution with doctrine modifiers
//! - Sector capture mechanics
//!
//! All simulation logic must be deterministic for future network synchronization.

mod ai;
mod combat;

use crate::config::Config;
use crate::types::*;
use macroquad::prelude::*;

/// Simulation system - handles autonomous unit behavior and combat
pub struct Simulation {
    next_unit_id: u32,
    next_squad_id: u32,
    ai_timer: f32,
    pub(self) attack_timers: std::collections::HashMap<u32, f32>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            next_unit_id: 0,
            next_squad_id: 0,
            ai_timer: 0.0,
            attack_timers: std::collections::HashMap::new(),
        }
    }

    pub fn next_unit_id(&mut self) -> u32 {
        let id = self.next_unit_id;
        self.next_unit_id += 1;
        id
    }

    pub fn next_squad_id(&mut self) -> u32 {
        let id = self.next_squad_id;
        self.next_squad_id += 1;
        id
    }

    /// Update all squads - autonomous behavior based on orders
    pub fn update_squads(
        &mut self,
        squads: &mut [Squad],
        sectors: &[Sector],
        commanders: &[Commander; 3],
        dt: f32,
    ) {
        self.ai_timer += dt;

        // Update AI decisions periodically, not every frame
        let should_update_ai = self.ai_timer >= Config::AI_UPDATE_INTERVAL;
        if should_update_ai {
            self.ai_timer = 0.0;
        }

        // Clone squads for read-only access during AI updates
        let squads_snapshot = squads.to_vec();

        for squad in squads.iter_mut() {
            let commander = &commanders[squad.owner.index()];

            self.update_squad(
                squad,
                &squads_snapshot,
                sectors,
                commander,
                dt,
                should_update_ai,
            );
        }

        // Update attack cooldowns
        for timer in self.attack_timers.values_mut() {
            *timer -= dt;
        }
    }

    fn update_squad(
        &mut self,
        squad: &mut Squad,
        all_squads: &[Squad],
        sectors: &[Sector],
        commander: &Commander,
        dt: f32,
        update_ai: bool,
    ) {
        // Remove dead units
        squad.remove_dead_units();

        // Get speed modifier from doctrines
        let speed_mult = if commander.has_doctrine(Doctrine::AggressivePosture) {
            Config::AGGRESSIVE_SPEED_MULT
        } else if commander.has_doctrine(Doctrine::EntrenchedAssault) {
            Config::ENTRENCHED_SPEED_MULT
        } else {
            1.0
        };

        for unit in squad.units.iter_mut() {
            if update_ai {
                self.update_unit_ai(unit, squad.order, all_squads, sectors);
            }

            // Move unit towards target
            if let Some(target_pos) = unit.target_position {
                let direction = (target_pos - unit.position).normalize_or_zero();
                let move_speed = unit.unit_type.move_speed() * speed_mult;
                unit.position += direction * move_speed * dt;

                // Clear target if reached
                if unit.position.distance(target_pos) < 5.0 {
                    unit.target_position = None;
                }
            }
        }
    }

    /// Update sector control based on unit presence (3-faction system)
    pub fn update_sectors(&self, sectors: &mut [Sector], squads: &[Squad], dt: f32) {
        for sector in sectors.iter_mut() {
            let mut faction_units = [0; 3];

            // Count units per faction in sector
            for squad in squads {
                for unit in &squad.units {
                    if sector.contains_point(unit.position) {
                        faction_units[squad.owner.index()] += 1;
                    }
                }
            }

            // Determine which faction has majority
            let total_units: i32 = faction_units.iter().sum();
            if total_units == 0 {
                // No units, decay toward neutral
                if sector.control_progress.abs() > 0.01 {
                    sector.control_progress *= 1.0 - (Config::SECTOR_CAPTURE_RATE * 0.3 * dt);
                }
                continue;
            }

            let max_units = *faction_units.iter().max().unwrap();
            let dominant_faction = faction_units
                .iter()
                .position(|&count| count == max_units)
                .and_then(FactionId::from_index);

            if let Some(faction) = dominant_faction {
                // Check if this faction has clear majority (more than others combined)
                let others_total: i32 = faction_units
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != faction.index())
                    .map(|(_, count)| count)
                    .sum();

                if faction_units[faction.index()] > others_total {
                    // This faction is capturing
                    let target_progress = match faction {
                        FactionId::Faction1 => -1.0,
                        FactionId::Faction2 => 0.0,
                        FactionId::Faction3 => 1.0,
                    };

                    let diff = target_progress - sector.control_progress;
                    let change = diff.signum() * Config::SECTOR_CAPTURE_RATE * dt;
                    sector.control_progress += change;

                    // Clamp and set control
                    sector.control_progress = sector.control_progress.clamp(-1.0, 1.0);

                    // Update control based on progress thresholds
                    if sector.control_progress < -0.6 {
                        sector.control = Some(FactionId::Faction1);
                    } else if sector.control_progress > 0.6 {
                        sector.control = Some(FactionId::Faction3);
                    } else if sector.control_progress.abs() < 0.3 {
                        sector.control = Some(FactionId::Faction2);
                    }
                }
            }
        }
    }
}
