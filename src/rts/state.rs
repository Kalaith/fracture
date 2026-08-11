use super::map::{RtsMapArea, RtsMapBlocker, RtsMapExpansionMarker};
use super::{
    BuildingKind, RaceId, TechKind, UnitKind, PLAYER_ONE, PLAYER_TWO, STARTING_AETHER,
    STARTING_MATTER, STARTING_WORKERS,
};
use macroquad::prelude::{vec2, Vec2};
use std::collections::VecDeque;

const BUILDING_BLOCKER_PADDING: f32 = 32.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceNodeId(pub u32);

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
    MissingTech,
    InvalidAetherClaim,
    ResourceNodeAlreadyClaimed,
    InvalidTarget,
    InvalidBuildLocation,
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
    pub(super) fn new(race: RaceId) -> Self {
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
    pub(super) gather_buffer: f32,
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
    pub(super) aether_buffer: f32,
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
    pub camera_bounds: Option<RtsMapArea>,
    pub buildable_areas: Vec<RtsMapArea>,
    pub path_blockers: Vec<RtsMapBlocker>,
    pub expansion_markers: Vec<RtsMapExpansionMarker>,
    pub players: Vec<PlayerState>,
    pub units: Vec<UnitInstance>,
    pub buildings: Vec<BuildingInstance>,
    pub resource_nodes: Vec<ResourceNode>,
    pub winner: Option<usize>,
    pub(super) next_entity_id: u32,
    pub(super) next_resource_node_id: u32,
}

impl RtsGameState {
    pub fn new_two_player_test_match() -> Self {
        let mut state = Self {
            map_id: None,
            map_size: vec2(1_200.0, 720.0),
            camera_bounds: None,
            buildable_areas: Vec::new(),
            path_blockers: Vec::new(),
            expansion_markers: Vec::new(),
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
        let target = self.nearest_passable_position(target);
        let unit = self.unit_mut(unit_id).ok_or(RtsError::InvalidEntity)?;
        unit.command = UnitCommand::Move(target);
        Ok(())
    }

    pub fn command_attack_move_unit(
        &mut self,
        unit_id: EntityId,
        target: Vec2,
    ) -> Result<(), RtsError> {
        let target = self.nearest_passable_position(target);
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
        self.start_construction_internal(worker_id, building_kind, position, false)
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

        let building_id =
            self.start_construction_internal(worker_id, building_kind, position, true)?;
        let building = self
            .building_mut(building_id)
            .ok_or(RtsError::InvalidEntity)?;
        building.claimed_node = Some(node_id);

        Ok(building_id)
    }

    fn start_construction_internal(
        &mut self,
        worker_id: EntityId,
        building_kind: BuildingKind,
        position: Vec2,
        allow_resource_node_position: bool,
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

        if !allow_resource_node_position && !self.can_place_standard_building(position) {
            return Err(RtsError::InvalidBuildLocation);
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

    pub fn can_place_standard_building(&self, position: Vec2) -> bool {
        if position.x < 0.0
            || position.y < 0.0
            || position.x > self.map_size.x
            || position.y > self.map_size.y
        {
            return false;
        }

        if !self.buildable_areas.is_empty()
            && !self
                .buildable_areas
                .iter()
                .any(|area| area.contains(position))
        {
            return false;
        }

        self.path_blockers.iter().all(|blocker| {
            position.distance(blocker.position.as_vec2())
                > blocker.radius + BUILDING_BLOCKER_PADDING
        })
    }

    pub fn nearest_passable_position(&self, position: Vec2) -> Vec2 {
        let mut passable = if let Some(bounds) = self.camera_bounds {
            bounds.clamp(position)
        } else {
            vec2(
                position.x.clamp(0.0, self.map_size.x),
                position.y.clamp(0.0, self.map_size.y),
            )
        };

        for blocker in &self.path_blockers {
            let center = blocker.position.as_vec2();
            let offset = passable - center;
            let distance = offset.length();
            if distance >= blocker.radius {
                continue;
            }

            let direction = if distance <= f32::EPSILON {
                vec2(1.0, 0.0)
            } else {
                offset / distance
            };
            passable = center + direction * blocker.radius;
        }

        if let Some(bounds) = self.camera_bounds {
            bounds.clamp(passable)
        } else {
            vec2(
                passable.x.clamp(0.0, self.map_size.x),
                passable.y.clamp(0.0, self.map_size.y),
            )
        }
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

    pub(super) fn unit(&self, unit_id: EntityId) -> Option<&UnitInstance> {
        self.units.iter().find(|unit| unit.id == unit_id)
    }

    pub(super) fn unit_mut(&mut self, unit_id: EntityId) -> Option<&mut UnitInstance> {
        self.units.iter_mut().find(|unit| unit.id == unit_id)
    }

    pub(super) fn building(&self, building_id: EntityId) -> Option<&BuildingInstance> {
        self.buildings
            .iter()
            .find(|building| building.id == building_id)
    }

    pub(super) fn building_mut(&mut self, building_id: EntityId) -> Option<&mut BuildingInstance> {
        self.buildings
            .iter_mut()
            .find(|building| building.id == building_id)
    }

    pub(super) fn ensure_player(&self, player_id: usize) -> Result<(), RtsError> {
        if self.players.get(player_id).is_some() {
            Ok(())
        } else {
            Err(RtsError::InvalidPlayer)
        }
    }

    pub(super) fn allocate_entity_id(&mut self) -> EntityId {
        let id = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        id
    }
}
