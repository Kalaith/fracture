use super::{EntityId, RtsGameState, UnitCommand, UnitInstance};
use macroquad::prelude::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CombatTarget {
    Unit(EntityId),
    Building(EntityId),
}

impl RtsGameState {
    pub(super) fn update_combat(&mut self) {
        let mut attacks = Vec::new();

        for unit_index in 0..self.units.len() {
            let unit = &self.units[unit_index];
            let stats = self.unit_stats_for_player(unit.owner, unit.kind);

            if stats.attack_damage == 0 || unit.attack_cooldown_remaining > 0.0 {
                continue;
            }

            let Some(target) = self.find_attack_target_for_unit(unit, stats.attack_range) else {
                continue;
            };

            attacks.push((unit_index, target, stats.attack_damage));
        }

        for (unit_index, target, damage) in attacks {
            let owner = self.units[unit_index].owner;
            let kind = self.units[unit_index].kind;
            let cooldown = self.unit_stats_for_player(owner, kind).attack_cooldown;
            self.units[unit_index].attack_cooldown_remaining = cooldown;

            match target {
                CombatTarget::Unit(target_id) => self.damage_unit(target_id, damage),
                CombatTarget::Building(target_id) => self.damage_building(target_id, damage),
            }
        }
    }

    fn find_attack_target_for_unit(&self, unit: &UnitInstance, range: f32) -> Option<CombatTarget> {
        match unit.command {
            UnitCommand::AttackUnit(target_id) => {
                let target = self.unit(target_id)?;
                if target.owner != unit.owner && unit.position.distance(target.position) <= range {
                    return Some(CombatTarget::Unit(target_id));
                }
            }
            UnitCommand::AttackBuilding(target_id) => {
                let target = self.building(target_id)?;
                if target.owner != unit.owner && unit.position.distance(target.position) <= range {
                    return Some(CombatTarget::Building(target_id));
                }
            }
            UnitCommand::Idle | UnitCommand::AttackMove(_) => {}
            UnitCommand::Move(_) | UnitCommand::GatherMatter(_) | UnitCommand::Build(_) => {
                return None;
            }
        }

        self.find_auto_attack_target(unit.owner, unit.position, range)
    }

    pub(super) fn find_auto_attack_target(
        &self,
        owner: usize,
        position: Vec2,
        range: f32,
    ) -> Option<CombatTarget> {
        let closest_unit = self
            .units
            .iter()
            .filter(|unit| unit.owner != owner && position.distance(unit.position) <= range)
            .min_by(|left, right| {
                position
                    .distance(left.position)
                    .total_cmp(&position.distance(right.position))
            })
            .map(|unit| CombatTarget::Unit(unit.id));

        if closest_unit.is_some() {
            return closest_unit;
        }

        self.buildings
            .iter()
            .filter(|building| {
                building.owner != owner && position.distance(building.position) <= range
            })
            .min_by(|left, right| {
                position
                    .distance(left.position)
                    .total_cmp(&position.distance(right.position))
            })
            .map(|building| CombatTarget::Building(building.id))
    }

    fn damage_unit(&mut self, target_id: EntityId, damage: u32) {
        let Some(target_index) = self.units.iter().position(|unit| unit.id == target_id) else {
            return;
        };

        let owner = self.units[target_index].owner;
        let kind = self.units[target_index].kind;
        let armor = self.unit_stats_for_player(owner, kind).armor;
        let actual_damage = damage.saturating_sub(armor).max(1) as f32;
        self.units[target_index].health -= actual_damage;
    }

    fn damage_building(&mut self, target_id: EntityId, damage: u32) {
        let Some(target) = self.building_mut(target_id) else {
            return;
        };

        let actual_damage = damage.saturating_sub(target.kind.armor()).max(1) as f32;
        target.health -= actual_damage;
    }
}
