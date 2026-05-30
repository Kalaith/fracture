mod ai;
mod app;
mod combat;
mod definitions;
mod economy;
mod map;
mod movement;
mod production;
mod state;
mod victory;

pub use ai::BasicSkirmishAi;
pub use app::RtsApp;
pub use definitions::{
    building_definition, command_catalog_for, tech_definition, unit_definition, BuildingDefinition,
    BuildingKind, RaceCommandCatalog, RaceId, TechDefinition, TechKind, TechStatBonus,
    UnitDefinition, UnitKind, UnitStats, ALL_BUILDING_KINDS, ALL_RACES, ALL_TECH_KINDS,
    ALL_UNIT_KINDS, BUILDING_DEFINITIONS, RACE_COMMAND_CATALOGS, TECH_DEFINITIONS,
    UNIT_DEFINITIONS,
};
pub use map::{
    RtsMapArea, RtsMapBlocker, RtsMapBuildingPlacement, RtsMapDefinition, RtsMapDimensions,
    RtsMapExpansionMarker, RtsMapLeyNode, RtsMapLeySegment, RtsMapMatterNode, RtsMapPlayerStart,
    RtsMapPosition, RtsMapUnitPlacement,
};
pub use state::{
    BuildingInstance, EntityId, PlayerState, ProductionJob, ResearchJob, ResourceNode,
    ResourceNodeId, ResourceNodeKind, ResourceStockpile, RtsError, RtsGameState, UnitCommand,
    UnitInstance,
};

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
