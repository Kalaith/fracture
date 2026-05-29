use fracture_command::rts::{BuildingKind, RtsGameState, UnitKind, PLAYER_ONE, PLAYER_TWO};
use macroquad::prelude::{vec2, Vec2};

const TEST_DT: f32 = 0.25;

fn run_for(state: &mut RtsGameState, seconds: f32) {
    let steps = (seconds / TEST_DT).ceil() as usize;
    for _ in 0..steps {
        state.update(TEST_DT);
    }
}

fn unit_health(state: &RtsGameState, unit_id: fracture_command::rts::EntityId) -> Option<f32> {
    state
        .units
        .iter()
        .find(|unit| unit.id == unit_id)
        .map(|unit| unit.health)
}

fn spawn_attackers(
    state: &mut RtsGameState,
    player_id: usize,
    unit_kind: UnitKind,
    count: usize,
    start: Vec2,
) -> Vec<fracture_command::rts::EntityId> {
    (0..count)
        .map(|index| state.add_unit(player_id, unit_kind, start + vec2(0.0, index as f32 * 8.0)))
        .collect()
}

#[test]
fn units_attack_enemy_in_range() {
    let mut state = RtsGameState::new_two_player_test_match();
    let ranger = state.add_unit(PLAYER_TWO, UnitKind::RangerTrooper, vec2(450.0, 300.0));
    let warden = state.add_unit(PLAYER_ONE, UnitKind::ElvenWarden, vec2(560.0, 300.0));

    state.command_attack_unit(ranger, warden).unwrap();
    state.update(TEST_DT);

    let warden_health = unit_health(&state, warden).unwrap();
    let warden_max_health = state
        .unit_stats_for_player(PLAYER_ONE, UnitKind::ElvenWarden)
        .max_health as f32;

    assert!(warden_health < warden_max_health);
}

#[test]
fn workers_damage_enemy_workers_when_ordered_to_attack() {
    let mut state = RtsGameState::new_two_player_test_match();
    let sprite = state.add_unit(PLAYER_ONE, UnitKind::SpriteGatherer, vec2(500.0, 300.0));
    let engineer = state.add_unit(PLAYER_TWO, UnitKind::FieldEngineer, vec2(555.0, 300.0));

    state.command_attack_unit(sprite, engineer).unwrap();
    state.update(TEST_DT);

    let engineer_health = unit_health(&state, engineer).unwrap();
    let engineer_max_health = state
        .unit_stats_for_player(PLAYER_TWO, UnitKind::FieldEngineer)
        .max_health as f32;

    assert!(engineer_health < engineer_max_health);
}

#[test]
fn attack_move_closes_distance_and_engages() {
    let mut state = RtsGameState::new_two_player_test_match();
    let ranger = state.add_unit(PLAYER_TWO, UnitKind::RangerTrooper, vec2(600.0, 300.0));
    let warden = state.add_unit(PLAYER_ONE, UnitKind::ElvenWarden, vec2(900.0, 300.0));

    state
        .command_attack_move_unit(ranger, vec2(900.0, 300.0))
        .unwrap();
    run_for(&mut state, 3.0);

    let ranger_position = state
        .units
        .iter()
        .find(|unit| unit.id == ranger)
        .unwrap()
        .position;
    let warden_health = unit_health(&state, warden).unwrap();
    let warden_max_health = state
        .unit_stats_for_player(PLAYER_ONE, UnitKind::ElvenWarden)
        .max_health as f32;

    assert!(ranger_position.x > 600.0);
    assert!(warden_health < warden_max_health);
}

#[test]
fn ranger_troopers_can_kill_warden_under_controlled_conditions() {
    let mut state = RtsGameState::new_two_player_test_match();
    let rangers = spawn_attackers(
        &mut state,
        PLAYER_TWO,
        UnitKind::RangerTrooper,
        3,
        vec2(430.0, 280.0),
    );
    let warden = state.add_unit(PLAYER_ONE, UnitKind::ElvenWarden, vec2(540.0, 300.0));

    for ranger in rangers {
        state.command_attack_unit(ranger, warden).unwrap();
    }
    run_for(&mut state, 8.0);

    assert!(state.units.iter().all(|unit| unit.id != warden));
}

#[test]
fn destroying_non_main_building_does_not_set_winner() {
    let mut state = RtsGameState::new_two_player_test_match();
    let supply_pylon =
        state.add_completed_building(PLAYER_TWO, BuildingKind::SupplyPylon, vec2(520.0, 300.0));
    let wardens = spawn_attackers(
        &mut state,
        PLAYER_ONE,
        UnitKind::ElvenWarden,
        6,
        vec2(410.0, 280.0),
    );

    for warden in wardens {
        state.command_attack_building(warden, supply_pylon).unwrap();
    }
    run_for(&mut state, 10.0);

    assert!(state
        .buildings
        .iter()
        .all(|building| building.id != supply_pylon));
    assert_eq!(state.winner, None);
}

#[test]
fn destroying_enemy_main_base_sets_winner() {
    let mut state = RtsGameState::new_two_player_test_match();
    let command_ark = state
        .first_completed_building_for(PLAYER_TWO, BuildingKind::CommandArk)
        .unwrap();
    let wardens = spawn_attackers(
        &mut state,
        PLAYER_ONE,
        UnitKind::ElvenWarden,
        12,
        vec2(880.0, 250.0),
    );

    for warden in wardens {
        state.command_attack_building(warden, command_ark).unwrap();
    }
    run_for(&mut state, 30.0);

    assert_eq!(state.winner, Some(PLAYER_ONE));
    assert!(state
        .buildings
        .iter()
        .all(|building| building.id != command_ark));
}
