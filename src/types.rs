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
        [
            FactionId::Faction1,
            FactionId::Faction2,
            FactionId::Faction3,
        ]
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
    // Infantry
    InfantryLight, // 3 supply - cheap, fast, weak
    InfantryHeavy, // 8 supply - tough, versatile

    // Armor
    ArmorLight, // 10 supply - mobile tank
    ArmorHeavy, // 15 supply - heavy tank, slow

    // Artillery
    Artillery, // 20 supply - long range siege

    // Drones
    DroneScout,   // 5 supply - fast recon
    DroneGunship, // 12 supply - air support

    // Support
    Engineer, // 8 supply - repairs and builds

    // Specialists
    AntiArmor, // 10 supply - counters heavy armor
    AntiAir,   // 10 supply - counters drones
}

impl UnitType {
    pub fn supply_cost(&self) -> u32 {
        match self {
            UnitType::InfantryLight => 3,
            UnitType::InfantryHeavy => 8,
            UnitType::ArmorLight => 10,
            UnitType::ArmorHeavy => 15,
            UnitType::Artillery => 20,
            UnitType::DroneScout => 5,
            UnitType::DroneGunship => 12,
            UnitType::Engineer => 8,
            UnitType::AntiArmor => 10,
            UnitType::AntiAir => 10,
        }
    }

    pub fn max_health(&self) -> f32 {
        match self {
            UnitType::InfantryLight => 80.0,
            UnitType::InfantryHeavy => 150.0,
            UnitType::ArmorLight => 200.0,
            UnitType::ArmorHeavy => 350.0,
            UnitType::Artillery => 60.0,
            UnitType::DroneScout => 50.0,
            UnitType::DroneGunship => 120.0,
            UnitType::Engineer => 100.0,
            UnitType::AntiArmor => 100.0,
            UnitType::AntiAir => 90.0,
        }
    }

    pub fn armor(&self) -> f32 {
        match self {
            UnitType::InfantryLight => 0.0,
            UnitType::InfantryHeavy => 10.0,
            UnitType::ArmorLight => 30.0,
            UnitType::ArmorHeavy => 60.0,
            UnitType::Artillery => 0.0,
            UnitType::DroneScout => 0.0,
            UnitType::DroneGunship => 15.0,
            UnitType::Engineer => 5.0,
            UnitType::AntiArmor => 5.0,
            UnitType::AntiAir => 5.0,
        }
    }

    pub fn move_speed(&self) -> f32 {
        match self {
            UnitType::InfantryLight => 60.0,
            UnitType::InfantryHeavy => 40.0,
            UnitType::ArmorLight => 45.0,
            UnitType::ArmorHeavy => 25.0,
            UnitType::Artillery => 20.0,
            UnitType::DroneScout => 100.0,
            UnitType::DroneGunship => 70.0,
            UnitType::Engineer => 35.0,
            UnitType::AntiArmor => 50.0,
            UnitType::AntiAir => 45.0,
        }
    }

    pub fn attack_damage(&self) -> f32 {
        match self {
            UnitType::InfantryLight => 8.0,
            UnitType::InfantryHeavy => 15.0,
            UnitType::ArmorLight => 20.0,
            UnitType::ArmorHeavy => 30.0,
            UnitType::Artillery => 50.0,
            UnitType::DroneScout => 3.0,
            UnitType::DroneGunship => 18.0,
            UnitType::Engineer => 5.0,
            UnitType::AntiArmor => 35.0,
            UnitType::AntiAir => 25.0,
        }
    }

    pub fn attack_range(&self) -> f32 {
        match self {
            UnitType::InfantryLight => 90.0,
            UnitType::InfantryHeavy => 100.0,
            UnitType::ArmorLight => 120.0,
            UnitType::ArmorHeavy => 110.0,
            UnitType::Artillery => 400.0,
            UnitType::DroneScout => 80.0,
            UnitType::DroneGunship => 150.0,
            UnitType::Engineer => 50.0,
            UnitType::AntiArmor => 130.0,
            UnitType::AntiAir => 200.0,
        }
    }

    /// Get counter bonus multiplier against target type
    pub fn counter_bonus(&self, target: UnitType) -> f32 {
        match (self, target) {
            // Anti-Armor counters all armor
            (UnitType::AntiArmor, UnitType::ArmorLight) => 2.0,
            (UnitType::AntiArmor, UnitType::ArmorHeavy) => 2.5,

            // Anti-Air counters all drones
            (UnitType::AntiAir, UnitType::DroneScout) => 3.0,
            (UnitType::AntiAir, UnitType::DroneGunship) => 2.5,

            // Heavy infantry good vs light units
            (UnitType::InfantryHeavy, UnitType::InfantryLight) => 1.5,
            (UnitType::InfantryHeavy, UnitType::DroneScout) => 1.3,

            // Armor strong vs infantry
            (UnitType::ArmorLight, UnitType::InfantryLight) => 1.5,
            (UnitType::ArmorLight, UnitType::InfantryHeavy) => 1.3,
            (UnitType::ArmorHeavy, UnitType::InfantryLight) => 2.0,
            (UnitType::ArmorHeavy, UnitType::InfantryHeavy) => 1.5,

            // Artillery wrecks static targets
            (UnitType::Artillery, UnitType::Engineer) => 1.8,
            (UnitType::Artillery, UnitType::ArmorHeavy) => 1.4,

            // Default: no bonus
            _ => 1.0,
        }
    }
}

/// Commander type/archetype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommanderType {
    Vanguard,  // Frontline pressure
    Engineer,  // Logistics and support
    Control,   // Area denial
    Disruptor, // Sabotage and harassment
    Wildcard,  // Experimental tactics
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

    /// Get passive damage bonus for this commander type (Phase 4)
    pub fn damage_bonus(&self) -> f32 {
        match self {
            CommanderType::Vanguard => 1.1,   // +10% damage
            CommanderType::Disruptor => 1.05, // +5% damage
            _ => 1.0,
        }
    }

    /// Get passive morale recovery bonus for this commander type (Phase 4)
    pub fn morale_recovery_bonus(&self) -> f32 {
        match self {
            CommanderType::Engineer => 1.5, // +50% morale recovery
            CommanderType::Control => 1.2,  // +20% morale recovery
            _ => 1.0,
        }
    }

    /// Get description of passive ability
    pub fn ability_description(&self) -> &'static str {
        match self {
            CommanderType::Vanguard => "+10% damage",
            CommanderType::Engineer => "+50% morale recovery",
            CommanderType::Control => "+20% morale recovery",
            CommanderType::Disruptor => "+5% damage",
            CommanderType::Wildcard => "Adaptive tactics",
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

    /// Check if two doctrines synergize (work well together)
    /// Returns synergy multiplier (1.0 = no synergy, >1.0 = positive synergy)
    pub fn synergy_with(&self, other: Doctrine) -> f32 {
        match (self, other) {
            // Offensive synergy: Aggressive + Shock Windows
            (Doctrine::AggressivePosture, Doctrine::ShockWindows)
            | (Doctrine::ShockWindows, Doctrine::AggressivePosture) => 1.15,

            // Defensive synergy: Entrenched + Rapid Deployment
            (Doctrine::EntrenchedAssault, Doctrine::RapidDeployment)
            | (Doctrine::RapidDeployment, Doctrine::EntrenchedAssault) => 1.15,

            // Economic synergy: Resource Efficiency + Rapid Deployment
            (Doctrine::ResourceEfficiency, Doctrine::RapidDeployment)
            | (Doctrine::RapidDeployment, Doctrine::ResourceEfficiency) => 1.1,

            _ => 1.0,
        }
    }

    /// Check if two doctrines conflict (work poorly together)
    /// Returns conflict penalty (1.0 = no conflict, <1.0 = penalty)
    pub fn conflict_with(&self, other: Doctrine) -> f32 {
        match (self, other) {
            // Major conflict: Aggressive vs Entrenched (contradictory strategies)
            (Doctrine::AggressivePosture, Doctrine::EntrenchedAssault)
            | (Doctrine::EntrenchedAssault, Doctrine::AggressivePosture) => 0.85,

            _ => 1.0,
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
    pub morale: f32, // 0.0 to 1.0, affects combat effectiveness
}

impl Squad {
    pub fn new(id: u32, owner: PlayerId, order: SquadOrder, rally_point: Vec2) -> Self {
        Self {
            id,
            units: Vec::new(),
            order,
            owner,
            rally_point,
            morale: 1.0, // Start with full morale
        }
    }

    /// Get morale modifier for combat (0.7 to 1.3 range)
    pub fn morale_modifier(&self) -> f32 {
        0.7 + (self.morale * 0.6) // Maps 0.0-1.0 morale to 0.7-1.3 multiplier
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

    /// Get the doctrine synergy/conflict modifier
    /// Returns the combined synergy and conflict multiplier for active doctrines
    pub fn doctrine_modifier(&self) -> f32 {
        match (self.active_doctrines[0], self.active_doctrines[1]) {
            (Some(d1), Some(d2)) => {
                // Calculate synergy and conflict
                let synergy = d1.synergy_with(d2);
                let conflict = d1.conflict_with(d2);
                synergy * conflict
            }
            _ => 1.0, // No modifier if less than 2 doctrines active
        }
    }

    /// Get a description of the current doctrine interaction
    pub fn doctrine_interaction_desc(&self) -> Option<String> {
        match (self.active_doctrines[0], self.active_doctrines[1]) {
            (Some(d1), Some(d2)) => {
                let modifier = self.doctrine_modifier();
                if modifier > 1.05 {
                    Some(format!(
                        "Synergy: {:?} + {:?} (+{:.0}%)",
                        d1,
                        d2,
                        (modifier - 1.0) * 100.0
                    ))
                } else if modifier < 0.95 {
                    Some(format!(
                        "Conflict: {:?} vs {:?} ({:.0}%)",
                        d1,
                        d2,
                        (modifier - 1.0) * 100.0
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Sector type - determines strategic value and bonuses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectorType {
    Standard,    // No special bonus
    SupplyDepot, // +20 max supply when controlled
    Fortified,   // +20% defense for units in sector
    Industrial,  // -15% unit spawn time when controlled
    HighGround,  // +15% damage for units in sector
}

impl SectorType {
    pub fn description(&self) -> &'static str {
        match self {
            SectorType::Standard => "Standard sector",
            SectorType::SupplyDepot => "+20 max supply",
            SectorType::Fortified => "+20% defense in sector",
            SectorType::Industrial => "-15% spawn time",
            SectorType::HighGround => "+15% damage in sector",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            SectorType::Standard => "○",
            SectorType::SupplyDepot => "S",
            SectorType::Fortified => "F",
            SectorType::Industrial => "I",
            SectorType::HighGround => "H",
        }
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
    pub sector_type: SectorType,
}

impl Sector {
    pub fn new(id: u32, position: Vec2, radius: f32) -> Self {
        Self {
            id,
            position,
            radius,
            control: None,
            control_progress: 0.0,
            sector_type: SectorType::Standard,
        }
    }

    pub fn with_type(mut self, sector_type: SectorType) -> Self {
        self.sector_type = sector_type;
        self
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

/// Strategic marker types for player intent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerType {
    Attack, // Prioritize this sector for offensive
    Defend, // Prioritize this sector for defense
}

/// Strategic marker placed by players on sectors
#[derive(Debug, Clone, Copy)]
pub struct StrategicMarker {
    pub sector_id: u32,
    pub marker_type: MarkerType,
    pub owner: FactionId,
}

impl StrategicMarker {
    pub fn new(sector_id: u32, marker_type: MarkerType, owner: FactionId) -> Self {
        Self {
            sector_id,
            marker_type,
            owner,
        }
    }
}
