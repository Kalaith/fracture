use super::{RtsGameState, UnitCommand};
use macroquad::prelude::Vec2;

impl RtsGameState {
    pub(super) fn update_attack_cooldowns(&mut self, dt: f32) {
        for unit in &mut self.units {
            unit.attack_cooldown_remaining = (unit.attack_cooldown_remaining - dt).max(0.0);
        }
    }

    pub(super) fn update_movement(&mut self, dt: f32) {
        let movement_orders: Vec<(usize, Vec2, bool)> = self
            .units
            .iter()
            .enumerate()
            .filter_map(|(unit_index, unit)| {
                let stats = self.unit_stats_for_player(unit.owner, unit.kind);
                match unit.command {
                    UnitCommand::Move(target) => Some((unit_index, target, true)),
                    UnitCommand::AttackMove(target) => {
                        if self
                            .find_auto_attack_target(unit.owner, unit.position, stats.attack_range)
                            .is_some()
                        {
                            None
                        } else {
                            Some((unit_index, target, true))
                        }
                    }
                    UnitCommand::AttackUnit(target_id) => {
                        let target_position = self.unit(target_id)?.position;
                        if unit.position.distance(target_position) <= stats.attack_range * 0.9 {
                            None
                        } else {
                            Some((unit_index, target_position, false))
                        }
                    }
                    UnitCommand::AttackBuilding(target_id) => {
                        let target_position = self.building(target_id)?.position;
                        if unit.position.distance(target_position) <= stats.attack_range * 0.9 {
                            None
                        } else {
                            Some((unit_index, target_position, false))
                        }
                    }
                    UnitCommand::Idle | UnitCommand::GatherMatter(_) | UnitCommand::Build(_) => {
                        None
                    }
                }
            })
            .collect();

        for (unit_index, target, stop_when_reached) in movement_orders {
            let owner = self.units[unit_index].owner;
            let kind = self.units[unit_index].kind;
            let speed = self.unit_stats_for_player(owner, kind).movement_speed;
            let reached = move_toward(&mut self.units[unit_index].position, target, speed * dt);

            if reached && stop_when_reached {
                self.units[unit_index].command = UnitCommand::Idle;
            }
        }
    }
}

fn move_toward(position: &mut Vec2, target: Vec2, max_distance: f32) -> bool {
    let offset = target - *position;
    let distance = offset.length();

    if distance <= max_distance || distance <= f32::EPSILON {
        *position = target;
        true
    } else {
        *position += offset.normalize() * max_distance;
        false
    }
}
