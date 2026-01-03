//! Core data structures and type definitions
//!
//! This module contains all shared game types including:
//! - Player and unit enums
//! - Unit, Squad, Commander, and Sector structs
//! - Game configuration types
//!
//! Note: Serialization removed for Phase 1 prototype. Will be re-added in Phase 3
//! when networking is implemented for deterministic synchronization.

use macroquad::prelude::*;

/// Faction identifier - supports 3 factions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactionId {
    Faction1,
    Faction2,
    Faction3,
}

impl FactionId {
    pub fn index(&self) -> usize {
        match self {
            FactionId::Faction1 => 0,
            FactionId::Faction2 => 1,
            FactionId::Faction3 => 2,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(FactionId::Faction1),
            1 => Some(FactionId::Faction2),
            2 => Some(FactionId::Faction3),
            _ => None,
        }
    }

    pub fn all() -> [FactionId; 3] {
        [FactionId::Faction1, FactionId::Faction2, FactionId::Faction3]
    }
}

/// Commander identifier within a faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommanderId {
    pub faction: FactionId,
    pub index: usize, // Index within faction's commander list
}

/// Legacy alias for compatibility during transition
pub type PlayerId = FactionId;

/// Squad order types - abstract strategic commands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquadOrder {
    Advance,  // Push objectives
    Hold,     // Defend position
    Skirmish, // Harass and disengage
    Fortify,  // Entrench, build defenses
}

/// Unit types with different supply costs and capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Infantry,  // 5 supply - basic combat
    Heavy,     // 10 supply - armored, slow
    Artillery, // 20 supply - long range, fragile
    Drone,     // 10 supply - fast, support
}

impl UnitType {
    pub fn supply_cost(&self) -> u32 {
        match self {
            UnitType::Infantry => 5,
            UnitType::Heavy => 10,
            UnitType::Artillery => 20,
            UnitType::Drone => 10,
        }
    }

    pub fn max_health(&self) -> f32 {
        match self {
            UnitType::Infantry => 100.0,
            UnitType::Heavy => 200.0,
            UnitType::Artillery => 50.0,
            UnitType::Drone => 75.0,
        }
    }

    pub fn move_speed(&self) -> f32 {
        match self {
            UnitType::Infantry => 50.0,
            UnitType::Heavy => 25.0,
            UnitType::Artillery => 20.0,
            UnitType::Drone => 80.0,
        }
    }

    pub fn attack_damage(&self) -> f32 {
        match self {
            UnitType::Infantry => 10.0,
            UnitType::Heavy => 15.0,
            UnitType::Artillery => 30.0,
            UnitType::Drone => 5.0,
        }
    }

    pub fn attack_range(&self) -> f32 {
        match self {
            UnitType::Infantry => 100.0,
            UnitType::Heavy => 80.0,
            UnitType::Artillery => 300.0,
            UnitType::Drone => 120.0,
        }
    }
}

/// Commander type/archetype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommanderType {
    Vanguard,   // Frontline pressure
    Engineer,   // Logistics and support
    Control,    // Area denial
    Disruptor,  // Sabotage and harassment
    Wildcard,   // Experimental tactics
}

impl CommanderType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "vanguard" => Some(CommanderType::Vanguard),
            "engineer" => Some(CommanderType::Engineer),
            "control" => Some(CommanderType::Control),
            "disruptor" => Some(CommanderType::Disruptor),
            "wildcard" => Some(CommanderType::Wildcard),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CommanderType::Vanguard => "vanguard",
            CommanderType::Engineer => "engineer",
            CommanderType::Control => "control",
            CommanderType::Disruptor => "disruptor",
            CommanderType::Wildcard => "wildcard",
        }
    }
}

/// Commander doctrines - strategic toggles that modify behavior
/// Note: This enum contains the base doctrines. Full doctrine system
/// is loaded from JSON and can be extended without code changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Doctrine {
    AggressivePosture,  // Faster advance, higher casualties
    EntrenchedAssault,  // Slower push, better survivability
    ShockWindows,       // Damage spikes after reinforcements
    RapidDeployment,    // Faster unit spawning
    ResourceEfficiency, // Lower supply costs
}

impl Doctrine {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "aggressive_posture" => Some(Doctrine::AggressivePosture),
            "entrenched_assault" => Some(Doctrine::EntrenchedAssault),
            "shock_windows" => Some(Doctrine::ShockWindows),
            "rapid_deployment" => Some(Doctrine::RapidDeployment),
            "resource_efficiency" => Some(Doctrine::ResourceEfficiency),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Doctrine::AggressivePosture => "Units advance faster but take more damage",
            Doctrine::EntrenchedAssault => "Units move slower but have better defense",
            Doctrine::ShockWindows => "Units deal bonus damage when freshly deployed",
            Doctrine::RapidDeployment => "Units spawn faster",
            Doctrine::ResourceEfficiency => "Units cost less supply",
        }
    }
}

/// Individual unit in a squad
#[derive(Debug, Clone)]
pub struct Unit {
    pub id: u32,
    pub unit_type: UnitType,
    pub position: Vec2,
    pub health: f32,
    pub owner: PlayerId,
    pub target_position: Option<Vec2>,
    pub target_enemy: Option<u32>,
    pub spawn_time: f32, // Game time when unit was spawned (for ShockWindows doctrine)
}

impl Unit {
    pub fn new(id: u32, unit_type: UnitType, position: Vec2, owner: PlayerId) -> Self {
        Self {
            id,
            unit_type,
            position,
            health: unit_type.max_health(),
            owner,
            target_position: None,
            target_enemy: None,
            spawn_time: 0.0, // Will be set by game state
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn health_percent(&self) -> f32 {
        self.health / self.unit_type.max_health()
    }

    pub fn is_fresh_spawn(&self, current_time: f32) -> bool {
        current_time - self.spawn_time <= crate::config::Config::SHOCK_DURATION
    }
}

/// Squad - collection of units operating under a single order
#[derive(Debug, Clone)]
pub struct Squad {
    pub id: u32,
    pub units: Vec<Unit>,
    pub order: SquadOrder,
    pub owner: PlayerId,
    pub rally_point: Vec2,
}

impl Squad {
    pub fn new(id: u32, owner: PlayerId, order: SquadOrder, rally_point: Vec2) -> Self {
        Self {
            id,
            units: Vec::new(),
            order,
            owner,
            rally_point,
        }
    }

    pub fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }

    pub fn remove_dead_units(&mut self) {
        self.units.retain(|u| u.is_alive());
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    pub fn center_position(&self) -> Vec2 {
        if self.units.is_empty() {
            return self.rally_point;
        }

        let sum = self
            .units
            .iter()
            .fold(Vec2::ZERO, |acc, u| acc + u.position);
        sum / self.units.len() as f32
    }
}

/// Combat effect for visual feedback
#[derive(Debug, Clone)]
pub struct CombatEffect {
    pub position: Vec2,
    pub damage: f32,
    pub time_remaining: f32,
    pub effect_type: EffectType,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    Hit,
    Kill,
}

impl CombatEffect {
    pub fn new_hit(position: Vec2, damage: f32) -> Self {
        Self {
            position,
            damage,
            time_remaining: 0.5,
            effect_type: EffectType::Hit,
        }
    }

    pub fn new_kill(position: Vec2) -> Self {
        Self {
            position,
            damage: 0.0,
            time_remaining: 1.0,
            effect_type: EffectType::Kill,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.time_remaining -= dt;
        self.time_remaining > 0.0
    }
}

/// Commander state - player's strategic control interface
#[derive(Debug, Clone, Copy)]
pub struct Commander {
    pub faction_id: FactionId,
    pub commander_type: CommanderType,
    pub active_doctrines: [Option<Doctrine>; 2],
    pub supply_used: u32,
    pub supply_max: u32,
    pub is_player_controlled: bool,
    pub spawn_timer: f32,
}

impl Commander {
    pub fn new(
        faction_id: FactionId,
        commander_type: CommanderType,
        supply_max: u32,
        is_player_controlled: bool,
    ) -> Self {
        Self {
            faction_id,
            commander_type,
            active_doctrines: [None, None],
            supply_used: 0,
            supply_max,
            is_player_controlled,
            spawn_timer: 0.0,
        }
    }

    pub fn available_supply(&self) -> u32 {
        self.supply_max.saturating_sub(self.supply_used)
    }

    pub fn can_afford(&self, cost: u32) -> bool {
        self.available_supply() >= cost
    }

    pub fn set_doctrine(&mut self, slot: usize, doctrine: Option<Doctrine>) {
        if slot < 2 {
            self.active_doctrines[slot] = doctrine;
        }
    }

    pub fn has_doctrine(&self, doctrine: Doctrine) -> bool {
        self.active_doctrines.contains(&Some(doctrine))
    }

    pub fn is_ai(&self) -> bool {
        !self.is_player_controlled
    }
}

/// Battlefield sector - strategic control points
#[derive(Debug, Clone)]
pub struct Sector {
    pub id: u32,
    pub position: Vec2,
    pub radius: f32,
    pub control: Option<FactionId>,
    pub control_progress: f32, // -1.0 to 1.0 (Faction1 to Faction3)
}

impl Sector {
    pub fn new(id: u32, position: Vec2, radius: f32) -> Self {
        Self {
            id,
            position,
            radius,
            control: None,
            control_progress: 0.0,
        }
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        self.position.distance(point) <= self.radius
    }

    /// Set initial control from faction index (1-3)
    pub fn set_initial_control(&mut self, faction_index: u32) {
        self.control = FactionId::from_index((faction_index - 1) as usize);
        self.control_progress = match self.control {
            Some(FactionId::Faction1) => -1.0,
            Some(FactionId::Faction2) => 0.0,
            Some(FactionId::Faction3) => 1.0,
            None => 0.0,
        };
    }
}
