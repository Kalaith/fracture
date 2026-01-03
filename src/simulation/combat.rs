//! Combat resolution and damage calculation

use crate::config::Config;
use crate::types::*;

impl super::Simulation {
    /// Resolve combat between units
    /// This processes attacks from the given commander's units only
    /// Defense modifiers are applied based on the defender's doctrines
    /// Returns hit effects for visual feedback
    pub fn resolve_combat(
        &mut self,
        squads: &mut [Squad],
        commanders: &[Commander; 3],
        attacker_id: crate::types::FactionId,
        game_time: f32,
        _dt: f32,
    ) -> Vec<crate::types::CombatEffect> {
        let mut damage_queue: Vec<(u32, f32, crate::types::FactionId)> = Vec::new();
        let mut hit_effects = Vec::new();

        // Collect all units with targets from the attacking faction
        for squad in squads.iter() {
            if squad.owner != attacker_id {
                continue;
            }

            let attacker_commander = &commanders[attacker_id.index()];

            for unit in squad.units.iter() {
                if let Some(target_id) = unit.target_enemy {
                    // Check attack cooldown
                    let cooldown = self.attack_timers.get(&unit.id).copied().unwrap_or(0.0);
                    if cooldown > 0.0 {
                        continue;
                    }

                    // Find target and its owner
                    let target_info = squads
                        .iter()
                        .flat_map(|s| s.units.iter().map(move |u| (u, s.owner)))
                        .find(|(u, _)| u.id == target_id);

                    if let Some((target, target_owner)) = target_info {
                        let distance = unit.position.distance(target.position);
                        if distance <= unit.unit_type.attack_range() {
                            // Calculate damage with variance
                            let variance = 1.0
                                + (::rand::random::<f32>() - 0.5) * 2.0 * Config::DAMAGE_VARIANCE;
                            let mut damage = unit.unit_type.attack_damage() * variance;

                            // Apply attacker's doctrine modifiers
                            if attacker_commander.has_doctrine(Doctrine::ShockWindows)
                                && unit.is_fresh_spawn(game_time)
                            {
                                // Shock damage bonus for freshly spawned units
                                damage *= Config::SHOCK_DAMAGE_BONUS;
                            }

                            damage_queue.push((target_id, damage, target_owner));
                            self.attack_timers.insert(unit.id, Config::ATTACK_COOLDOWN);
                        }
                    }
                }
            }
        }

        // Apply damage with defender's doctrine modifiers
        for squad in squads.iter_mut() {
            let defender_commander = &commanders[squad.owner.index()];

            for unit in squad.units.iter_mut() {
                for (target_id, damage, _target_owner) in damage_queue.iter() {
                    if unit.id == *target_id {
                        // Apply defense modifiers based on DEFENDER's doctrines
                        let defense_mult = if defender_commander.has_doctrine(Doctrine::EntrenchedAssault)
                        {
                            1.0 / Config::ENTRENCHED_DEFENSE_MULT
                        } else if defender_commander.has_doctrine(Doctrine::AggressivePosture) {
                            1.0 / Config::AGGRESSIVE_DAMAGE_MULT
                        } else {
                            1.0
                        };

                        let actual_damage = damage * defense_mult;
                        unit.health -= actual_damage;

                        // Create hit effect
                        hit_effects.push(CombatEffect::new_hit(unit.position, actual_damage));
                    }
                }
            }
        }

        hit_effects
    }
}
