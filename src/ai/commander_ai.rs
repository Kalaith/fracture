//! AI Commander - makes strategic decisions for a single AI commander

use super::personality::AIPersonality;
use super::strategic::StrategicAnalysis;
use crate::game::GameState;
use crate::types::*;

/// AI Commander - controls a single AI-controlled commander
pub struct CommanderAI {
    pub faction_id: FactionId,
    pub personality: AIPersonality,
    pub decision_timer: f32,
    pub spawn_timer: f32,
    pub last_analysis: Option<StrategicAnalysis>,
}

impl CommanderAI {
    pub fn new(faction_id: FactionId, personality: AIPersonality) -> Self {
        Self {
            faction_id,
            personality,
            decision_timer: 0.0,
            spawn_timer: 0.0,
            last_analysis: None,
        }
    }

    /// Update AI decision-making
    pub fn update(&mut self, game: &mut GameState, dt: f32) {
        self.decision_timer += dt;
        self.spawn_timer += dt;

        // Make decisions at intervals based on personality
        if self.decision_timer >= self.personality.decision_timing.reaction_delay {
            self.decision_timer = 0.0;
            self.make_decisions(game);
        }

        // Spawn units based on personality
        let spawn_interval = 2.0 / self.personality.spawn_frequency.max(0.1);
        if self.spawn_timer >= spawn_interval && game.can_spawn() {
            self.spawn_timer = 0.0;
            self.spawn_units(game);
        }
    }

    /// Main decision-making logic
    fn make_decisions(&mut self, game: &mut GameState) {
        // Analyze battlefield
        let analysis = StrategicAnalysis::analyze(game, self.faction_id);

        // Decide whether to change doctrines
        self.update_doctrines(game, &analysis);

        // Decide squad orders based on strategic situation
        self.update_squad_orders(game, &analysis);

        // Store analysis for next decision cycle
        self.last_analysis = Some(analysis);
    }

    /// Update active doctrines based on situation
    fn update_doctrines(&self, game: &mut GameState, analysis: &StrategicAnalysis) {
        // Only change doctrines occasionally (low confidence personalities change more)
        if macroquad_toolkit::rng::rand() > self.personality.decision_timing.decision_confidence {
            return;
        }

        // Choose doctrines based on personality and situation
        let should_be_aggressive = analysis.should_expand(self.faction_id);
        let should_be_defensive = analysis.should_defend(self.faction_id);

        // Slot 0: Offense vs Defense
        if should_be_aggressive && self.personality.behavior_weights.expand_territory > 0.6 {
            game.set_doctrine(self.faction_id, 0, Some(Doctrine::AggressivePosture));
        } else if should_be_defensive && self.personality.behavior_weights.defend_position > 0.6 {
            game.set_doctrine(self.faction_id, 0, Some(Doctrine::EntrenchedAssault));
        }

        // Slot 1: Economy or Special
        if analysis.supply_advantage[self.faction_id.index()] < 0.3 {
            // Low on supply
            game.set_doctrine(self.faction_id, 1, Some(Doctrine::ResourceEfficiency));
        } else if self.personality.behavior_weights.harass_enemy > 0.7 {
            game.set_doctrine(self.faction_id, 1, Some(Doctrine::ShockWindows));
        } else {
            game.set_doctrine(self.faction_id, 1, Some(Doctrine::RapidDeployment));
        }
    }

    /// Update squad orders based on strategic analysis
    fn update_squad_orders(&self, game: &mut GameState, analysis: &StrategicAnalysis) {
        // Find our squads
        let our_squads: Vec<u32> = game
            .squads
            .iter()
            .filter(|s| s.owner == self.faction_id)
            .map(|s| s.id)
            .collect();

        for squad_id in our_squads {
            let order = self.decide_squad_order(game, squad_id, analysis);
            game.change_squad_order(squad_id, order);
        }
    }

    /// Decide order for a specific squad
    fn decide_squad_order(
        &self,
        game: &GameState,
        squad_id: u32,
        analysis: &StrategicAnalysis,
    ) -> SquadOrder {
        let squad = game.squads.iter().find(|s| s.id == squad_id);
        if squad.is_none() {
            return SquadOrder::Hold;
        }
        let squad = squad.unwrap();

        // Calculate squad health percentage
        let avg_health: f32 = squad.units.iter().map(|u| u.health_percent()).sum::<f32>()
            / squad.units.len().max(1) as f32;

        // Defensive behavior if squad is weak
        if avg_health < self.personality.combat_behavior.retreat_health_threshold {
            return SquadOrder::Hold;
        }

        // Strategic decision based on personality
        // Strategic decision based on personality
        // Use deterministic checks for high-priority behaviors to prevent jitter
        // If the situation calls for it AND the personality supports it strongly, do it reliably.
        if analysis.should_expand(self.faction_id)
            && self.personality.behavior_weights.expand_territory > 0.6
        {
            SquadOrder::Advance
        } else if analysis.should_defend(self.faction_id)
            && self.personality.behavior_weights.defend_position > 0.6
        {
            SquadOrder::Fortify
        } else if self.personality.should_perform("harass") {
            SquadOrder::Skirmish
        } else {
            SquadOrder::Hold
        }
    }

    /// Spawn new units based on personality preferences
    fn spawn_units(&self, game: &mut GameState) {
        let commander = game.get_commander(self.faction_id);

        // Don't spawn if we can't afford it
        if commander.available_supply() < 10 {
            return;
        }

        // Choose unit type based on preferred types
        let unit_type = self.choose_unit_type(game);

        // Choose initial order based on personality
        let order = if self.personality.behavior_weights.expand_territory > 0.7 {
            SquadOrder::Advance
        } else if self.personality.behavior_weights.defend_position > 0.7 {
            SquadOrder::Fortify
        } else {
            SquadOrder::Hold
        };

        // Spawn squad of 3 units
        let _ = game.spawn_squad(self.faction_id, unit_type, 3, order);
    }

    /// Choose which unit type to spawn based on preferences
    fn choose_unit_type(&self, _game: &GameState) -> UnitType {
        // Simple heuristic: choose randomly from base types
        // In full implementation, would use preferred_unit_types from personality
        let choice = macroquad_toolkit::rng::rand();

        if choice < 0.3 {
            UnitType::InfantryLight
        } else if choice < 0.5 {
            UnitType::InfantryHeavy
        } else if choice < 0.65 {
            UnitType::ArmorLight
        } else if choice < 0.75 {
            UnitType::DroneScout
        } else if choice < 0.85 {
            UnitType::DroneGunship
        } else {
            UnitType::Artillery
        }
    }
}
