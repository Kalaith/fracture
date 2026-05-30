use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RaceId {
    Aetherborn,
    Terran,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitKind {
    SpriteGatherer,
    FieldEngineer,
    ElvenWarden,
    GroveSentinel,
    WizardAdept,
    RangerTrooper,
    AegisWalker,
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
    StarfallSanctum,
    MechFoundry,
    ElderSanctum,
    TacticalLab,
    LeyShrine,
    RitualNode,
    AetherExtractorRig,
    BatteryDepot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TechKind {
    LivingBark,
    StabilizedBarrels,
    RootguardPact,
    AstralChanneling,
    AegisFrame,
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

#[derive(Debug, Clone, Copy)]
pub struct UnitDefinition {
    pub kind: UnitKind,
    pub race: RaceId,
    pub supply_cost: u32,
    pub matter_cost: u32,
    pub production_time: f32,
    pub base_stats: UnitStats,
    pub required_tech: Option<TechKind>,
}

#[derive(Debug, Clone, Copy)]
pub struct BuildingDefinition {
    pub kind: BuildingKind,
    pub race: RaceId,
    pub matter_cost: u32,
    pub build_time: f32,
    pub supply_provided: u32,
    pub max_health: f32,
    pub armor: u32,
    pub produces: &'static [UnitKind],
    pub researches: &'static [TechKind],
    pub can_claim_ley_node: bool,
    pub is_aetherborn_ritual_network_member: bool,
    pub is_terran_battery_destination: bool,
    pub is_main_base: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct TechDefinition {
    pub kind: TechKind,
    pub race: RaceId,
    pub matter_cost: u32,
    pub research_time: f32,
    pub stat_bonuses: &'static [TechStatBonus],
}

#[derive(Debug, Clone, Copy)]
pub struct TechStatBonus {
    pub unit_kind: UnitKind,
    pub max_health: u32,
    pub attack_damage: u32,
    pub armor: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct RaceCommandCatalog {
    pub race: RaceId,
    pub supply_building: BuildingKind,
    pub production_building: BuildingKind,
    pub advanced_production_building: BuildingKind,
    pub research_building: BuildingKind,
    pub ley_claim_building: BuildingKind,
    pub aether_link_building: BuildingKind,
    pub worker_unit: UnitKind,
    pub combat_unit: UnitKind,
    pub heavy_unit: UnitKind,
    pub specialist_unit: Option<UnitKind>,
    pub basic_tech: TechKind,
    pub heavy_tech: TechKind,
    pub specialist_tech: Option<TechKind>,
    pub advanced_units: &'static [UnitKind],
    pub tech_plan: &'static [TechKind],
}

pub const ALL_RACES: &[RaceId] = &[RaceId::Aetherborn, RaceId::Terran];

pub const ALL_UNIT_KINDS: &[UnitKind] = &[
    UnitKind::SpriteGatherer,
    UnitKind::FieldEngineer,
    UnitKind::ElvenWarden,
    UnitKind::GroveSentinel,
    UnitKind::WizardAdept,
    UnitKind::RangerTrooper,
    UnitKind::AegisWalker,
];

pub const ALL_BUILDING_KINDS: &[BuildingKind] = &[
    BuildingKind::HeartwoodNexus,
    BuildingKind::CommandArk,
    BuildingKind::Moonwell,
    BuildingKind::SupplyPylon,
    BuildingKind::GroveCircle,
    BuildingKind::FabricatorBay,
    BuildingKind::StarfallSanctum,
    BuildingKind::MechFoundry,
    BuildingKind::ElderSanctum,
    BuildingKind::TacticalLab,
    BuildingKind::LeyShrine,
    BuildingKind::RitualNode,
    BuildingKind::AetherExtractorRig,
    BuildingKind::BatteryDepot,
];

pub const ALL_TECH_KINDS: &[TechKind] = &[
    TechKind::LivingBark,
    TechKind::StabilizedBarrels,
    TechKind::RootguardPact,
    TechKind::AstralChanneling,
    TechKind::AegisFrame,
];

const HEARTWOOD_PRODUCES: &[UnitKind] = &[UnitKind::SpriteGatherer];
const COMMAND_ARK_PRODUCES: &[UnitKind] = &[UnitKind::FieldEngineer];
const GROVE_CIRCLE_PRODUCES: &[UnitKind] = &[UnitKind::ElvenWarden];
const FABRICATOR_BAY_PRODUCES: &[UnitKind] = &[UnitKind::RangerTrooper];
const STARFALL_SANCTUM_PRODUCES: &[UnitKind] =
    &[UnitKind::GroveSentinel, UnitKind::WizardAdept];
const MECH_FOUNDRY_PRODUCES: &[UnitKind] = &[UnitKind::AegisWalker];

const GROVE_CIRCLE_RESEARCHES: &[TechKind] = &[TechKind::LivingBark];
const FABRICATOR_BAY_RESEARCHES: &[TechKind] = &[TechKind::StabilizedBarrels];
const ELDER_SANCTUM_RESEARCHES: &[TechKind] =
    &[TechKind::RootguardPact, TechKind::AstralChanneling];
const TACTICAL_LAB_RESEARCHES: &[TechKind] =
    &[TechKind::AegisFrame, TechKind::StabilizedBarrels];

const LIVING_BARK_BONUSES: &[TechStatBonus] = &[TechStatBonus {
    unit_kind: UnitKind::ElvenWarden,
    max_health: 12,
    attack_damage: 0,
    armor: 0,
}];

const STABILIZED_BARRELS_BONUSES: &[TechStatBonus] = &[TechStatBonus {
    unit_kind: UnitKind::RangerTrooper,
    max_health: 0,
    attack_damage: 2,
    armor: 0,
}];

const ROOTGUARD_PACT_BONUSES: &[TechStatBonus] = &[TechStatBonus {
    unit_kind: UnitKind::GroveSentinel,
    max_health: 20,
    attack_damage: 0,
    armor: 1,
}];

const ASTRAL_CHANNELING_BONUSES: &[TechStatBonus] = &[TechStatBonus {
    unit_kind: UnitKind::WizardAdept,
    max_health: 0,
    attack_damage: 4,
    armor: 0,
}];

const AEGIS_FRAME_BONUSES: &[TechStatBonus] = &[TechStatBonus {
    unit_kind: UnitKind::AegisWalker,
    max_health: 30,
    attack_damage: 0,
    armor: 0,
}];

const AETHERBORN_ADVANCED_UNITS: &[UnitKind] =
    &[UnitKind::GroveSentinel, UnitKind::WizardAdept];
const TERRAN_ADVANCED_UNITS: &[UnitKind] = &[UnitKind::AegisWalker];

const AETHERBORN_TECH_PLAN: &[TechKind] = &[
    TechKind::LivingBark,
    TechKind::RootguardPact,
    TechKind::AstralChanneling,
];
const TERRAN_TECH_PLAN: &[TechKind] = &[TechKind::StabilizedBarrels, TechKind::AegisFrame];

pub const UNIT_DEFINITIONS: &[UnitDefinition] = &[
    UnitDefinition {
        kind: UnitKind::SpriteGatherer,
        race: RaceId::Aetherborn,
        supply_cost: 1,
        matter_cost: 50,
        production_time: 12.0,
        base_stats: UnitStats {
            max_health: 45,
            attack_damage: 3,
            attack_range: 65.0,
            attack_cooldown: 1.15,
            armor: 0,
            movement_speed: 72.0,
        },
        required_tech: None,
    },
    UnitDefinition {
        kind: UnitKind::FieldEngineer,
        race: RaceId::Terran,
        supply_cost: 1,
        matter_cost: 50,
        production_time: 11.0,
        base_stats: UnitStats {
            max_health: 55,
            attack_damage: 4,
            attack_range: 70.0,
            attack_cooldown: 1.15,
            armor: 0,
            movement_speed: 68.0,
        },
        required_tech: None,
    },
    UnitDefinition {
        kind: UnitKind::ElvenWarden,
        race: RaceId::Aetherborn,
        supply_cost: 2,
        matter_cost: 75,
        production_time: 18.0,
        base_stats: UnitStats {
            max_health: 80,
            attack_damage: 9,
            attack_range: 150.0,
            attack_cooldown: 1.0,
            armor: 1,
            movement_speed: 74.0,
        },
        required_tech: None,
    },
    UnitDefinition {
        kind: UnitKind::GroveSentinel,
        race: RaceId::Aetherborn,
        supply_cost: 3,
        matter_cost: 140,
        production_time: 28.0,
        base_stats: UnitStats {
            max_health: 180,
            attack_damage: 12,
            attack_range: 95.0,
            attack_cooldown: 1.2,
            armor: 3,
            movement_speed: 54.0,
        },
        required_tech: Some(TechKind::RootguardPact),
    },
    UnitDefinition {
        kind: UnitKind::WizardAdept,
        race: RaceId::Aetherborn,
        supply_cost: 3,
        matter_cost: 130,
        production_time: 26.0,
        base_stats: UnitStats {
            max_health: 70,
            attack_damage: 16,
            attack_range: 210.0,
            attack_cooldown: 1.35,
            armor: 0,
            movement_speed: 66.0,
        },
        required_tech: Some(TechKind::AstralChanneling),
    },
    UnitDefinition {
        kind: UnitKind::RangerTrooper,
        race: RaceId::Terran,
        supply_cost: 2,
        matter_cost: 80,
        production_time: 17.0,
        base_stats: UnitStats {
            max_health: 95,
            attack_damage: 8,
            attack_range: 170.0,
            attack_cooldown: 1.0,
            armor: 1,
            movement_speed: 70.0,
        },
        required_tech: None,
    },
    UnitDefinition {
        kind: UnitKind::AegisWalker,
        race: RaceId::Terran,
        supply_cost: 4,
        matter_cost: 190,
        production_time: 34.0,
        base_stats: UnitStats {
            max_health: 240,
            attack_damage: 18,
            attack_range: 155.0,
            attack_cooldown: 1.45,
            armor: 4,
            movement_speed: 48.0,
        },
        required_tech: Some(TechKind::AegisFrame),
    },
];

pub const BUILDING_DEFINITIONS: &[BuildingDefinition] = &[
    BuildingDefinition {
        kind: BuildingKind::HeartwoodNexus,
        race: RaceId::Aetherborn,
        matter_cost: 400,
        build_time: 70.0,
        supply_provided: 10,
        max_health: 1_200.0,
        armor: 2,
        produces: HEARTWOOD_PRODUCES,
        researches: &[],
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: true,
        is_terran_battery_destination: false,
        is_main_base: true,
    },
    BuildingDefinition {
        kind: BuildingKind::CommandArk,
        race: RaceId::Terran,
        matter_cost: 400,
        build_time: 70.0,
        supply_provided: 10,
        max_health: 1_200.0,
        armor: 2,
        produces: COMMAND_ARK_PRODUCES,
        researches: &[],
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: true,
        is_main_base: true,
    },
    BuildingDefinition {
        kind: BuildingKind::Moonwell,
        race: RaceId::Aetherborn,
        matter_cost: 90,
        build_time: 10.0,
        supply_provided: 8,
        max_health: 300.0,
        armor: 0,
        produces: &[],
        researches: &[],
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::SupplyPylon,
        race: RaceId::Terran,
        matter_cost: 90,
        build_time: 10.0,
        supply_provided: 8,
        max_health: 300.0,
        armor: 0,
        produces: &[],
        researches: &[],
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::GroveCircle,
        race: RaceId::Aetherborn,
        matter_cost: 160,
        build_time: 35.0,
        supply_provided: 0,
        max_health: 500.0,
        armor: 1,
        produces: GROVE_CIRCLE_PRODUCES,
        researches: GROVE_CIRCLE_RESEARCHES,
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::FabricatorBay,
        race: RaceId::Terran,
        matter_cost: 160,
        build_time: 35.0,
        supply_provided: 0,
        max_health: 500.0,
        armor: 1,
        produces: FABRICATOR_BAY_PRODUCES,
        researches: FABRICATOR_BAY_RESEARCHES,
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::StarfallSanctum,
        race: RaceId::Aetherborn,
        matter_cost: 220,
        build_time: 45.0,
        supply_provided: 0,
        max_health: 650.0,
        armor: 1,
        produces: STARFALL_SANCTUM_PRODUCES,
        researches: &[],
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::MechFoundry,
        race: RaceId::Terran,
        matter_cost: 220,
        build_time: 45.0,
        supply_provided: 0,
        max_health: 650.0,
        armor: 1,
        produces: MECH_FOUNDRY_PRODUCES,
        researches: &[],
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::ElderSanctum,
        race: RaceId::Aetherborn,
        matter_cost: 180,
        build_time: 38.0,
        supply_provided: 0,
        max_health: 420.0,
        armor: 0,
        produces: &[],
        researches: ELDER_SANCTUM_RESEARCHES,
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::TacticalLab,
        race: RaceId::Terran,
        matter_cost: 180,
        build_time: 38.0,
        supply_provided: 0,
        max_health: 420.0,
        armor: 0,
        produces: &[],
        researches: TACTICAL_LAB_RESEARCHES,
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::LeyShrine,
        race: RaceId::Aetherborn,
        matter_cost: 120,
        build_time: 25.0,
        supply_provided: 0,
        max_health: 350.0,
        armor: 0,
        produces: &[],
        researches: &[],
        can_claim_ley_node: true,
        is_aetherborn_ritual_network_member: true,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::RitualNode,
        race: RaceId::Aetherborn,
        matter_cost: 70,
        build_time: 12.0,
        supply_provided: 0,
        max_health: 180.0,
        armor: 0,
        produces: &[],
        researches: &[],
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: true,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::AetherExtractorRig,
        race: RaceId::Terran,
        matter_cost: 140,
        build_time: 28.0,
        supply_provided: 0,
        max_health: 350.0,
        armor: 0,
        produces: &[],
        researches: &[],
        can_claim_ley_node: true,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: false,
        is_main_base: false,
    },
    BuildingDefinition {
        kind: BuildingKind::BatteryDepot,
        race: RaceId::Terran,
        matter_cost: 110,
        build_time: 16.0,
        supply_provided: 0,
        max_health: 320.0,
        armor: 0,
        produces: &[],
        researches: &[],
        can_claim_ley_node: false,
        is_aetherborn_ritual_network_member: false,
        is_terran_battery_destination: true,
        is_main_base: false,
    },
];

pub const TECH_DEFINITIONS: &[TechDefinition] = &[
    TechDefinition {
        kind: TechKind::LivingBark,
        race: RaceId::Aetherborn,
        matter_cost: 100,
        research_time: 20.0,
        stat_bonuses: LIVING_BARK_BONUSES,
    },
    TechDefinition {
        kind: TechKind::StabilizedBarrels,
        race: RaceId::Terran,
        matter_cost: 120,
        research_time: 20.0,
        stat_bonuses: STABILIZED_BARRELS_BONUSES,
    },
    TechDefinition {
        kind: TechKind::RootguardPact,
        race: RaceId::Aetherborn,
        matter_cost: 140,
        research_time: 24.0,
        stat_bonuses: ROOTGUARD_PACT_BONUSES,
    },
    TechDefinition {
        kind: TechKind::AstralChanneling,
        race: RaceId::Aetherborn,
        matter_cost: 160,
        research_time: 28.0,
        stat_bonuses: ASTRAL_CHANNELING_BONUSES,
    },
    TechDefinition {
        kind: TechKind::AegisFrame,
        race: RaceId::Terran,
        matter_cost: 170,
        research_time: 30.0,
        stat_bonuses: AEGIS_FRAME_BONUSES,
    },
];

pub const RACE_COMMAND_CATALOGS: &[RaceCommandCatalog] = &[
    RaceCommandCatalog {
        race: RaceId::Aetherborn,
        supply_building: BuildingKind::Moonwell,
        production_building: BuildingKind::GroveCircle,
        advanced_production_building: BuildingKind::StarfallSanctum,
        research_building: BuildingKind::ElderSanctum,
        ley_claim_building: BuildingKind::LeyShrine,
        aether_link_building: BuildingKind::RitualNode,
        worker_unit: UnitKind::SpriteGatherer,
        combat_unit: UnitKind::ElvenWarden,
        heavy_unit: UnitKind::GroveSentinel,
        specialist_unit: Some(UnitKind::WizardAdept),
        basic_tech: TechKind::LivingBark,
        heavy_tech: TechKind::RootguardPact,
        specialist_tech: Some(TechKind::AstralChanneling),
        advanced_units: AETHERBORN_ADVANCED_UNITS,
        tech_plan: AETHERBORN_TECH_PLAN,
    },
    RaceCommandCatalog {
        race: RaceId::Terran,
        supply_building: BuildingKind::SupplyPylon,
        production_building: BuildingKind::FabricatorBay,
        advanced_production_building: BuildingKind::MechFoundry,
        research_building: BuildingKind::TacticalLab,
        ley_claim_building: BuildingKind::AetherExtractorRig,
        aether_link_building: BuildingKind::BatteryDepot,
        worker_unit: UnitKind::FieldEngineer,
        combat_unit: UnitKind::RangerTrooper,
        heavy_unit: UnitKind::AegisWalker,
        specialist_unit: None,
        basic_tech: TechKind::StabilizedBarrels,
        heavy_tech: TechKind::AegisFrame,
        specialist_tech: None,
        advanced_units: TERRAN_ADVANCED_UNITS,
        tech_plan: TERRAN_TECH_PLAN,
    },
];

impl UnitKind {
    pub fn definition(self) -> &'static UnitDefinition {
        unit_definition(self)
    }

    pub fn supply_cost(self) -> u32 {
        self.definition().supply_cost
    }

    pub fn matter_cost(self) -> u32 {
        self.definition().matter_cost
    }

    pub fn production_time(self) -> f32 {
        self.definition().production_time
    }

    pub fn base_stats(self) -> UnitStats {
        self.definition().base_stats
    }

    pub fn race(self) -> RaceId {
        self.definition().race
    }

    pub fn is_worker(self) -> bool {
        matches!(self, UnitKind::SpriteGatherer | UnitKind::FieldEngineer)
    }

    pub fn required_tech(self) -> Option<TechKind> {
        self.definition().required_tech
    }
}

impl BuildingKind {
    pub fn definition(self) -> &'static BuildingDefinition {
        building_definition(self)
    }

    pub fn race(self) -> RaceId {
        self.definition().race
    }

    pub fn matter_cost(self) -> u32 {
        self.definition().matter_cost
    }

    pub fn build_time(self) -> f32 {
        self.definition().build_time
    }

    pub fn supply_provided(self) -> u32 {
        self.definition().supply_provided
    }

    pub fn can_produce(self, unit_kind: UnitKind) -> bool {
        self.definition().produces.contains(&unit_kind)
    }

    pub fn can_research(self, tech_kind: TechKind) -> bool {
        self.definition().researches.contains(&tech_kind)
    }

    pub(crate) fn can_claim_ley_node(self) -> bool {
        self.definition().can_claim_ley_node
    }

    pub(crate) fn is_aetherborn_ritual_network_member(self) -> bool {
        self.definition().is_aetherborn_ritual_network_member
    }

    pub(crate) fn is_terran_battery_destination(self) -> bool {
        self.definition().is_terran_battery_destination
    }

    pub(crate) fn armor(self) -> u32 {
        self.definition().armor
    }

    pub fn max_health(self) -> f32 {
        self.definition().max_health
    }

    pub fn is_main_base(self) -> bool {
        self.definition().is_main_base
    }
}

impl TechKind {
    pub fn definition(self) -> &'static TechDefinition {
        tech_definition(self)
    }

    pub fn race(self) -> RaceId {
        self.definition().race
    }

    pub fn matter_cost(self) -> u32 {
        self.definition().matter_cost
    }

    pub fn research_time(self) -> f32 {
        self.definition().research_time
    }

    pub fn stat_bonuses(self) -> &'static [TechStatBonus] {
        self.definition().stat_bonuses
    }
}

pub fn unit_definition(kind: UnitKind) -> &'static UnitDefinition {
    UNIT_DEFINITIONS
        .iter()
        .find(|definition| definition.kind == kind)
        .expect("unit kind is missing a definition")
}

pub fn building_definition(kind: BuildingKind) -> &'static BuildingDefinition {
    BUILDING_DEFINITIONS
        .iter()
        .find(|definition| definition.kind == kind)
        .expect("building kind is missing a definition")
}

pub fn tech_definition(kind: TechKind) -> &'static TechDefinition {
    TECH_DEFINITIONS
        .iter()
        .find(|definition| definition.kind == kind)
        .expect("tech kind is missing a definition")
}

pub fn command_catalog_for(race: RaceId) -> &'static RaceCommandCatalog {
    RACE_COMMAND_CATALOGS
        .iter()
        .find(|catalog| catalog.race == race)
        .expect("race is missing a command catalog")
}
