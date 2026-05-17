//! Combat resolution and damage calculation

use crate::config::Config;
use crate::types::*;

impl super::Simulation {
    /// Resolve combat between units
    /// This processes attacks from the given commander's units only
    /// Defense modifiers are applied based on the defender's doctrines
    /// Sector bonuses are applied based on unit positions
    /// Returns hit effects for visual feedback
    pub fn resolve_combat(
        &mut self,
        squads: &mut [Squad],
        commanders: &[Commander; 3],
        sectors: &[crate::types::Sector],
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
                                + (macroquad_toolkit::rng::rand() - 0.5)
                                    * 2.0
                                    * Config::DAMAGE_VARIANCE;
                            let mut damage = unit.unit_type.attack_damage() * variance;

                            // Apply counter bonus (Phase 4 counter system)
                            let counter_mult = unit.unit_type.counter_bonus(target.unit_type);
                            damage *= counter_mult;

                            // Apply armor reduction (Phase 4 armor system)
                            let armor = target.unit_type.armor();
                            damage = (damage - armor).max(damage * 0.2); // Armor reduces damage, minimum 20% gets through

                            // Apply attacker's doctrine modifiers
                            if attacker_commander.has_doctrine(Doctrine::ShockWindows)
                                && unit.is_fresh_spawn(game_time)
                            {
                                // Shock damage bonus for freshly spawned units
                                damage *= Config::SHOCK_DAMAGE_BONUS;
                            }

                            // Apply doctrine synergy/conflict modifier (Phase 4)
                            damage *= attacker_commander.doctrine_modifier();

                            // Apply sector bonuses (Phase 4)
                            // HighGround: +15% damage if attacker is in high ground sector
                            let in_high_ground = sectors.iter().any(|s| {
                                s.contains_point(unit.position)
                                    && s.sector_type == SectorType::HighGround
                            });
                            if in_high_ground {
                                damage *= 1.15;
                            }

                            // Apply morale modifier (Phase 4)
                            damage *= squad.morale_modifier();

                            // Apply commander type passive ability (Phase 4)
                            damage *= attacker_commander.commander_type.damage_bonus();

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
                        let mut defense_mult =
                            if defender_commander.has_doctrine(Doctrine::EntrenchedAssault) {
                                1.0 / Config::ENTRENCHED_DEFENSE_MULT
                            } else if defender_commander.has_doctrine(Doctrine::AggressivePosture) {
                                1.0 / Config::AGGRESSIVE_DAMAGE_MULT
                            } else {
                                1.0
                            };

                        // Apply sector bonuses (Phase 4)
                        // Fortified: +20% defense if defender is in fortified sector
                        let in_fortified = sectors.iter().any(|s| {
                            s.contains_point(unit.position)
                                && s.sector_type == SectorType::Fortified
                        });
                        if in_fortified {
                            defense_mult *= 0.8; // 20% damage reduction
                        }

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
