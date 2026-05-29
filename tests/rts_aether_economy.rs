use fracture_command::rts::{BuildingKind, RtsGameState, PLAYER_ONE, PLAYER_TWO};
use macroquad::prelude::vec2;

const TEST_DT: f32 = 0.25;

fn run_for(state: &mut RtsGameState, seconds: f32) {
    let steps = (seconds / TEST_DT).ceil() as usize;
    for _ in 0..steps {
        state.update(TEST_DT);
    }
}

#[test]
fn aetherborn_connected_ley_shrine_generates_aether_and_flow() {
    let mut state = RtsGameState::new_two_player_test_match();
    let ley_node = state.add_ley_node(vec2(520.0, 300.0));

    state.add_completed_building(PLAYER_ONE, BuildingKind::RitualNode, vec2(360.0, 300.0));
    state
        .add_completed_building_on_node(PLAYER_ONE, BuildingKind::LeyShrine, ley_node)
        .unwrap();

    run_for(&mut state, 1.0);

    assert_eq!(state.players[PLAYER_ONE].resources.aether, 4);
    assert_eq!(state.players[PLAYER_ONE].ley_flow_capacity, 3);
}

#[test]
fn aetherborn_disconnected_ley_shrine_does_not_generate_aether() {
    let mut state = RtsGameState::new_two_player_test_match();
    let ley_node = state.add_ley_node(vec2(720.0, 300.0));

    state
        .add_completed_building_on_node(PLAYER_ONE, BuildingKind::LeyShrine, ley_node)
        .unwrap();

    run_for(&mut state, 3.0);

    assert_eq!(state.players[PLAYER_ONE].resources.aether, 0);
    assert_eq!(state.players[PLAYER_ONE].ley_flow_capacity, 0);
}

#[test]
fn aetherborn_broken_ritual_chain_stops_income() {
    let mut state = RtsGameState::new_two_player_test_match();
    let ley_node = state.add_ley_node(vec2(520.0, 300.0));
    let ritual_id =
        state.add_completed_building(PLAYER_ONE, BuildingKind::RitualNode, vec2(360.0, 300.0));

    state
        .add_completed_building_on_node(PLAYER_ONE, BuildingKind::LeyShrine, ley_node)
        .unwrap();
    run_for(&mut state, 1.0);
    let aether_before_break = state.players[PLAYER_ONE].resources.aether;

    let ritual = state
        .buildings
        .iter_mut()
        .find(|building| building.id == ritual_id)
        .unwrap();
    ritual.completed = false;

    run_for(&mut state, 2.0);

    assert_eq!(
        state.players[PLAYER_ONE].resources.aether,
        aether_before_break
    );
    assert_eq!(state.players[PLAYER_ONE].ley_flow_capacity, 0);
}

#[test]
fn terran_extractor_generates_only_with_battery_route() {
    let mut state = RtsGameState::new_two_player_test_match();
    let ley_node = state.add_ley_node(vec2(650.0, 300.0));

    state
        .add_completed_building_on_node(PLAYER_TWO, BuildingKind::AetherExtractorRig, ley_node)
        .unwrap();
    run_for(&mut state, 2.0);
    assert_eq!(state.players[PLAYER_TWO].resources.aether, 0);

    state.add_completed_building(PLAYER_TWO, BuildingKind::BatteryDepot, vec2(720.0, 300.0));
    run_for(&mut state, 1.0);

    assert_eq!(state.players[PLAYER_TWO].resources.aether, 5);
}

#[test]
fn disabled_battery_depot_stops_terran_aether_delivery() {
    let mut state = RtsGameState::new_two_player_test_match();
    let ley_node = state.add_ley_node(vec2(650.0, 300.0));

    state
        .add_completed_building_on_node(PLAYER_TWO, BuildingKind::AetherExtractorRig, ley_node)
        .unwrap();
    let depot_id =
        state.add_completed_building(PLAYER_TWO, BuildingKind::BatteryDepot, vec2(720.0, 300.0));
    run_for(&mut state, 1.0);
    let aether_before_break = state.players[PLAYER_TWO].resources.aether;

    let depot = state
        .buildings
        .iter_mut()
        .find(|building| building.id == depot_id)
        .unwrap();
    depot.completed = false;

    run_for(&mut state, 2.0);

    assert_eq!(
        state.players[PLAYER_TWO].resources.aether,
        aether_before_break
    );
}
