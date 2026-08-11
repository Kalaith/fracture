use fracture_command::rts::{
    BasicSkirmishAi, BuildingKind, RaceId, RtsGameState, RtsMapDefinition, UnitKind, PLAYER_ONE,
    PLAYER_TWO,
};
use macroquad::prelude::{vec2, Vec2};

const CRASH_BASIN_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/rts_maps/crash_basin_skirmish.json");
const TEST_DT: f32 = 0.25;

fn crash_basin_state() -> RtsGameState {
    let map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON).unwrap();
    RtsGameState::from_map_definition(&map).unwrap()
}

fn run_for(state: &mut RtsGameState, seconds: f32) {
    let steps = (seconds / TEST_DT).ceil() as usize;
    for _ in 0..steps {
        state.update(TEST_DT);
    }
}

fn run_with_ai(state: &mut RtsGameState, ai: &mut BasicSkirmishAi, seconds: f32) {
    let steps = (seconds / TEST_DT).ceil() as usize;
    for _ in 0..steps {
        ai.update(state, TEST_DT);
        state.update(TEST_DT);
    }
}

fn first_worker(state: &RtsGameState, player_id: usize) -> fracture_command::rts::EntityId {
    state
        .first_worker_for(player_id)
        .expect("player should start with worker")
}

fn command_all_workers_to_gather(state: &mut RtsGameState, player_id: usize) {
    let worker_ids: Vec<_> = state
        .units
        .iter()
        .filter(|unit| unit.owner == player_id && unit.kind.is_worker())
        .map(|unit| unit.id)
        .collect();
    let closest_node = state
        .resource_nodes
        .iter()
        .filter(|node| node.remaining > 0)
        .min_by(|left, right| {
            let base_position = main_base_position(state, player_id);
            base_position
                .distance(left.position)
                .total_cmp(&base_position.distance(right.position))
        })
        .unwrap()
        .id;

    for worker_id in worker_ids {
        state
            .command_gather_matter(worker_id, closest_node)
            .unwrap();
    }
}

fn build_production(state: &mut RtsGameState, player_id: usize) -> fracture_command::rts::EntityId {
    let race = state.players[player_id].race;
    let worker_id = first_worker(state, player_id);
    let kind = match race {
        RaceId::Aetherborn => BuildingKind::GroveCircle,
        RaceId::Terran => BuildingKind::FabricatorBay,
    };
    let offset = match race {
        RaceId::Aetherborn => vec2(90.0, -70.0),
        RaceId::Terran => vec2(-90.0, -70.0),
    };
    let building_id = state
        .start_construction(
            worker_id,
            kind,
            main_base_position(state, player_id) + offset,
        )
        .unwrap();

    run_for(state, kind.build_time() + TEST_DT);
    building_id
}

fn train_combat_units(
    state: &mut RtsGameState,
    player_id: usize,
    producer_id: fracture_command::rts::EntityId,
    count: usize,
) -> Vec<fracture_command::rts::EntityId> {
    let unit_kind = match state.players[player_id].race {
        RaceId::Aetherborn => UnitKind::ElvenWarden,
        RaceId::Terran => UnitKind::RangerTrooper,
    };
    let existing_units: Vec<_> = state
        .units
        .iter()
        .filter(|unit| unit.owner == player_id && unit.kind == unit_kind)
        .map(|unit| unit.id)
        .collect();

    for _ in 0..count {
        state.train_unit(producer_id, unit_kind).unwrap();
    }

    run_for(state, unit_kind.production_time() * count as f32 + TEST_DT);

    state
        .units
        .iter()
        .filter(|unit| {
            unit.owner == player_id && unit.kind == unit_kind && !existing_units.contains(&unit.id)
        })
        .map(|unit| unit.id)
        .collect()
}

fn attack_enemy_main_base(
    state: &mut RtsGameState,
    player_id: usize,
    units: &[fracture_command::rts::EntityId],
) {
    let target_id = state
        .buildings
        .iter()
        .find(|building| building.owner != player_id && building.kind.is_main_base())
        .unwrap()
        .id;

    for unit_id in units {
        state.command_attack_building(*unit_id, target_id).unwrap();
    }
}

fn main_base_position(state: &RtsGameState, player_id: usize) -> Vec2 {
    state
        .buildings
        .iter()
        .find(|building| building.owner == player_id && building.kind.is_main_base())
        .unwrap()
        .position
}

#[test]
fn aetherborn_player_can_complete_match_without_debug_hotkeys() {
    let mut state = crash_basin_state();

    command_all_workers_to_gather(&mut state, PLAYER_ONE);
    run_for(&mut state, 12.0);
    let producer_id = build_production(&mut state, PLAYER_ONE);
    let attackers = train_combat_units(&mut state, PLAYER_ONE, producer_id, 7);
    attack_enemy_main_base(&mut state, PLAYER_ONE, &attackers);
    run_for(&mut state, 50.0);

    assert_eq!(state.winner, Some(PLAYER_ONE));
}

#[test]
fn terran_player_can_complete_match_without_debug_hotkeys() {
    let mut state = crash_basin_state();

    command_all_workers_to_gather(&mut state, PLAYER_TWO);
    run_for(&mut state, 12.0);
    let producer_id = build_production(&mut state, PLAYER_TWO);
    let attackers = train_combat_units(&mut state, PLAYER_TWO, producer_id, 7);
    attack_enemy_main_base(&mut state, PLAYER_TWO, &attackers);
    run_for(&mut state, 55.0);

    assert_eq!(state.winner, Some(PLAYER_TWO));
}

#[test]
fn ignored_basic_ai_can_destroy_player_main_base() {
    let mut state = crash_basin_state();
    let mut ai = BasicSkirmishAi::new(PLAYER_TWO);

    run_with_ai(&mut state, &mut ai, 360.0);

    assert_eq!(state.winner, Some(PLAYER_TWO));
}
