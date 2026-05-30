use super::{
    command_catalog_for, BuildingInstance, BuildingKind, EntityId, RaceId, ResourceNodeId,
    ResourceNodeKind, RtsGameState, UnitCommand,
};
use macroquad::prelude::{vec2, Vec2};

const TARGET_WORKERS: usize = 6;

#[derive(Debug, Clone)]
pub struct BasicSkirmishAi {
    pub player_id: usize,
    pub attack_wave_size: usize,
    decision_timer: f32,
}

impl BasicSkirmishAi {
    pub fn new(player_id: usize) -> Self {
        Self {
            player_id,
            attack_wave_size: 3,
            decision_timer: 0.0,
        }
    }

    pub fn update(&mut self, state: &mut RtsGameState, dt: f32) {
        if state.winner.is_some() || state.players.get(self.player_id).is_none() {
            return;
        }

        self.decision_timer = (self.decision_timer - dt).max(0.0);
        if self.decision_timer > 0.0 {
            return;
        }
        self.decision_timer = 1.0;

        self.command_workers_to_gather(state);
        self.train_worker_if_needed(state);
        self.ensure_supply_if_needed(state);
        self.ensure_production_building(state);
        self.ensure_aether_infrastructure(state);
        self.ensure_research_building(state);
        self.ensure_advanced_production_building(state);
        self.research_available_tech(state);
        self.train_combat_unit(state);
        self.train_advanced_unit(state);
        self.send_attack_wave(state);
    }

    fn command_workers_to_gather(&self, state: &mut RtsGameState) {
        let orders: Vec<(EntityId, ResourceNodeId)> = state
            .units
            .iter()
            .filter(|unit| {
                unit.owner == self.player_id
                    && unit.kind.is_worker()
                    && matches!(unit.command, UnitCommand::Idle)
            })
            .filter_map(|unit| {
                nearest_matter_node(state, unit.position).map(|node_id| (unit.id, node_id))
            })
            .collect();

        for (worker_id, node_id) in orders {
            let _ = state.command_gather_matter(worker_id, node_id);
        }
    }

    fn train_worker_if_needed(&self, state: &mut RtsGameState) {
        let worker_kind = command_catalog_for(state.players[self.player_id].race).worker_unit;
        let worker_count = state
            .units
            .iter()
            .filter(|unit| unit.owner == self.player_id && unit.kind == worker_kind)
            .count();

        if worker_count >= TARGET_WORKERS {
            return;
        }

        let Some(main_base_id) = main_base_id(state, self.player_id) else {
            return;
        };
        let main_base_busy = state
            .buildings
            .iter()
            .find(|building| building.id == main_base_id)
            .is_some_and(|building| !building.production_queue.is_empty());

        if !main_base_busy && state.can_train_unit(self.player_id, worker_kind).is_ok() {
            let _ = state.train_unit(main_base_id, worker_kind);
        }
    }

    fn ensure_supply_if_needed(&self, state: &mut RtsGameState) {
        if state.players[self.player_id].available_supply() > 2
            || has_incomplete_building(state, self.player_id)
        {
            return;
        }

        let kind = command_catalog_for(state.players[self.player_id].race).supply_building;
        if state.players[self.player_id].resources.matter < kind.matter_cost() {
            return;
        }

        let Some(worker_id) = worker_available_for_construction(state, self.player_id) else {
            return;
        };
        let position = main_base_position(state, self.player_id).unwrap_or(vec2(0.0, 0.0))
            + supply_offset_for(state.players[self.player_id].race);
        let _ = state.start_construction(worker_id, kind, position);
    }

    fn ensure_production_building(&self, state: &mut RtsGameState) {
        let race = state.players[self.player_id].race;
        let production_kind = command_catalog_for(race).production_building;
        let already_exists = state
            .buildings
            .iter()
            .any(|building| building.owner == self.player_id && building.kind == production_kind);

        if already_exists
            || has_incomplete_building(state, self.player_id)
            || state.players[self.player_id].resources.matter < production_kind.matter_cost()
        {
            return;
        }

        let Some(worker_id) = worker_available_for_construction(state, self.player_id) else {
            return;
        };

        let position = main_base_position(state, self.player_id).unwrap_or(vec2(0.0, 0.0))
            + production_offset_for(race);
        let _ = state.start_construction(worker_id, production_kind, position);
    }

    fn ensure_research_building(&self, state: &mut RtsGameState) {
        let race = state.players[self.player_id].race;
        let kind = command_catalog_for(race).research_building;
        let already_exists = state
            .buildings
            .iter()
            .any(|building| building.owner == self.player_id && building.kind == kind);

        if already_exists
            || has_incomplete_building(state, self.player_id)
            || state.players[self.player_id].resources.matter < kind.matter_cost()
        {
            return;
        }

        let position = main_base_position(state, self.player_id).unwrap_or(vec2(0.0, 0.0))
            + research_offset_for(race);
        self.try_construct_at(state, kind, position);
    }

    fn ensure_advanced_production_building(&self, state: &mut RtsGameState) {
        let race = state.players[self.player_id].race;
        let kind = command_catalog_for(race).advanced_production_building;
        let already_exists = state
            .buildings
            .iter()
            .any(|building| building.owner == self.player_id && building.kind == kind);

        if already_exists
            || has_incomplete_building(state, self.player_id)
            || state.players[self.player_id].resources.matter < kind.matter_cost()
        {
            return;
        }

        let position = main_base_position(state, self.player_id).unwrap_or(vec2(0.0, 0.0))
            + advanced_production_offset_for(race);
        self.try_construct_at(state, kind, position);
    }

    fn ensure_aether_infrastructure(&self, state: &mut RtsGameState) {
        if has_incomplete_building(state, self.player_id) {
            return;
        }

        match state.players[self.player_id].race {
            RaceId::Aetherborn => self.ensure_aetherborn_ley_network(state),
            RaceId::Terran => self.ensure_terran_battery_route(state),
        }
    }

    fn ensure_aetherborn_ley_network(&self, state: &mut RtsGameState) {
        let catalog = command_catalog_for(state.players[self.player_id].race);
        let has_shrine = state.buildings.iter().any(|building| {
            building.owner == self.player_id && building.kind == catalog.ley_claim_building
        });
        let has_ritual_node = state.buildings.iter().any(|building| {
            building.owner == self.player_id && building.kind == catalog.aether_link_building
        });

        if !has_shrine {
            self.try_claim_ley_node(state, catalog.ley_claim_building);
            return;
        }

        if !has_ritual_node {
            let Some(shrine) = first_owned_building(
                state,
                self.player_id,
                catalog.ley_claim_building,
            ) else {
                return;
            };
            let Some(base_position) = main_base_position(state, self.player_id) else {
                return;
            };

            let position = (base_position + shrine.position) * 0.5;
            self.try_construct_at(state, catalog.aether_link_building, position);
        }
    }

    fn ensure_terran_battery_route(&self, state: &mut RtsGameState) {
        let catalog = command_catalog_for(state.players[self.player_id].race);
        let has_extractor = state.buildings.iter().any(|building| {
            building.owner == self.player_id && building.kind == catalog.ley_claim_building
        });
        let has_depot = state.buildings.iter().any(|building| {
            building.owner == self.player_id && building.kind == catalog.aether_link_building
        });

        if !has_extractor {
            self.try_claim_ley_node(state, catalog.ley_claim_building);
            return;
        }

        if !has_depot {
            let Some(extractor) =
                first_owned_building(state, self.player_id, catalog.ley_claim_building)
            else {
                return;
            };
            self.try_construct_at(
                state,
                catalog.aether_link_building,
                extractor.position + vec2(70.0, 0.0),
            );
        }
    }

    fn try_claim_ley_node(&self, state: &mut RtsGameState, building_kind: BuildingKind) {
        if state.players[self.player_id].resources.matter < building_kind.matter_cost() {
            return;
        }

        let Some(worker_id) = worker_available_for_construction(state, self.player_id) else {
            return;
        };
        let Some(node_id) = nearest_unclaimed_ley_node(state, self.player_id) else {
            return;
        };

        let _ = state.start_construction_on_resource_node(worker_id, building_kind, node_id);
    }

    fn try_construct_at(
        &self,
        state: &mut RtsGameState,
        building_kind: BuildingKind,
        position: Vec2,
    ) {
        if state.players[self.player_id].resources.matter < building_kind.matter_cost() {
            return;
        }

        let Some(worker_id) = worker_available_for_construction(state, self.player_id) else {
            return;
        };

        let position = preferred_build_position(state, self.player_id, position);
        let _ = state.start_construction(worker_id, building_kind, position);
    }

    fn train_combat_unit(&self, state: &mut RtsGameState) {
        let race = state.players[self.player_id].race;
        let catalog = command_catalog_for(race);
        let production_kind = catalog.production_building;
        let unit_kind = catalog.combat_unit;

        let producers: Vec<EntityId> = state
            .buildings
            .iter()
            .filter(|building| {
                building.owner == self.player_id
                    && building.kind == production_kind
                    && building.completed
                    && building.production_queue.is_empty()
            })
            .map(|building| building.id)
            .collect();

        for producer_id in producers {
            if state.can_train_unit(self.player_id, unit_kind).is_ok() {
                let _ = state.train_unit(producer_id, unit_kind);
            }
        }
    }

    fn research_available_tech(&self, state: &mut RtsGameState) {
        let race = state.players[self.player_id].race;
        let catalog = command_catalog_for(race);
        let research_building_kind = catalog.research_building;
        let tech_plan = catalog.tech_plan;

        let researchers: Vec<EntityId> = state
            .buildings
            .iter()
            .filter(|building| {
                building.owner == self.player_id
                    && building.completed
                    && building.research_queue.is_empty()
                    && (building.kind == catalog.production_building
                        || building.kind == research_building_kind)
            })
            .map(|building| building.id)
            .collect();

        for researcher_id in researchers {
            for &tech_kind in tech_plan {
                if state.players[self.player_id].has_tech(tech_kind) {
                    continue;
                }

                if state.research_tech(researcher_id, tech_kind).is_ok() {
                    break;
                }
            }
        }
    }

    fn train_advanced_unit(&self, state: &mut RtsGameState) {
        let race = state.players[self.player_id].race;
        let catalog = command_catalog_for(race);
        let producer_kind = catalog.advanced_production_building;
        let advanced_units = catalog.advanced_units;

        let producers: Vec<EntityId> = state
            .buildings
            .iter()
            .filter(|building| {
                building.owner == self.player_id
                    && building.kind == producer_kind
                    && building.completed
                    && building.production_queue.is_empty()
            })
            .map(|building| building.id)
            .collect();

        for producer_id in producers {
            for &unit_kind in advanced_units {
                if state.can_train_unit(self.player_id, unit_kind).is_ok() {
                    let _ = state.train_unit(producer_id, unit_kind);
                    break;
                }
            }
        }
    }

    fn send_attack_wave(&self, state: &mut RtsGameState) {
        let Some(target_id) = enemy_main_base_id(state, self.player_id) else {
            return;
        };

        let combat_units: Vec<EntityId> = state
            .units
            .iter()
            .filter(|unit| unit.owner == self.player_id && !unit.kind.is_worker())
            .map(|unit| unit.id)
            .collect();

        if combat_units.len() < self.attack_wave_size {
            return;
        }

        for unit_id in combat_units {
            let already_attacking = state.units.iter().any(|unit| {
                unit.id == unit_id
                    && matches!(unit.command, UnitCommand::AttackBuilding(id) if id == target_id)
            });

            if !already_attacking {
                let _ = state.command_attack_building(unit_id, target_id);
            }
        }
    }
}

fn supply_offset_for(race: RaceId) -> Vec2 {
    match race {
        RaceId::Aetherborn => vec2(70.0, 58.0),
        RaceId::Terran => vec2(-70.0, 58.0),
    }
}

fn production_offset_for(race: RaceId) -> Vec2 {
    match race {
        RaceId::Aetherborn => vec2(80.0, -64.0),
        RaceId::Terran => vec2(-80.0, -64.0),
    }
}

fn advanced_production_offset_for(race: RaceId) -> Vec2 {
    match race {
        RaceId::Aetherborn => vec2(155.0, -120.0),
        RaceId::Terran => vec2(-155.0, -120.0),
    }
}

fn research_offset_for(race: RaceId) -> Vec2 {
    match race {
        RaceId::Aetherborn => vec2(150.0, 40.0),
        RaceId::Terran => vec2(-150.0, 40.0),
    }
}

fn worker_available_for_construction(state: &RtsGameState, player_id: usize) -> Option<EntityId> {
    state
        .units
        .iter()
        .find(|unit| {
            unit.owner == player_id
                && unit.kind.is_worker()
                && !matches!(unit.command, UnitCommand::Build(_))
        })
        .map(|unit| unit.id)
}

fn has_incomplete_building(state: &RtsGameState, player_id: usize) -> bool {
    state
        .buildings
        .iter()
        .any(|building| building.owner == player_id && !building.completed)
}

fn first_owned_building(
    state: &RtsGameState,
    player_id: usize,
    kind: BuildingKind,
) -> Option<&BuildingInstance> {
    state
        .buildings
        .iter()
        .find(|building| building.owner == player_id && building.kind == kind)
}

fn nearest_unclaimed_ley_node(state: &RtsGameState, player_id: usize) -> Option<ResourceNodeId> {
    let base_position = main_base_position(state, player_id)?;
    let race = state.players[player_id].race;
    let expansion_positions: Vec<Vec2> = state
        .expansion_markers
        .iter()
        .filter(|marker| marker.recommended_for.map_or(true, |recommended| recommended == race))
        .map(|marker| marker.position.as_vec2())
        .collect();

    state
        .resource_nodes
        .iter()
        .filter(|node| node.kind == ResourceNodeKind::Ley)
        .filter(|node| {
            state
                .buildings
                .iter()
                .all(|building| building.claimed_node != Some(node.id))
        })
        .min_by(|left, right| {
            ley_node_expansion_score(left.position, base_position, &expansion_positions)
                .total_cmp(&ley_node_expansion_score(
                    right.position,
                    base_position,
                    &expansion_positions,
                ))
        })
        .map(|node| node.id)
}

fn preferred_build_position(state: &RtsGameState, player_id: usize, desired: Vec2) -> Vec2 {
    if state.can_place_standard_building(desired) {
        return desired;
    }

    let race = state.players[player_id].race;
    state
        .expansion_markers
        .iter()
        .filter(|marker| marker.recommended_for.map_or(true, |recommended| recommended == race))
        .map(|marker| marker.position.as_vec2())
        .filter(|position| state.can_place_standard_building(*position))
        .min_by(|left, right| {
            desired
                .distance(*left)
                .total_cmp(&desired.distance(*right))
        })
        .unwrap_or(desired)
}

fn ley_node_expansion_score(position: Vec2, base_position: Vec2, expansions: &[Vec2]) -> f32 {
    let expansion_score = expansions
        .iter()
        .map(|expansion| position.distance(*expansion))
        .min_by(|left, right| left.total_cmp(right))
        .unwrap_or(0.0);

    expansion_score + base_position.distance(position) * 0.25
}

fn main_base_id(state: &RtsGameState, player_id: usize) -> Option<EntityId> {
    state
        .buildings
        .iter()
        .find(|building| {
            building.owner == player_id && building.completed && building.kind.is_main_base()
        })
        .map(|building| building.id)
}

fn nearest_matter_node(state: &RtsGameState, position: Vec2) -> Option<ResourceNodeId> {
    state
        .resource_nodes
        .iter()
        .filter(|node| node.kind == ResourceNodeKind::Matter && node.remaining > 0)
        .min_by(|left, right| {
            position
                .distance(left.position)
                .total_cmp(&position.distance(right.position))
        })
        .map(|node| node.id)
}

fn main_base_position(state: &RtsGameState, player_id: usize) -> Option<Vec2> {
    state
        .buildings
        .iter()
        .find(|building| {
            building.owner == player_id && building.completed && building.kind.is_main_base()
        })
        .map(|building| building.position)
}

fn enemy_main_base_id(state: &RtsGameState, player_id: usize) -> Option<EntityId> {
    state
        .buildings
        .iter()
        .find(|building| {
            building.owner != player_id && building.completed && building.kind.is_main_base()
        })
        .map(|building| building.id)
}
