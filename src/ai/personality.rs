//! AI personality system - defines behavioral patterns for AI commanders

use crate::data::AIPersonalityDefinition;
use std::collections::HashMap;

/// AI personality - drives decision-making behavior
#[derive(Debug, Clone)]
pub struct AIPersonality {
    pub name: String,
    pub behavior_weights: BehaviorWeights,
    pub doctrine_preferences: DoctrinePreferences,
    pub decision_timing: DecisionTiming,
    pub combat_behavior: CombatBehavior,
    pub preferred_unit_types: Vec<String>,
    pub spawn_frequency: f32,
    pub ability_usage: AbilityUsage,
}

#[derive(Debug, Clone)]
pub struct BehaviorWeights {
    pub expand_territory: f32,
    pub defend_position: f32,
    pub build_economy: f32,
    pub harass_enemy: f32,
    pub consolidate_forces: f32,
    pub capture_objectives: f32,
    pub assist_ally: f32,
}

impl BehaviorWeights {
    fn from_map(map: &HashMap<String, f32>) -> Self {
        Self {
            expand_territory: *map.get("expand_territory").unwrap_or(&0.5),
            defend_position: *map.get("defend_position").unwrap_or(&0.5),
            build_economy: *map.get("build_economy").unwrap_or(&0.5),
            harass_enemy: *map.get("harass_enemy").unwrap_or(&0.5),
            consolidate_forces: *map.get("consolidate_forces").unwrap_or(&0.5),
            capture_objectives: *map.get("capture_objectives").unwrap_or(&0.7),
            assist_ally: *map.get("assist_ally").unwrap_or(&0.5),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DoctrinePreferences {
    pub offensive: f32,
    pub defensive: f32,
    pub economic: f32,
    pub special: f32,
}

impl DoctrinePreferences {
    fn from_map(map: &HashMap<String, f32>) -> Self {
        Self {
            offensive: *map.get("offensive").unwrap_or(&0.5),
            defensive: *map.get("defensive").unwrap_or(&0.5),
            economic: *map.get("economic").unwrap_or(&0.5),
            special: *map.get("special").unwrap_or(&0.5),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DecisionTiming {
    pub reaction_delay: f32,
    pub planning_lookahead: f32,
    pub adaptation_speed: f32,
    pub decision_confidence: f32,
}

#[derive(Debug, Clone)]
pub struct CombatBehavior {
    pub risk_tolerance: f32,
    pub preferred_engagement_range: String,
    pub retreat_health_threshold: f32,
    pub reinforcement_timing: String,
}

#[derive(Debug, Clone)]
pub struct AbilityUsage {
    pub frequency: f32,
    pub timing: String,
    pub coordination: f32,
}

impl AIPersonality {
    /// Create personality from JSON definition
    pub fn from_definition(def: &AIPersonalityDefinition) -> Self {
        Self {
            name: def.name.clone(),
            behavior_weights: BehaviorWeights::from_map(&def.behavior_weights),
            doctrine_preferences: DoctrinePreferences::from_map(&def.doctrine_preferences),
            decision_timing: DecisionTiming {
                reaction_delay: def.decision_timing.reaction_delay,
                planning_lookahead: def.decision_timing.planning_lookahead,
                adaptation_speed: def.decision_timing.adaptation_speed,
                decision_confidence: def.decision_timing.decision_confidence,
            },
            combat_behavior: CombatBehavior {
                risk_tolerance: def.combat_behavior.risk_tolerance,
                preferred_engagement_range: def.combat_behavior.preferred_engagement_range.clone(),
                retreat_health_threshold: def.combat_behavior.retreat_health_threshold,
                reinforcement_timing: def.combat_behavior.reinforcement_timing.clone(),
            },
            preferred_unit_types: def.preferred_unit_types.clone(),
            spawn_frequency: def.spawn_frequency,
            ability_usage: AbilityUsage {
                frequency: def.ability_usage.frequency,
                timing: def.ability_usage.timing.clone(),
                coordination: def.ability_usage.coordination,
            },
        }
    }

    /// Check if AI should perform an action based on personality weight
    pub fn should_perform(&self, action: &str) -> bool {
        let weight = match action {
            "expand" => self.behavior_weights.expand_territory,
            "defend" => self.behavior_weights.defend_position,
            "economy" => self.behavior_weights.build_economy,
            "harass" => self.behavior_weights.harass_enemy,
            "consolidate" => self.behavior_weights.consolidate_forces,
            "objective" => self.behavior_weights.capture_objectives,
            "assist" => self.behavior_weights.assist_ally,
            _ => 0.5,
        };

        rand::random::<f32>() < weight
    }

    /// Get doctrine category preference
    pub fn doctrine_preference(&self, category: &str) -> f32 {
        match category {
            "offensive" => self.doctrine_preferences.offensive,
            "defensive" => self.doctrine_preferences.defensive,
            "economic" => self.doctrine_preferences.economic,
            "special" => self.doctrine_preferences.special,
            _ => 0.5,
        }
    }
}
