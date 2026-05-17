//! Game data loading and definitions from JSON files
//!
//! This module handles loading all game data from JSON files including:
//! - Unit type definitions
//! - Commander archetypes
//! - Doctrine definitions
//! - AI personality configurations
//! - Map layouts
//!
//! All data is loaded once at startup and validated for consistency.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unit type definition from JSON
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnitDefinition {
    pub id: String,
    pub name: String,
    pub category: String,
    pub supply_cost: u32,
    pub max_health: f32,
    pub armor: f32,
    pub move_speed: f32,
    pub attack: AttackDefinition,
    pub counters: Vec<String>,
    pub countered_by: Vec<String>,
    pub ai_behavior: AIBehaviorDefinition,
    pub visual: VisualDefinition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackDefinition {
    pub damage: f32,
    pub range: f32,
    pub cooldown: f32,
    #[serde(rename = "type")]
    pub attack_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AIBehaviorDefinition {
    pub aggression: f32,
    pub retreat_health: f32,
    pub formation: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisualDefinition {
    pub size: f32,
    pub color: String,
    pub icon: String,
}

/// Commander type definition from JSON
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommanderDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub passive_effects: HashMap<String, f32>,
    pub available_doctrines: Vec<String>,
    pub abilities: Vec<AbilityDefinition>,
    pub starting_supply: u32,
    pub supply_cap: u32,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilityDefinition {
    pub id: String,
    pub name: String,
    pub cooldown: f32,
    pub duration: f32,
    pub effect: EffectDefinition,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EffectDefinition {
    #[serde(rename = "type")]
    pub effect_type: String,
    pub value: f32,
    pub radius: f32,
    #[serde(default)]
    pub target: String,
}

/// Doctrine definition from JSON
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DoctrineDefinition {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub compatible_commanders: Vec<String>,
    pub effects: HashMap<String, f32>,
    pub conflicts_with: Vec<String>,
    pub synergizes_with: Vec<String>,
    pub unlock_requirement: UnlockRequirement,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnlockRequirement {
    #[serde(rename = "type")]
    pub requirement_type: String,
    #[serde(default)]
    pub value: u32,
}

/// AI personality definition from JSON
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AIPersonalityDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub behavior_weights: HashMap<String, f32>,
    pub doctrine_preferences: HashMap<String, f32>,
    pub decision_timing: DecisionTiming,
    pub combat_behavior: CombatBehavior,
    pub preferred_unit_types: Vec<String>,
    pub spawn_frequency: f32,
    pub ability_usage: AbilityUsage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecisionTiming {
    pub reaction_delay: f32,
    pub planning_lookahead: f32,
    pub adaptation_speed: f32,
    pub decision_confidence: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CombatBehavior {
    pub risk_tolerance: f32,
    pub preferred_engagement_range: String,
    pub retreat_health_threshold: f32,
    pub reinforcement_timing: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilityUsage {
    pub frequency: f32,
    pub timing: String,
    pub coordination: f32,
}

/// Map definition from JSON
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MapDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub difficulty: String,
    pub recommended_players: String,
    pub dimensions: Dimensions,
    pub factions: Vec<FactionSetup>,
    pub sectors: Vec<SectorDefinition>,
    pub objectives: Vec<ObjectiveDefinition>,
    pub environmental_effects: Vec<EnvironmentalEffect>,
    pub recommended_ai_setup: Vec<AISetup>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FactionSetup {
    pub faction_id: u32,
    pub name: String,
    pub spawn_position: Position,
    pub spawn_hub_position: Position,
    pub starting_sectors: Vec<String>,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SectorDefinition {
    pub id: String,
    pub name: String,
    pub position: Position,
    pub radius: f32,
    #[serde(rename = "type")]
    pub sector_type: String,
    pub connections: Vec<String>,
    pub strategic_value: u32,
    pub terrain_type: String,
    pub fortification_slots: u32,
    pub initial_control: Option<u32>,
    #[serde(default)]
    pub resource_bonus: Option<ResourceBonus>,
    #[serde(default)]
    pub terrain_bonus: Option<TerrainBonus>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceBonus {
    #[serde(rename = "type")]
    pub bonus_type: String,
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TerrainBonus {
    #[serde(rename = "type")]
    pub bonus_type: String,
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObjectiveDefinition {
    #[serde(rename = "type")]
    pub objective_type: String,
    #[serde(default)]
    pub sector_id: String,
    #[serde(default)]
    pub required_count: u32,
    #[serde(default)]
    pub duration: f32,
    pub victory_points: u32,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentalEffect {
    #[serde(rename = "type")]
    pub effect_type: String,
    #[serde(default)]
    pub position: Option<Position>,
    #[serde(default)]
    pub radius: f32,
    #[serde(default)]
    pub damage_per_second: f32,
    #[serde(default)]
    pub sectors: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AISetup {
    pub faction_id: u32,
    pub commander_type: String,
    pub personality: String,
    pub difficulty: f32,
}

/// Root structure for unit types JSON
#[derive(Debug, Deserialize, Serialize)]
struct UnitTypesRoot {
    units: Vec<UnitDefinition>,
}

/// Root structure for commander types JSON
#[derive(Debug, Deserialize, Serialize)]
struct CommanderTypesRoot {
    commander_types: Vec<CommanderDefinition>,
}

/// Root structure for doctrines JSON
#[derive(Debug, Deserialize, Serialize)]
struct DoctrinesRoot {
    doctrines: Vec<DoctrineDefinition>,
}

/// Root structure for AI personalities JSON
#[derive(Debug, Deserialize, Serialize)]
struct AIPersonalitiesRoot {
    personalities: Vec<AIPersonalityDefinition>,
}

/// Root structure for maps JSON
#[derive(Debug, Deserialize, Serialize)]
struct MapsRoot {
    maps: Vec<MapDefinition>,
}

/// Container for all game data loaded from JSON
pub struct GameData {
    pub units: HashMap<String, UnitDefinition>,
    pub commanders: HashMap<String, CommanderDefinition>,
    pub doctrines: HashMap<String, DoctrineDefinition>,
    pub ai_personalities: HashMap<String, AIPersonalityDefinition>,
    pub maps: HashMap<String, MapDefinition>,
}

impl GameData {
    /// Load all game data from JSON files
    pub fn load() -> Result<Self, String> {
        // Helper macro to load JSON files correctly on both WASM and Native
        macro_rules! load_json_content {
            ($path:literal) => {{
                #[cfg(target_arch = "wasm32")]
                let content = include_str!(concat!("../", $path));

                #[cfg(not(target_arch = "wasm32"))]
                let content = match std::fs::read_to_string($path) {
                    Ok(c) => c,
                    Err(_) => include_str!(concat!("../", $path)).to_string(),
                };
                content
            }};
        }

        // Load unit types
        let units_json = load_json_content!("assets/data/unit_types.json");
        let units_root: UnitTypesRoot = serde_json::from_str(&units_json)
            .map_err(|e| format!("Failed to parse unit_types.json: {}", e))?;

        // Load commander types
        let commanders_json = load_json_content!("assets/data/commander_types.json");
        let commanders_root: CommanderTypesRoot = serde_json::from_str(&commanders_json)
            .map_err(|e| format!("Failed to parse commander_types.json: {}", e))?;

        // Load doctrines
        let doctrines_json = load_json_content!("assets/data/doctrines.json");
        let doctrines_root: DoctrinesRoot = serde_json::from_str(&doctrines_json)
            .map_err(|e| format!("Failed to parse doctrines.json: {}", e))?;

        // Load AI personalities
        let ai_json = load_json_content!("assets/data/ai_personalities.json");
        let ai_root: AIPersonalitiesRoot = serde_json::from_str(&ai_json)
            .map_err(|e| format!("Failed to parse ai_personalities.json: {}", e))?;

        // Load maps
        let maps_json = load_json_content!("assets/data/maps.json");
        let maps_root: MapsRoot = serde_json::from_str(&maps_json)
            .map_err(|e| format!("Failed to parse maps.json: {}", e))?;

        // Build lookup tables
        let units = build_lookup(units_root.units);
        let commanders = build_lookup(commanders_root.commander_types);
        let doctrines = build_lookup(doctrines_root.doctrines);
        let ai_personalities = build_lookup(ai_root.personalities);
        let maps = build_lookup(maps_root.maps);

        let game_data = Self {
            units,
            commanders,
            doctrines,
            ai_personalities,
            maps,
        };

        // Validate cross-references
        game_data.validate()?;

        Ok(game_data)
    }

    /// Validate cross-references between data files
    fn validate(&self) -> Result<(), String> {
        // Validate commander doctrines exist
        for (cmd_id, commander) in &self.commanders {
            for doctrine_id in &commander.available_doctrines {
                if !self.doctrines.contains_key(doctrine_id) {
                    return Err(format!(
                        "Commander '{}' references unknown doctrine '{}'",
                        cmd_id, doctrine_id
                    ));
                }
            }
        }

        // Validate AI preferred unit types exist
        for (ai_id, personality) in &self.ai_personalities {
            for unit_id in &personality.preferred_unit_types {
                if !self.units.contains_key(unit_id) {
                    return Err(format!(
                        "AI personality '{}' references unknown unit type '{}'",
                        ai_id, unit_id
                    ));
                }
            }
        }

        // Validate map AI setups
        for (map_id, map) in &self.maps {
            for ai_setup in &map.recommended_ai_setup {
                if !self.commanders.contains_key(&ai_setup.commander_type) {
                    return Err(format!(
                        "Map '{}' references unknown commander type '{}'",
                        map_id, ai_setup.commander_type
                    ));
                }
                if !self.ai_personalities.contains_key(&ai_setup.personality) {
                    return Err(format!(
                        "Map '{}' references unknown AI personality '{}'",
                        map_id, ai_setup.personality
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Build a lookup HashMap from a vector of items with IDs
fn build_lookup<T>(items: Vec<T>) -> HashMap<String, T>
where
    T: HasId,
{
    items
        .into_iter()
        .map(|item| (item.id().to_string(), item))
        .collect()
}

/// Trait for items that have an ID field
trait HasId {
    fn id(&self) -> &str;
}

impl HasId for UnitDefinition {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for CommanderDefinition {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for DoctrineDefinition {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for AIPersonalityDefinition {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for MapDefinition {
    fn id(&self) -> &str {
        &self.id
    }
}
