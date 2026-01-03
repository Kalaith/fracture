//! Autonomous unit AI - squad behavior based on orders

use crate::config::Config;
use crate::types::*;
use macroquad::prelude::*;

impl super::Simulation {
    /// Update unit AI based on squad order
    pub(super) fn update_unit_ai(
        &mut self,
        unit: &mut Unit,
        order: SquadOrder,
        all_squads: &[Squad],
        sectors: &[Sector],
    ) {
        match order {
            SquadOrder::Advance => self.ai_advance(unit, all_squads, sectors),
            SquadOrder::Hold => self.ai_hold(unit, all_squads),
            SquadOrder::Skirmish => self.ai_skirmish(unit, all_squads),
            SquadOrder::Fortify => self.ai_fortify(unit, all_squads),
        }
    }

    pub(super) fn ai_advance(&mut self, unit: &mut Unit, all_squads: &[Squad], sectors: &[Sector]) {
        // Find nearest uncaptured or enemy sector
        let target_sector = sectors
            .iter()
            .filter(|s| s.control != Some(unit.owner))
            .min_by_key(|s| (s.position.distance(unit.position) * 100.0) as i32);

        if let Some(sector) = target_sector {
            // Check for enemies blocking path (long range detection for advance)
            if let Some(enemy) = self.find_nearest_enemy(unit, all_squads, 400.0) {
                unit.target_enemy = Some(enemy);
                unit.target_position = None;
            } else {
                unit.target_position = Some(sector.position);
                unit.target_enemy = None;
            }
        }
    }

    pub(super) fn ai_hold(&mut self, unit: &mut Unit, all_squads: &[Squad]) {
        // Stay in position, attack nearby enemies
        if let Some(enemy) = self.find_nearest_enemy(unit, all_squads, 300.0) {
            unit.target_enemy = Some(enemy);
        } else {
            unit.target_enemy = None;
        }
        unit.target_position = None;
    }

    pub(super) fn ai_skirmish(&mut self, unit: &mut Unit, all_squads: &[Squad]) {
        // If low health, retreat
        if unit.health_percent() < Config::SKIRMISH_RETREAT_HEALTH {
            // Move away from nearest enemy
            if let Some(enemy_pos) = self.find_nearest_enemy_pos(unit, all_squads) {
                let away = (unit.position - enemy_pos).normalize_or_zero();
                unit.target_position = Some(unit.position + away * 200.0);
                unit.target_enemy = None;
            }
        } else {
            // Attack if healthy (long detection range for skirmishers)
            if let Some(enemy) = self.find_nearest_enemy(unit, all_squads, 350.0) {
                unit.target_enemy = Some(enemy);
                unit.target_position = None;
            } else {
                // Move to find enemies
                let angle = ::rand::random::<f32>() * std::f32::consts::TAU;
                let offset = vec2(angle.cos(), angle.sin()) * 150.0;
                unit.target_position = Some(unit.position + offset);
            }
        }
    }

    pub(super) fn ai_fortify(&mut self, unit: &mut Unit, all_squads: &[Squad]) {
        // Stay put, but still defend by attacking nearby enemies
        unit.target_position = None;
        if let Some(enemy) = self.find_nearest_enemy(unit, all_squads, 300.0) {
            unit.target_enemy = Some(enemy);
        } else {
            unit.target_enemy = None;
        }
    }

    pub(super) fn find_nearest_enemy(
        &self,
        unit: &Unit,
        all_squads: &[Squad],
        max_range: f32,
    ) -> Option<u32> {
        all_squads
            .iter()
            .filter(|s| s.owner != unit.owner)
            .flat_map(|s| &s.units)
            .filter(|u| u.is_alive())
            .filter(|u| u.position.distance(unit.position) <= max_range)
            .min_by_key(|u| (u.position.distance(unit.position) * 100.0) as i32)
            .map(|u| u.id)
    }

    pub(super) fn find_nearest_enemy_pos(&self, unit: &Unit, all_squads: &[Squad]) -> Option<Vec2> {
        all_squads
            .iter()
            .filter(|s| s.owner != unit.owner)
            .flat_map(|s| &s.units)
            .filter(|u| u.is_alive())
            .min_by_key(|u| (u.position.distance(unit.position) * 100.0) as i32)
            .map(|u| u.position)
    }
}
