use fracture_command::rts::{
    BuildingKind, RtsError, RtsGameState, UnitKind, PLAYER_ONE, PLAYER_TWO,
};
use macroquad::prelude::vec2;

const TEST_DT: f32 = 0.25;

fn run_for(state: &mut RtsGameState, seconds: f32) {
    let steps = (seconds / TEST_DT).ceil() as usize;

    for _ in 0..steps {
        state.update(TEST_DT);
    }
}

#[test]
fn worker_gathers_matter_from_node() {
    let mut state = RtsGameState::new_two_player_test_match();
    let worker_id = state.first_worker_for(PLAYER_ONE).unwrap();
    let node_id = state.first_matter_node().unwrap();

    let starting_matter = state.players[PLAYER_ONE].resources.matter;
    let starting_node_matter = state.resource_node(node_id).unwrap().remaining;

    state.command_gather_matter(worker_id, node_id).unwrap();
    run_for(&mut state, 2.0);

    assert!(state.players[PLAYER_ONE].resources.matter > starting_matter);
    assert!(state.resource_node(node_id).unwrap().remaining < starting_node_matter);
}

#[test]
fn worker_constructs_supply_building() {
    let mut state = RtsGameState::new_two_player_test_match();
    let worker_id = state.first_worker_for(PLAYER_ONE).unwrap();
    let starting_supply_cap = state.players[PLAYER_ONE].supply_cap;
    let starting_matter = state.players[PLAYER_ONE].resources.matter;

    let building_id = state
        .start_construction(worker_id, BuildingKind::Moonwell, vec2(240.0, 320.0))
        .unwrap();

    assert!(state.players[PLAYER_ONE].resources.matter < starting_matter);
    assert!(state
        .buildings
        .iter()
        .any(|building| building.id == building_id && !building.completed));

    run_for(&mut state, BuildingKind::Moonwell.build_time() + TEST_DT);

    assert_eq!(
        starting_supply_cap + BuildingKind::Moonwell.supply_provided(),
        state.players[PLAYER_ONE].supply_cap
    );
    assert_eq!(
        1,
        state
            .completed_buildings_for(PLAYER_ONE, BuildingKind::Moonwell)
            .count()
    );
}

#[test]
fn cannot_train_when_supply_blocked() {
    let mut state = RtsGameState::new_two_player_test_match();
    state.players[PLAYER_TWO].resources.matter = 500;
    state.players[PLAYER_TWO].supply_used = state.players[PLAYER_TWO].supply_cap;

    let result = state.can_train_unit(PLAYER_TWO, UnitKind::RangerTrooper);

    assert_eq!(Err(RtsError::SupplyBlocked), result);
}
