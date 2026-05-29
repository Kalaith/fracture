use super::{
    BuildingInstance, BuildingKind, EntityId, RaceId, ResourceNodeId, ResourceNodeKind,
    RtsGameState, UnitCommand, UnitKind,
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
        self.train_combat_unit(state);
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
        let worker_kind = worker_unit_for(state.players[self.player_id].race);
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

        let kind = supply_building_for(state.players[self.player_id].race);
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
        let production_kind = production_building_for(race);
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
        let has_shrine = state.buildings.iter().any(|building| {
            building.owner == self.player_id && building.kind == BuildingKind::LeyShrine
        });
        let has_ritual_node = state.buildings.iter().any(|building| {
            building.owner == self.player_id && building.kind == BuildingKind::RitualNode
        });

        if !has_shrine {
            self.try_claim_ley_node(state, BuildingKind::LeyShrine);
            return;
        }

        if !has_ritual_node {
            let Some(shrine) = first_owned_building(state, self.player_id, BuildingKind::LeyShrine)
            else {
                return;
            };
            let Some(base_position) = main_base_position(state, self.player_id) else {
                return;
            };

            let position = (base_position + shrine.position) * 0.5;
            self.try_construct_at(state, BuildingKind::RitualNode, position);
        }
    }

    fn ensure_terran_battery_route(&self, state: &mut RtsGameState) {
        let has_extractor = state.buildings.iter().any(|building| {
            building.owner == self.player_id && building.kind == BuildingKind::AetherExtractorRig
        });
        let has_depot = state.buildings.iter().any(|building| {
            building.owner == self.player_id && building.kind == BuildingKind::BatteryDepot
        });

        if !has_extractor {
            self.try_claim_ley_node(state, BuildingKind::AetherExtractorRig);
            return;
        }

        if !has_depot {
            let Some(extractor) =
                first_owned_building(state, self.player_id, BuildingKind::AetherExtractorRig)
            else {
                return;
            };
            self.try_construct_at(
                state,
                BuildingKind::BatteryDepot,
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

        let _ = state.start_construction(worker_id, building_kind, position);
    }

    fn train_combat_unit(&self, state: &mut RtsGameState) {
        let race = state.players[self.player_id].race;
        let production_kind = production_building_for(race);
        let unit_kind = combat_unit_for(race);

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

fn production_building_for(race: RaceId) -> BuildingKind {
    match race {
        RaceId::Aetherborn => BuildingKind::GroveCircle,
        RaceId::Terran => BuildingKind::FabricatorBay,
    }
}

fn supply_building_for(race: RaceId) -> BuildingKind {
    match race {
        RaceId::Aetherborn => BuildingKind::Moonwell,
        RaceId::Terran => BuildingKind::SupplyPylon,
    }
}

fn worker_unit_for(race: RaceId) -> UnitKind {
    match race {
        RaceId::Aetherborn => UnitKind::SpriteGatherer,
        RaceId::Terran => UnitKind::FieldEngineer,
    }
}

fn combat_unit_for(race: RaceId) -> UnitKind {
    match race {
        RaceId::Aetherborn => UnitKind::ElvenWarden,
        RaceId::Terran => UnitKind::RangerTrooper,
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
            base_position
                .distance(left.position)
                .total_cmp(&base_position.distance(right.position))
        })
        .map(|node| node.id)
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
