mod ai;
mod app;
mod map;

pub use ai::BasicSkirmishAi;
pub use app::RtsApp;
use macroquad::prelude::{vec2, Vec2};
pub use map::{
    RtsMapArea, RtsMapBlocker, RtsMapBuildingPlacement, RtsMapDefinition, RtsMapDimensions,
    RtsMapExpansionMarker, RtsMapLeyNode, RtsMapLeySegment, RtsMapMatterNode, RtsMapPlayerStart,
    RtsMapPosition, RtsMapUnitPlacement,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

pub const PLAYER_ONE: usize = 0;
pub const PLAYER_TWO: usize = 1;

const STARTING_MATTER: u32 = 200;
const STARTING_AETHER: u32 = 0;
const STARTING_WORKERS: u32 = 4;
const WORKER_GATHER_RATE: f32 = 12.0;
const AETHERBORN_SHRINE_RATE: f32 = 4.0;
const AETHERBORN_LEY_FLOW_PER_SHRINE: u32 = 3;
const AETHERBORN_RITUAL_LINK_RANGE: f32 = 180.0;
const TERRAN_EXTRACTOR_RATE: f32 = 5.0;
const TERRAN_BATTERY_ROUTE_RANGE: f32 = 240.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RaceId {
    Aetherborn,
    Terran,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceNodeId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitKind {
    SpriteGatherer,
    FieldEngineer,
    ElvenWarden,
    RangerTrooper,
}

impl UnitKind {
    pub fn supply_cost(self) -> u32 {
        match self {
            UnitKind::SpriteGatherer | UnitKind::FieldEngineer => 1,
            UnitKind::ElvenWarden | UnitKind::RangerTrooper => 2,
        }
    }

    pub fn matter_cost(self) -> u32 {
        match self {
            UnitKind::SpriteGatherer | UnitKind::FieldEngineer => 50,
            UnitKind::ElvenWarden => 75,
            UnitKind::RangerTrooper => 80,
        }
    }

    pub fn production_time(self) -> f32 {
        match self {
            UnitKind::SpriteGatherer => 12.0,
            UnitKind::FieldEngineer => 11.0,
            UnitKind::ElvenWarden => 18.0,
            UnitKind::RangerTrooper => 17.0,
        }
    }

    pub fn base_stats(self) -> UnitStats {
        match self {
            UnitKind::SpriteGatherer => UnitStats {
                max_health: 45,
                attack_damage: 3,
                attack_range: 65.0,
                attack_cooldown: 1.15,
                armor: 0,
                movement_speed: 72.0,
            },
            UnitKind::FieldEngineer => UnitStats {
                max_health: 55,
                attack_damage: 4,
                attack_range: 70.0,
                attack_cooldown: 1.15,
                armor: 0,
                movement_speed: 68.0,
            },
            UnitKind::ElvenWarden => UnitStats {
                max_health: 80,
                attack_damage: 9,
                attack_range: 150.0,
                attack_cooldown: 1.0,
                armor: 1,
                movement_speed: 74.0,
            },
            UnitKind::RangerTrooper => UnitStats {
                max_health: 95,
                attack_damage: 8,
                attack_range: 170.0,
                attack_cooldown: 1.0,
                armor: 1,
                movement_speed: 70.0,
            },
        }
    }

    pub fn race(self) -> RaceId {
        match self {
            UnitKind::SpriteGatherer | UnitKind::ElvenWarden => RaceId::Aetherborn,
            UnitKind::FieldEngineer | UnitKind::RangerTrooper => RaceId::Terran,
        }
    }

    pub fn is_worker(self) -> bool {
        matches!(self, UnitKind::SpriteGatherer | UnitKind::FieldEngineer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildingKind {
    HeartwoodNexus,
    CommandArk,
    Moonwell,
    SupplyPylon,
    GroveCircle,
    FabricatorBay,
    LeyShrine,
    RitualNode,
    AetherExtractorRig,
    BatteryDepot,
}

impl BuildingKind {
    pub fn race(self) -> RaceId {
        match self {
            BuildingKind::HeartwoodNexus
            | BuildingKind::Moonwell
            | BuildingKind::GroveCircle
            | BuildingKind::LeyShrine
            | BuildingKind::RitualNode => RaceId::Aetherborn,
            BuildingKind::CommandArk
            | BuildingKind::SupplyPylon
            | BuildingKind::FabricatorBay
            | BuildingKind::AetherExtractorRig
            | BuildingKind::BatteryDepot => RaceId::Terran,
        }
    }

    pub fn matter_cost(self) -> u32 {
        match self {
            BuildingKind::HeartwoodNexus | BuildingKind::CommandArk => 400,
            BuildingKind::Moonwell | BuildingKind::SupplyPylon => 90,
            BuildingKind::GroveCircle | BuildingKind::FabricatorBay => 160,
            BuildingKind::LeyShrine => 120,
            BuildingKind::RitualNode => 70,
            BuildingKind::AetherExtractorRig => 140,
            BuildingKind::BatteryDepot => 110,
        }
    }

    pub fn build_time(self) -> f32 {
        match self {
            BuildingKind::HeartwoodNexus | BuildingKind::CommandArk => 70.0,
            BuildingKind::Moonwell | BuildingKind::SupplyPylon => 10.0,
            BuildingKind::GroveCircle | BuildingKind::FabricatorBay => 35.0,
            BuildingKind::LeyShrine => 25.0,
            BuildingKind::RitualNode => 12.0,
            BuildingKind::AetherExtractorRig => 28.0,
            BuildingKind::BatteryDepot => 16.0,
        }
    }

    pub fn supply_provided(self) -> u32 {
        match self {
            BuildingKind::HeartwoodNexus | BuildingKind::CommandArk => 10,
            BuildingKind::Moonwell | BuildingKind::SupplyPylon => 8,
            BuildingKind::GroveCircle
            | BuildingKind::FabricatorBay
            | BuildingKind::LeyShrine
            | BuildingKind::RitualNode
            | BuildingKind::AetherExtractorRig
            | BuildingKind::BatteryDepot => 0,
        }
    }

    pub fn can_produce(self, unit_kind: UnitKind) -> bool {
        matches!(
            (self, unit_kind),
            (BuildingKind::HeartwoodNexus, UnitKind::SpriteGatherer)
                | (BuildingKind::CommandArk, UnitKind::FieldEngineer)
                | (BuildingKind::GroveCircle, UnitKind::ElvenWarden)
                | (BuildingKind::FabricatorBay, UnitKind::RangerTrooper)
        )
    }

    pub fn can_research(self, tech_kind: TechKind) -> bool {
        matches!(
            (self, tech_kind),
            (BuildingKind::GroveCircle, TechKind::LivingBark)
                | (BuildingKind::FabricatorBay, TechKind::StabilizedBarrels)
        )
    }

    fn can_claim_ley_node(self) -> bool {
        matches!(
            self,
            BuildingKind::LeyShrine | BuildingKind::AetherExtractorRig
        )
    }

    fn is_aetherborn_ritual_network_member(self) -> bool {
        matches!(
            self,
            BuildingKind::HeartwoodNexus | BuildingKind::RitualNode | BuildingKind::LeyShrine
        )
    }

    fn is_terran_battery_destination(self) -> bool {
        matches!(self, BuildingKind::CommandArk | BuildingKind::BatteryDepot)
    }

    fn max_health(self) -> f32 {
        match self {
            BuildingKind::HeartwoodNexus | BuildingKind::CommandArk => 1_200.0,
            BuildingKind::Moonwell | BuildingKind::SupplyPylon => 300.0,
            BuildingKind::GroveCircle | BuildingKind::FabricatorBay => 500.0,
            BuildingKind::LeyShrine | BuildingKind::AetherExtractorRig => 350.0,
            BuildingKind::RitualNode => 180.0,
            BuildingKind::BatteryDepot => 320.0,
        }
    }

    fn armor(self) -> u32 {
        match self {
            BuildingKind::HeartwoodNexus | BuildingKind::CommandArk => 2,
            BuildingKind::GroveCircle | BuildingKind::FabricatorBay => 1,
            _ => 0,
        }
    }

    pub fn is_main_base(self) -> bool {
        matches!(
            self,
            BuildingKind::HeartwoodNexus | BuildingKind::CommandArk
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TechKind {
    LivingBark,
    StabilizedBarrels,
}

impl TechKind {
    pub fn race(self) -> RaceId {
        match self {
            TechKind::LivingBark => RaceId::Aetherborn,
            TechKind::StabilizedBarrels => RaceId::Terran,
        }
    }

    pub fn matter_cost(self) -> u32 {
        match self {
            TechKind::LivingBark => 100,
            TechKind::StabilizedBarrels => 120,
        }
    }

    pub fn research_time(self) -> f32 {
        match self {
            TechKind::LivingBark => 20.0,
            TechKind::StabilizedBarrels => 20.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitStats {
    pub max_health: u32,
    pub attack_damage: u32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub armor: u32,
    pub movement_speed: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitCommand {
    Idle,
    Move(Vec2),
    AttackMove(Vec2),
    AttackUnit(EntityId),
    AttackBuilding(EntityId),
    GatherMatter(ResourceNodeId),
    Build(EntityId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceNodeKind {
    Matter,
    Ley,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtsError {
    InvalidPlayer,
    InvalidEntity,
    InvalidResourceNode,
    WrongRace,
    NotAWorker,
    InsufficientMatter,
    SupplyBlocked,
    BuildingIncomplete,
    UnsupportedProduction,
    UnsupportedResearch,
    TechAlreadyResearched,
    InvalidAetherClaim,
    ResourceNodeAlreadyClaimed,
    InvalidTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CombatTarget {
    Unit(EntityId),
    Building(EntityId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceStockpile {
    pub matter: u32,
    pub aether: u32,
}

impl ResourceStockpile {
    pub fn new(matter: u32, aether: u32) -> Self {
        Self { matter, aether }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub race: RaceId,
    pub resources: ResourceStockpile,
    pub ley_flow_capacity: u32,
    pub supply_used: u32,
    pub supply_cap: u32,
    pub researched_techs: Vec<TechKind>,
}

impl PlayerState {
    fn new(race: RaceId) -> Self {
        Self {
            race,
            resources: ResourceStockpile::new(STARTING_MATTER, STARTING_AETHER),
            ley_flow_capacity: 0,
            supply_used: 0,
            supply_cap: 0,
            researched_techs: Vec::new(),
        }
    }

    pub fn available_supply(&self) -> u32 {
        self.supply_cap.saturating_sub(self.supply_used)
    }

    pub fn has_tech(&self, tech_kind: TechKind) -> bool {
        self.researched_techs.contains(&tech_kind)
    }
}

#[derive(Debug, Clone)]
pub struct UnitInstance {
    pub id: EntityId,
    pub owner: usize,
    pub kind: UnitKind,
    pub position: Vec2,
    pub health: f32,
    pub attack_cooldown_remaining: f32,
    pub command: UnitCommand,
    gather_buffer: f32,
}

#[derive(Debug, Clone)]
pub struct BuildingInstance {
    pub id: EntityId,
    pub owner: usize,
    pub kind: BuildingKind,
    pub position: Vec2,
    pub health: f32,
    pub completed: bool,
    pub build_time_remaining: f32,
    pub claimed_node: Option<ResourceNodeId>,
    aether_buffer: f32,
    pub production_queue: VecDeque<ProductionJob>,
    pub research_queue: VecDeque<ResearchJob>,
}

#[derive(Debug, Clone)]
pub struct ProductionJob {
    pub unit_kind: UnitKind,
    pub time_remaining: f32,
}

#[derive(Debug, Clone)]
pub struct ResearchJob {
    pub tech_kind: TechKind,
    pub time_remaining: f32,
}

#[derive(Debug, Clone)]
pub struct ResourceNode {
    pub id: ResourceNodeId,
    pub kind: ResourceNodeKind,
    pub position: Vec2,
    pub remaining: u32,
}

#[derive(Debug, Clone)]
pub struct RtsGameState {
    pub map_id: Option<String>,
    pub map_size: Vec2,
    pub players: Vec<PlayerState>,
    pub units: Vec<UnitInstance>,
    pub buildings: Vec<BuildingInstance>,
    pub resource_nodes: Vec<ResourceNode>,
    pub winner: Option<usize>,
    next_entity_id: u32,
    next_resource_node_id: u32,
}

impl RtsGameState {
    pub fn new_two_player_test_match() -> Self {
        let mut state = Self {
            map_id: None,
            map_size: vec2(1_200.0, 720.0),
            players: vec![
                PlayerState::new(RaceId::Aetherborn),
                PlayerState::new(RaceId::Terran),
            ],
            units: Vec::new(),
            buildings: Vec::new(),
            resource_nodes: Vec::new(),
            winner: None,
            next_entity_id: 0,
            next_resource_node_id: 0,
        };

        state.add_completed_building(PLAYER_ONE, BuildingKind::HeartwoodNexus, vec2(200.0, 300.0));
        state.add_completed_building(PLAYER_TWO, BuildingKind::CommandArk, vec2(1000.0, 300.0));

        for index in 0..STARTING_WORKERS {
            let offset = vec2(index as f32 * 12.0, 28.0);
            state.add_unit(
                PLAYER_ONE,
                UnitKind::SpriteGatherer,
                vec2(200.0, 300.0) + offset,
            );
            state.add_unit(
                PLAYER_TWO,
                UnitKind::FieldEngineer,
                vec2(1000.0, 300.0) + offset,
            );
        }

        state.add_matter_node(vec2(260.0, 360.0), 5_000);
        state.add_matter_node(vec2(940.0, 360.0), 5_000);

        state
    }

    pub fn update(&mut self, dt: f32) {
        if self.winner.is_some() {
            return;
        }

        self.update_attack_cooldowns(dt);
        self.update_gathering(dt);
        self.update_movement(dt);
        self.update_construction(dt);
        self.update_aether(dt);
        self.update_combat();
        self.cleanup_destroyed_entities();
        self.update_production(dt);
        self.update_research(dt);
    }

    pub fn command_gather_matter(
        &mut self,
        unit_id: EntityId,
        node_id: ResourceNodeId,
    ) -> Result<(), RtsError> {
        if self.resource_node(node_id).is_none() {
            return Err(RtsError::InvalidResourceNode);
        }

        let unit = self.unit_mut(unit_id).ok_or(RtsError::InvalidEntity)?;
        if !unit.kind.is_worker() {
            return Err(RtsError::NotAWorker);
        }

        unit.command = UnitCommand::GatherMatter(node_id);
        Ok(())
    }

    pub fn command_move_unit(&mut self, unit_id: EntityId, target: Vec2) -> Result<(), RtsError> {
        let unit = self.unit_mut(unit_id).ok_or(RtsError::InvalidEntity)?;
        unit.command = UnitCommand::Move(target);
        Ok(())
    }

    pub fn command_attack_move_unit(
        &mut self,
        unit_id: EntityId,
        target: Vec2,
    ) -> Result<(), RtsError> {
        let unit = self.unit_mut(unit_id).ok_or(RtsError::InvalidEntity)?;
        unit.command = UnitCommand::AttackMove(target);
        Ok(())
    }

    pub fn command_attack_unit(
        &mut self,
        unit_id: EntityId,
        target_id: EntityId,
    ) -> Result<(), RtsError> {
        let attacker_owner = self.unit(unit_id).ok_or(RtsError::InvalidEntity)?.owner;
        let target = self.unit(target_id).ok_or(RtsError::InvalidTarget)?;
        if target.owner == attacker_owner {
            return Err(RtsError::InvalidTarget);
        }

        let unit = self.unit_mut(unit_id).ok_or(RtsError::InvalidEntity)?;
        unit.command = UnitCommand::AttackUnit(target_id);
        Ok(())
    }

    pub fn command_attack_building(
        &mut self,
        unit_id: EntityId,
        target_id: EntityId,
    ) -> Result<(), RtsError> {
        let attacker_owner = self.unit(unit_id).ok_or(RtsError::InvalidEntity)?.owner;
        let target = self.building(target_id).ok_or(RtsError::InvalidTarget)?;
        if target.owner == attacker_owner {
            return Err(RtsError::InvalidTarget);
        }

        let unit = self.unit_mut(unit_id).ok_or(RtsError::InvalidEntity)?;
        unit.command = UnitCommand::AttackBuilding(target_id);
        Ok(())
    }

    pub fn start_construction(
        &mut self,
        worker_id: EntityId,
        building_kind: BuildingKind,
        position: Vec2,
    ) -> Result<EntityId, RtsError> {
        let worker = self.unit(worker_id).ok_or(RtsError::InvalidEntity)?;
        if !worker.kind.is_worker() {
            return Err(RtsError::NotAWorker);
        }

        let owner = worker.owner;
        self.ensure_player(owner)?;

        if self.players[owner].race != building_kind.race() {
            return Err(RtsError::WrongRace);
        }

        let cost = building_kind.matter_cost();
        if self.players[owner].resources.matter < cost {
            return Err(RtsError::InsufficientMatter);
        }

        self.players[owner].resources.matter -= cost;

        let building_id = self.allocate_entity_id();
        self.buildings.push(BuildingInstance {
            id: building_id,
            owner,
            kind: building_kind,
            position,
            health: building_kind.max_health(),
            completed: false,
            build_time_remaining: building_kind.build_time(),
            claimed_node: None,
            aether_buffer: 0.0,
            production_queue: VecDeque::new(),
            research_queue: VecDeque::new(),
        });

        let worker = self.unit_mut(worker_id).ok_or(RtsError::InvalidEntity)?;
        worker.command = UnitCommand::Build(building_id);

        Ok(building_id)
    }

    pub fn start_construction_on_resource_node(
        &mut self,
        worker_id: EntityId,
        building_kind: BuildingKind,
        node_id: ResourceNodeId,
    ) -> Result<EntityId, RtsError> {
        self.validate_ley_claim(building_kind, node_id)?;
        let position = self
            .resource_node(node_id)
            .ok_or(RtsError::InvalidResourceNode)?
            .position;

        let building_id = self.start_construction(worker_id, building_kind, position)?;
        let building = self
            .building_mut(building_id)
            .ok_or(RtsError::InvalidEntity)?;
        building.claimed_node = Some(node_id);

        Ok(building_id)
    }

    pub fn can_train_unit(&self, player_id: usize, unit_kind: UnitKind) -> Result<(), RtsError> {
        self.ensure_player(player_id)?;

        let player = &self.players[player_id];
        if player.race != unit_kind.race() {
            return Err(RtsError::WrongRace);
        }

        if player.resources.matter < unit_kind.matter_cost() {
            return Err(RtsError::InsufficientMatter);
        }

        if self.available_supply_after_queued(player_id) < unit_kind.supply_cost() {
            return Err(RtsError::SupplyBlocked);
        }

        Ok(())
    }

    pub fn train_unit(
        &mut self,
        producer_id: EntityId,
        unit_kind: UnitKind,
    ) -> Result<(), RtsError> {
        let (owner, building_kind) = {
            let building = self.building(producer_id).ok_or(RtsError::InvalidEntity)?;
            if !building.completed {
                return Err(RtsError::BuildingIncomplete);
            }
            (building.owner, building.kind)
        };

        if !building_kind.can_produce(unit_kind) {
            return Err(RtsError::UnsupportedProduction);
        }

        self.can_train_unit(owner, unit_kind)?;
        self.players[owner].resources.matter -= unit_kind.matter_cost();

        let building = self
            .building_mut(producer_id)
            .ok_or(RtsError::InvalidEntity)?;
        building.production_queue.push_back(ProductionJob {
            unit_kind,
            time_remaining: unit_kind.production_time(),
        });

        Ok(())
    }

    pub fn research_tech(
        &mut self,
        researcher_id: EntityId,
        tech_kind: TechKind,
    ) -> Result<(), RtsError> {
        let (owner, building_kind) = {
            let building = self
                .building(researcher_id)
                .ok_or(RtsError::InvalidEntity)?;
            if !building.completed {
                return Err(RtsError::BuildingIncomplete);
            }
            (building.owner, building.kind)
        };

        if self.players[owner].race != tech_kind.race() {
            return Err(RtsError::WrongRace);
        }

        if !building_kind.can_research(tech_kind) {
            return Err(RtsError::UnsupportedResearch);
        }

        if self.players[owner].has_tech(tech_kind) || self.is_tech_queued_for(owner, tech_kind) {
            return Err(RtsError::TechAlreadyResearched);
        }

        let cost = tech_kind.matter_cost();
        if self.players[owner].resources.matter < cost {
            return Err(RtsError::InsufficientMatter);
        }

        self.players[owner].resources.matter -= cost;

        let building = self
            .building_mut(researcher_id)
            .ok_or(RtsError::InvalidEntity)?;
        building.research_queue.push_back(ResearchJob {
            tech_kind,
            time_remaining: tech_kind.research_time(),
        });

        Ok(())
    }

    pub fn unit_stats_for_player(&self, player_id: usize, unit_kind: UnitKind) -> UnitStats {
        let mut stats = unit_kind.base_stats();

        if let Some(player) = self.players.get(player_id) {
            if unit_kind == UnitKind::RangerTrooper && player.has_tech(TechKind::StabilizedBarrels)
            {
                stats.attack_damage += 2;
            }

            if unit_kind == UnitKind::ElvenWarden && player.has_tech(TechKind::LivingBark) {
                stats.max_health += 12;
            }
        }

        stats
    }

    pub fn first_worker_for(&self, player_id: usize) -> Option<EntityId> {
        self.units
            .iter()
            .find(|unit| unit.owner == player_id && unit.kind.is_worker())
            .map(|unit| unit.id)
    }

    pub fn completed_buildings_for(
        &self,
        player_id: usize,
        kind: BuildingKind,
    ) -> impl Iterator<Item = &BuildingInstance> {
        self.buildings.iter().filter(move |building| {
            building.owner == player_id && building.kind == kind && building.completed
        })
    }

    pub fn first_completed_building_for(
        &self,
        player_id: usize,
        kind: BuildingKind,
    ) -> Option<EntityId> {
        self.completed_buildings_for(player_id, kind)
            .next()
            .map(|building| building.id)
    }

    pub fn resource_node(&self, node_id: ResourceNodeId) -> Option<&ResourceNode> {
        self.resource_nodes.iter().find(|node| node.id == node_id)
    }

    pub fn first_matter_node(&self) -> Option<ResourceNodeId> {
        self.resource_nodes
            .iter()
            .find(|node| node.kind == ResourceNodeKind::Matter)
            .map(|node| node.id)
    }

    pub fn first_ley_node(&self) -> Option<ResourceNodeId> {
        self.resource_nodes
            .iter()
            .find(|node| node.kind == ResourceNodeKind::Ley)
            .map(|node| node.id)
    }

    pub fn add_matter_node(&mut self, position: Vec2, remaining: u32) -> ResourceNodeId {
        let id = ResourceNodeId(self.next_resource_node_id);
        self.next_resource_node_id += 1;

        self.resource_nodes.push(ResourceNode {
            id,
            kind: ResourceNodeKind::Matter,
            position,
            remaining,
        });

        id
    }

    pub fn add_ley_node(&mut self, position: Vec2) -> ResourceNodeId {
        let id = ResourceNodeId(self.next_resource_node_id);
        self.next_resource_node_id += 1;

        self.resource_nodes.push(ResourceNode {
            id,
            kind: ResourceNodeKind::Ley,
            position,
            remaining: 0,
        });

        id
    }

    pub fn add_unit(&mut self, owner: usize, kind: UnitKind, position: Vec2) -> EntityId {
        let id = self.allocate_entity_id();
        self.players[owner].supply_used += kind.supply_cost();
        let stats = self.unit_stats_for_player(owner, kind);
        self.units.push(UnitInstance {
            id,
            owner,
            kind,
            position,
            health: stats.max_health as f32,
            attack_cooldown_remaining: 0.0,
            command: UnitCommand::Idle,
            gather_buffer: 0.0,
        });
        id
    }

    pub fn add_completed_building(
        &mut self,
        owner: usize,
        kind: BuildingKind,
        position: Vec2,
    ) -> EntityId {
        let id = self.allocate_entity_id();
        self.players[owner].supply_cap += kind.supply_provided();
        self.buildings.push(BuildingInstance {
            id,
            owner,
            kind,
            position,
            health: kind.max_health(),
            completed: true,
            build_time_remaining: 0.0,
            claimed_node: None,
            aether_buffer: 0.0,
            production_queue: VecDeque::new(),
            research_queue: VecDeque::new(),
        });
        id
    }

    pub fn add_completed_building_on_node(
        &mut self,
        owner: usize,
        kind: BuildingKind,
        node_id: ResourceNodeId,
    ) -> Result<EntityId, RtsError> {
        self.ensure_player(owner)?;
        if self.players[owner].race != kind.race() {
            return Err(RtsError::WrongRace);
        }

        self.validate_ley_claim(kind, node_id)?;
        let position = self
            .resource_node(node_id)
            .ok_or(RtsError::InvalidResourceNode)?
            .position;
        let id = self.add_completed_building(owner, kind, position);
        let building = self.building_mut(id).ok_or(RtsError::InvalidEntity)?;
        building.claimed_node = Some(node_id);

        Ok(id)
    }

    fn update_gathering(&mut self, dt: f32) {
        for unit_index in 0..self.units.len() {
            let UnitCommand::GatherMatter(node_id) = self.units[unit_index].command else {
                continue;
            };

            let Some(node_index) = self
                .resource_nodes
                .iter()
                .position(|node| node.id == node_id && node.kind == ResourceNodeKind::Matter)
            else {
                self.units[unit_index].command = UnitCommand::Idle;
                continue;
            };

            if self.resource_nodes[node_index].remaining == 0 {
                self.units[unit_index].command = UnitCommand::Idle;
                continue;
            }

            self.units[unit_index].gather_buffer += WORKER_GATHER_RATE * dt;
            let gathered = self.units[unit_index].gather_buffer.floor() as u32;
            if gathered == 0 {
                continue;
            }

            let owner = self.units[unit_index].owner;
            let gathered = gathered.min(self.resource_nodes[node_index].remaining);
            self.units[unit_index].gather_buffer -= gathered as f32;
            self.resource_nodes[node_index].remaining -= gathered;
            self.players[owner].resources.matter += gathered;
        }
    }

    fn update_attack_cooldowns(&mut self, dt: f32) {
        for unit in &mut self.units {
            unit.attack_cooldown_remaining = (unit.attack_cooldown_remaining - dt).max(0.0);
        }
    }

    fn update_movement(&mut self, dt: f32) {
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

    fn update_construction(&mut self, dt: f32) {
        let mut completed_buildings = Vec::new();

        for building in &mut self.buildings {
            if building.completed {
                continue;
            }

            let has_builder = self.units.iter().any(|unit| {
                unit.owner == building.owner && unit.command == UnitCommand::Build(building.id)
            });

            if !has_builder {
                continue;
            }

            building.build_time_remaining = (building.build_time_remaining - dt).max(0.0);
            if building.build_time_remaining == 0.0 {
                building.completed = true;
                completed_buildings.push((building.owner, building.kind, building.id));
            }
        }

        for (owner, kind, building_id) in completed_buildings {
            self.players[owner].supply_cap += kind.supply_provided();
            for unit in self.units.iter_mut().filter(|unit| {
                unit.owner == owner && unit.command == UnitCommand::Build(building_id)
            }) {
                unit.command = UnitCommand::Idle;
            }
        }
    }

    fn update_aether(&mut self, dt: f32) {
        let mut active_aether_buildings = Vec::new();
        let mut flow_by_player = vec![0; self.players.len()];

        for building_index in 0..self.buildings.len() {
            let building = &self.buildings[building_index];
            if !building.completed || building.claimed_node.is_none() {
                continue;
            }

            match building.kind {
                BuildingKind::LeyShrine if self.aetherborn_shrine_connected(building_index) => {
                    active_aether_buildings.push((building_index, AETHERBORN_SHRINE_RATE));
                    flow_by_player[building.owner] += AETHERBORN_LEY_FLOW_PER_SHRINE;
                }
                BuildingKind::AetherExtractorRig
                    if self.terran_extractor_has_route(building_index) =>
                {
                    active_aether_buildings.push((building_index, TERRAN_EXTRACTOR_RATE));
                }
                _ => {}
            }
        }

        for (player_id, player) in self.players.iter_mut().enumerate() {
            player.ley_flow_capacity = flow_by_player[player_id];
        }

        for (building_index, rate) in active_aether_buildings {
            let building = &mut self.buildings[building_index];
            building.aether_buffer += rate * dt;
            let produced = building.aether_buffer.floor() as u32;
            if produced == 0 {
                continue;
            }

            building.aether_buffer -= produced as f32;
            self.players[building.owner].resources.aether += produced;
        }
    }

    fn update_combat(&mut self) {
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

    fn cleanup_destroyed_entities(&mut self) {
        let destroyed_units: Vec<(usize, UnitKind)> = self
            .units
            .iter()
            .filter(|unit| unit.health <= 0.0)
            .map(|unit| (unit.owner, unit.kind))
            .collect();
        self.units.retain(|unit| unit.health > 0.0);

        for (owner, kind) in destroyed_units {
            self.players[owner].supply_used = self.players[owner]
                .supply_used
                .saturating_sub(kind.supply_cost());
        }

        let destroyed_buildings: Vec<(usize, BuildingKind)> = self
            .buildings
            .iter()
            .filter(|building| building.health <= 0.0)
            .map(|building| (building.owner, building.kind))
            .collect();
        self.buildings.retain(|building| building.health > 0.0);

        for (owner, kind) in destroyed_buildings {
            self.players[owner].supply_cap = self.players[owner]
                .supply_cap
                .saturating_sub(kind.supply_provided());
        }

        self.update_victory_state();
    }

    fn update_production(&mut self, dt: f32) {
        let mut completed_units = Vec::new();

        for building in &mut self.buildings {
            if !building.completed {
                continue;
            }

            let Some(job) = building.production_queue.front_mut() else {
                continue;
            };

            job.time_remaining = (job.time_remaining - dt).max(0.0);
            if job.time_remaining == 0.0 {
                let job = building.production_queue.pop_front().unwrap();
                completed_units.push((building.owner, job.unit_kind, building.position));
            }
        }

        for (owner, unit_kind, position) in completed_units {
            self.add_unit(owner, unit_kind, position + vec2(32.0, 0.0));
        }
    }

    fn update_research(&mut self, dt: f32) {
        let mut completed_research = Vec::new();

        for building in &mut self.buildings {
            if !building.completed {
                continue;
            }

            let Some(job) = building.research_queue.front_mut() else {
                continue;
            };

            job.time_remaining = (job.time_remaining - dt).max(0.0);
            if job.time_remaining == 0.0 {
                let job = building.research_queue.pop_front().unwrap();
                completed_research.push((building.owner, job.tech_kind));
            }
        }

        for (owner, tech_kind) in completed_research {
            if !self.players[owner].has_tech(tech_kind) {
                self.players[owner].researched_techs.push(tech_kind);
            }
        }
    }

    fn available_supply_after_queued(&self, player_id: usize) -> u32 {
        let queued_supply: u32 = self
            .buildings
            .iter()
            .filter(|building| building.owner == player_id)
            .flat_map(|building| &building.production_queue)
            .map(|job| job.unit_kind.supply_cost())
            .sum();

        self.players[player_id]
            .supply_cap
            .saturating_sub(self.players[player_id].supply_used + queued_supply)
    }

    fn is_tech_queued_for(&self, player_id: usize, tech_kind: TechKind) -> bool {
        self.buildings
            .iter()
            .filter(|building| building.owner == player_id)
            .flat_map(|building| &building.research_queue)
            .any(|job| job.tech_kind == tech_kind)
    }

    fn validate_ley_claim(
        &self,
        building_kind: BuildingKind,
        node_id: ResourceNodeId,
    ) -> Result<(), RtsError> {
        if !building_kind.can_claim_ley_node() {
            return Err(RtsError::InvalidAetherClaim);
        }

        let node = self
            .resource_node(node_id)
            .ok_or(RtsError::InvalidResourceNode)?;
        if node.kind != ResourceNodeKind::Ley {
            return Err(RtsError::InvalidResourceNode);
        }

        if self
            .buildings
            .iter()
            .any(|building| building.claimed_node == Some(node_id))
        {
            return Err(RtsError::ResourceNodeAlreadyClaimed);
        }

        Ok(())
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

    fn find_auto_attack_target(
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

    fn update_victory_state(&mut self) {
        if self.winner.is_some() {
            return;
        }

        let defeated_player = self.players.iter().enumerate().find_map(|(player_id, _)| {
            let has_main_base = self.buildings.iter().any(|building| {
                building.owner == player_id
                    && building.completed
                    && building.kind.is_main_base()
                    && building.health > 0.0
            });

            if has_main_base {
                None
            } else {
                Some(player_id)
            }
        });

        if let Some(defeated_player) = defeated_player {
            self.winner = self
                .players
                .iter()
                .enumerate()
                .find_map(|(player_id, _)| (player_id != defeated_player).then_some(player_id));
        }
    }

    fn aetherborn_shrine_connected(&self, shrine_index: usize) -> bool {
        let shrine = &self.buildings[shrine_index];
        let mut frontier: Vec<EntityId> = self
            .buildings
            .iter()
            .filter(|building| {
                building.owner == shrine.owner
                    && building.completed
                    && building.kind == BuildingKind::HeartwoodNexus
            })
            .map(|building| building.id)
            .collect();
        let mut visited = HashSet::new();

        while let Some(current_id) = frontier.pop() {
            if !visited.insert(current_id) {
                continue;
            }

            if current_id == shrine.id {
                return true;
            }

            let Some(current) = self.building(current_id) else {
                continue;
            };

            for neighbor in self.buildings.iter().filter(|building| {
                building.owner == shrine.owner
                    && building.completed
                    && building.kind.is_aetherborn_ritual_network_member()
                    && !visited.contains(&building.id)
                    && (building.position - current.position).length()
                        <= AETHERBORN_RITUAL_LINK_RANGE
            }) {
                frontier.push(neighbor.id);
            }
        }

        false
    }

    fn terran_extractor_has_route(&self, extractor_index: usize) -> bool {
        let extractor = &self.buildings[extractor_index];
        self.buildings.iter().any(|building| {
            building.owner == extractor.owner
                && building.completed
                && building.kind.is_terran_battery_destination()
                && (building.position - extractor.position).length() <= TERRAN_BATTERY_ROUTE_RANGE
        })
    }

    fn unit(&self, unit_id: EntityId) -> Option<&UnitInstance> {
        self.units.iter().find(|unit| unit.id == unit_id)
    }

    fn unit_mut(&mut self, unit_id: EntityId) -> Option<&mut UnitInstance> {
        self.units.iter_mut().find(|unit| unit.id == unit_id)
    }

    fn building(&self, building_id: EntityId) -> Option<&BuildingInstance> {
        self.buildings
            .iter()
            .find(|building| building.id == building_id)
    }

    fn building_mut(&mut self, building_id: EntityId) -> Option<&mut BuildingInstance> {
        self.buildings
            .iter_mut()
            .find(|building| building.id == building_id)
    }

    fn ensure_player(&self, player_id: usize) -> Result<(), RtsError> {
        if self.players.get(player_id).is_some() {
            Ok(())
        } else {
            Err(RtsError::InvalidPlayer)
        }
    }

    fn allocate_entity_id(&mut self) -> EntityId {
        let id = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        id
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
