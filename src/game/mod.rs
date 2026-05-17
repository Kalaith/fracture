//! Main game state and loop coordination
//!
//! This module owns the authoritative game state and orchestrates updates across
//! all game systems (simulation, sectors, victory conditions). It provides the
//! central API for spawning units, changing orders, and managing commanders.

use crate::config::{get_sector_positions, get_spawn_position, Config};
use crate::simulation::Simulation;
use crate::types::*;
use macroquad::prelude::*;

/// Game state - central container for all game data
pub struct GameState {
    pub local_faction: FactionId,
    pub commanders: [Commander; 3], // One per faction
    pub squads: Vec<Squad>,
    pub sectors: Vec<Sector>,
    pub simulation: Simulation,
    pub victory_timer: f32,
    pub game_time: f32, // Total elapsed game time (for ShockWindows doctrine)
    pub game_over: bool,
    pub winner: Option<FactionId>,
    pub combat_effects: Vec<CombatEffect>,
    pub paused: bool,
    pub game_speed: f32,               // 1.0 = normal, 0.5 = slow, 2.0 = fast
    pub markers: Vec<StrategicMarker>, // Player-placed attack/defend markers
    pub units_spawned: [u32; 3],       // Track units spawned per faction (Phase 4)
    pub units_lost: [u32; 3],          // Track units lost per faction (Phase 4)
}

impl GameState {
    pub fn new(local_faction: FactionId) -> Self {
        // Create commanders for all 3 factions
        // Faction1 is player-controlled, Faction2 and Faction3 are AI
        let commanders = [
            Commander::new(
                FactionId::Faction1,
                CommanderType::Vanguard,
                Config::SUPPLY_MAX,
                local_faction == FactionId::Faction1,
            ),
            Commander::new(
                FactionId::Faction2,
                CommanderType::Control,
                Config::SUPPLY_MAX,
                local_faction == FactionId::Faction2,
            ),
            Commander::new(
                FactionId::Faction3,
                CommanderType::Disruptor,
                Config::SUPPLY_MAX,
                local_faction == FactionId::Faction3,
            ),
        ];

        // Create sectors with different types for strategic variety
        let sector_positions = get_sector_positions();
        let sector_types = [
            SectorType::Standard,
            SectorType::SupplyDepot,
            SectorType::HighGround,
            SectorType::Fortified,
            SectorType::Industrial,
        ];
        let sectors = sector_positions
            .iter()
            .enumerate()
            .map(|(i, pos)| {
                let sector_type = sector_types[i % sector_types.len()];
                Sector::new(i as u32, *pos, Config::SECTOR_RADIUS).with_type(sector_type)
            })
            .collect();

        Self {
            local_faction,
            commanders,
            squads: Vec::new(),
            sectors,
            simulation: Simulation::new(),
            victory_timer: 0.0,
            game_time: 0.0,
            game_over: false,
            winner: None,
            combat_effects: Vec::new(),
            paused: false,
            game_speed: 1.0,
            markers: Vec::new(),
            units_spawned: [0, 0, 0],
            units_lost: [0, 0, 0],
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.game_over || self.paused {
            return;
        }

        // Apply game speed
        let dt = dt * self.game_speed;

        // Update timers
        // Spawn timer no longer used - removed for instant spawning
        // for commander in &mut self.commanders {
        //     commander.spawn_timer += dt;
        // }
        self.game_time += dt;

        // Update simulation (AI and movement)
        self.simulation
            .update_squads(&mut self.squads, &self.sectors, &self.commanders, dt);

        // Resolve combat for all factions and collect hit effects
        for faction_id in FactionId::all() {
            let effects = self.simulation.resolve_combat(
                &mut self.squads,
                &self.commanders,
                &self.sectors,
                faction_id,
                self.game_time,
                dt,
            );
            self.combat_effects.extend(effects);
        }

        // Update sectors
        self.simulation
            .update_sectors(&mut self.sectors, &self.squads, dt);

        // Update combat effects
        self.combat_effects.retain_mut(|effect| effect.update(dt));

        // Detect killed units and create effects
        for squad in &self.squads {
            for unit in &squad.units {
                if !unit.is_alive() && unit.health < 0.0 && unit.health > -10.0 {
                    // Just died, create death effect
                    self.combat_effects
                        .push(CombatEffect::new_kill(unit.position));
                    // Track units lost (Phase 4)
                    self.units_lost[squad.owner.index()] += 1;
                }
            }
        }

        // Clean up empty squads
        self.squads.retain(|s| !s.is_empty());

        // Update morale (Phase 4)
        self.update_morale(dt);

        // Update supply counts
        self.update_supply_counts();

        // Check victory conditions
        self.check_victory(dt);
    }

    fn update_morale(&mut self, dt: f32) {
        for squad in &mut self.squads {
            // Get commander type morale recovery bonus (Phase 4)
            let commander = &self.commanders[squad.owner.index()];
            let recovery_bonus = commander.commander_type.morale_recovery_bonus();

            // Morale recovers slowly over time (10% per 10 seconds, modified by commander type)
            squad.morale = (squad.morale + dt * 0.01 * recovery_bonus).min(1.0);

            // Morale drops if squad is heavily damaged
            if !squad.units.is_empty() {
                let avg_health: f32 = squad.units.iter().map(|u| u.health_percent()).sum::<f32>()
                    / squad.units.len() as f32;

                // Low average health reduces morale
                if avg_health < 0.3 {
                    squad.morale = (squad.morale - dt * 0.05).max(0.0);
                } else if avg_health < 0.5 {
                    squad.morale = (squad.morale - dt * 0.02).max(0.0);
                }
            }
        }
    }

    fn update_supply_counts(&mut self) {
        // Reset supply counts and max (will add sector bonuses)
        for commander in &mut self.commanders {
            commander.supply_used = 0;
            commander.supply_max = Config::SUPPLY_MAX; // Reset to base
        }

        // Count supply usage per faction
        let mut faction_supply = [0; 3];

        for squad in &self.squads {
            for unit in &squad.units {
                faction_supply[squad.owner.index()] += unit.unit_type.supply_cost();
            }
        }

        // Apply sector bonuses (Supply Depot sectors give +20 max supply)
        for sector in &self.sectors {
            if let Some(owner) = sector.control {
                if sector.sector_type == SectorType::SupplyDepot {
                    self.commanders[owner.index()].supply_max += 20;
                }
            }
        }

        // Update commanders
        for faction in FactionId::all() {
            self.commanders[faction.index()].supply_used = faction_supply[faction.index()];
        }
    }

    fn check_victory(&mut self, dt: f32) {
        // Count sectors per faction
        let mut faction_sectors = [0; 3];
        for sector in &self.sectors {
            if let Some(faction) = sector.control {
                faction_sectors[faction.index()] += 1;
            }
        }

        // Check if any faction has enough sectors for victory
        for faction in FactionId::all() {
            let sector_count = faction_sectors[faction.index()];
            if sector_count >= Config::VICTORY_SECTOR_CONTROL as usize {
                self.victory_timer += dt;
                if self.victory_timer >= Config::VICTORY_TIME_REQUIRED {
                    self.game_over = true;
                    self.winner = Some(faction);
                }
                return;
            }
        }

        // No faction is winning, reset timer
        self.victory_timer = 0.0;
    }

    pub fn spawn_squad(
        &mut self,
        owner: FactionId,
        unit_type: UnitType,
        count: u32,
        order: SquadOrder,
    ) -> Result<(), String> {
        let commander = self.get_commander(owner);
        let total_cost = unit_type.supply_cost() * count;

        if !commander.can_afford(total_cost) {
            return Err(format!(
                "Not enough supply: need {}, have {}",
                total_cost,
                commander.available_supply()
            ));
        }

        // Apply doctrine cost reduction
        let actual_cost = if commander.has_doctrine(Doctrine::ResourceEfficiency) {
            (total_cost as f32 * Config::EFFICIENCY_COST_MULT) as u32
        } else {
            total_cost
        };

        // Create squad
        let squad_id = self.simulation.next_squad_id();
        let spawn_pos = get_spawn_position(owner);
        let mut squad = Squad::new(squad_id, owner, order, spawn_pos);

        // Create units
        for i in 0..count {
            let unit_id = self.simulation.next_unit_id();
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let offset = vec2(angle.cos(), angle.sin()) * Config::SPAWN_OFFSET;
            let mut unit = Unit::new(unit_id, unit_type, spawn_pos + offset, owner);
            unit.spawn_time = self.game_time; // Set spawn time for ShockWindows doctrine
            squad.add_unit(unit);
        }

        self.squads.push(squad);

        // Track units spawned (Phase 4)
        self.units_spawned[owner.index()] += count;

        // Deduct supply and reset spawn timer
        let commander = self.get_commander_mut(owner);
        commander.supply_used += actual_cost;

        commander.spawn_timer = 0.0;

        println!("✓ Spawned {} {:?} units for {:?}", count, unit_type, owner);
        Ok(())
    }

    pub fn change_squad_order(&mut self, squad_id: u32, new_order: SquadOrder) {
        if let Some(squad) = self.squads.iter_mut().find(|s| s.id == squad_id) {
            squad.order = new_order;
        }
    }

    pub fn set_doctrine(&mut self, faction_id: FactionId, slot: usize, doctrine: Option<Doctrine>) {
        self.get_commander_mut(faction_id)
            .set_doctrine(slot, doctrine);
    }

    pub fn get_commander(&self, faction_id: FactionId) -> &Commander {
        &self.commanders[faction_id.index()]
    }

    pub fn get_commander_mut(&mut self, faction_id: FactionId) -> &mut Commander {
        &mut self.commanders[faction_id.index()]
    }

    pub fn get_local_commander(&self) -> &Commander {
        self.get_commander(self.local_faction)
    }

    pub fn get_local_commander_mut(&mut self) -> &mut Commander {
        self.get_commander_mut(self.local_faction)
    }

    pub fn get_local_squads(&self) -> impl Iterator<Item = &Squad> {
        self.squads
            .iter()
            .filter(move |s| s.owner == self.local_faction)
    }

    pub fn can_spawn(&self) -> bool {
        true
    }

    pub fn spawn_cooldown_percent(&self) -> f32 {
        let local_commander = &self.commanders[self.local_faction.index()];
        (local_commander.spawn_timer / Config::SPAWN_COOLDOWN).min(1.0)
    }

    /// Toggle marker on a sector (cycles: None -> Attack -> Defend -> None)
    pub fn toggle_marker(&mut self, sector_id: u32) {
        // Find existing marker for this sector and faction
        if let Some(pos) = self
            .markers
            .iter()
            .position(|m| m.sector_id == sector_id && m.owner == self.local_faction)
        {
            let current_type = self.markers[pos].marker_type;
            self.markers.remove(pos);

            // Cycle to next type
            match current_type {
                MarkerType::Attack => {
                    self.markers.push(StrategicMarker::new(
                        sector_id,
                        MarkerType::Defend,
                        self.local_faction,
                    ));
                }
                MarkerType::Defend => {
                    // Remove marker (already removed above)
                }
            }
        } else {
            // No marker, add attack marker
            self.markers.push(StrategicMarker::new(
                sector_id,
                MarkerType::Attack,
                self.local_faction,
            ));
        }
    }

    /// Get marker for a sector (if any)
    pub fn get_marker(&self, sector_id: u32, faction: FactionId) -> Option<&StrategicMarker> {
        self.markers
            .iter()
            .find(|m| m.sector_id == sector_id && m.owner == faction)
    }

    /// Get the sector at a specific position (if any)
    pub fn get_sector_at(&self, position: Vec2) -> Option<&Sector> {
        self.sectors.iter().find(|s| s.contains_point(position))
    }

    /// Check if a position is in a sector with a specific type
    pub fn is_in_sector_type(&self, position: Vec2, sector_type: SectorType) -> bool {
        self.get_sector_at(position)
            .map(|s| s.sector_type == sector_type)
            .unwrap_or(false)
    }
}
