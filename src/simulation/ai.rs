//! Autonomous unit AI - squad behavior based on orders

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
            SquadOrder::Hold => self.ai_hold(unit, all_squads, sectors),
            SquadOrder::Skirmish => self.ai_skirmish(unit, all_squads, sectors),
            SquadOrder::Fortify => self.ai_fortify(unit, all_squads, sectors),
        }
    }

    pub(super) fn ai_advance(&mut self, unit: &mut Unit, all_squads: &[Squad], sectors: &[Sector]) {
        // Priority 1: Find target sector (unclaimed first, then enemy)
        let unclaimed_sector = sectors
            .iter()
            .filter(|s| s.control.is_none())
            .min_by_key(|s| (s.position.distance(unit.position) * 100.0) as i32);

        let target_sector = if let Some(sector) = unclaimed_sector {
            Some(sector)
        } else {
            // No unclaimed sectors, target nearest enemy sector
            sectors
                .iter()
                .filter(|s| s.control.is_some() && s.control != Some(unit.owner))
                .min_by_key(|s| (s.position.distance(unit.position) * 100.0) as i32)
        };

        // Priority 2: Check for enemies to engage
        if let Some(enemy_id) = self.find_nearest_enemy(unit, all_squads, 400.0) {
            let enemy_pos = all_squads
                .iter()
                .flat_map(|s| &s.units)
                .find(|u| u.id == enemy_id)
                .map(|u| u.position);

            if let Some(pos) = enemy_pos {
                let dist = unit.position.distance(pos);
                let attack_range = unit.unit_type.attack_range();

                if dist > attack_range * 0.9 {
                    // Enemy out of range, move toward it
                    unit.target_enemy = Some(enemy_id);
                    let direction = (pos - unit.position).normalize_or_zero();
                    unit.target_position = Some(pos - direction * (attack_range * 0.8));
                } else {
                    // Enemy in range, stop and attack
                    unit.target_enemy = Some(enemy_id);
                    unit.target_position = None;
                }
                return;
            }
        }

        // Priority 3: No enemies, move to sector
        if let Some(sector) = target_sector {
            unit.target_position = Some(sector.position);
            unit.target_enemy = None;
        }
    }

    pub(super) fn ai_hold(&mut self, unit: &mut Unit, all_squads: &[Squad], sectors: &[Sector]) {
        // Use same logic as advance - all units should be active
        self.ai_advance(unit, all_squads, sectors);
    }

    pub(super) fn ai_skirmish(
        &mut self,
        unit: &mut Unit,
        all_squads: &[Squad],
        sectors: &[Sector],
    ) {
        // Use same logic as advance - all units should be active
        self.ai_advance(unit, all_squads, sectors);
    }

    pub(super) fn ai_fortify(&mut self, unit: &mut Unit, all_squads: &[Squad], sectors: &[Sector]) {
        // Use same logic as advance - all units should be active
        self.ai_advance(unit, all_squads, sectors);
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
}
