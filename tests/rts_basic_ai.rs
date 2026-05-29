use fracture_command::rts::{
    BasicSkirmishAi, BuildingKind, RtsGameState, RtsMapDefinition, UnitCommand, UnitKind,
    PLAYER_ONE, PLAYER_TWO,
};

const CRASH_BASIN_JSON: &str = include_str!("../assets/data/rts_maps/crash_basin_skirmish.json");
const TEST_DT: f32 = 0.25;

fn crash_basin_state() -> RtsGameState {
    let map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON).unwrap();
    RtsGameState::from_map_definition(&map).unwrap()
}

fn run_ai_for(state: &mut RtsGameState, ai: &mut BasicSkirmishAi, seconds: f32) {
    let steps = (seconds / TEST_DT).ceil() as usize;
    for _ in 0..steps {
        ai.update(state, TEST_DT);
        state.update(TEST_DT);
    }
}

fn unit_count(state: &RtsGameState, player_id: usize, unit_kind: UnitKind) -> usize {
    state
        .units
        .iter()
        .filter(|unit| unit.owner == player_id && unit.kind == unit_kind)
        .count()
}

fn building_count(state: &RtsGameState, player_id: usize, building_kind: BuildingKind) -> usize {
    state
        .buildings
        .iter()
        .filter(|building| building.owner == player_id && building.kind == building_kind)
        .count()
}

#[test]
fn basic_ai_builds_workers_until_target_count() {
    let mut state = crash_basin_state();
    let mut ai = BasicSkirmishAi::new(PLAYER_TWO);

    run_ai_for(&mut state, &mut ai, 30.0);

    assert!(unit_count(&state, PLAYER_TWO, UnitKind::FieldEngineer) >= 6);
}

#[test]
fn basic_ai_builds_supply_when_blocked() {
    let mut state = crash_basin_state();
    let mut ai = BasicSkirmishAi::new(PLAYER_TWO);
    state.players[PLAYER_TWO].supply_cap = state.players[PLAYER_TWO].supply_used;
    state.players[PLAYER_TWO].resources.matter = 500;
    let starting_pylons = building_count(&state, PLAYER_TWO, BuildingKind::SupplyPylon);

    run_ai_for(
        &mut state,
        &mut ai,
        BuildingKind::SupplyPylon.build_time() + 2.0,
    );

    assert!(building_count(&state, PLAYER_TWO, BuildingKind::SupplyPylon) > starting_pylons);
}

#[test]
fn basic_ai_builds_aetherborn_ley_network() {
    let mut state = crash_basin_state();
    let mut ai = BasicSkirmishAi::new(PLAYER_ONE);

    run_ai_for(&mut state, &mut ai, 150.0);

    assert!(state
        .completed_buildings_for(PLAYER_ONE, BuildingKind::LeyShrine)
        .next()
        .is_some());
    assert!(state
        .completed_buildings_for(PLAYER_ONE, BuildingKind::RitualNode)
        .next()
        .is_some());
    assert!(state.players[PLAYER_ONE].resources.aether > 0);
    assert!(state.players[PLAYER_ONE].ley_flow_capacity > 0);
}

#[test]
fn basic_ai_builds_terran_battery_route() {
    let mut state = crash_basin_state();
    let mut ai = BasicSkirmishAi::new(PLAYER_TWO);

    run_ai_for(&mut state, &mut ai, 150.0);

    assert!(state
        .completed_buildings_for(PLAYER_TWO, BuildingKind::AetherExtractorRig)
        .next()
        .is_some());
    assert!(state
        .completed_buildings_for(PLAYER_TWO, BuildingKind::BatteryDepot)
        .next()
        .is_some());
    assert!(state.players[PLAYER_TWO].resources.aether > 0);
}

#[test]
fn basic_ai_trains_combat_units_without_cheating() {
    let mut state = crash_basin_state();
    let mut ai = BasicSkirmishAi::new(PLAYER_TWO);

    run_ai_for(&mut state, &mut ai, 90.0);

    assert!(state
        .completed_buildings_for(PLAYER_TWO, BuildingKind::FabricatorBay)
        .next()
        .is_some());
    assert!(unit_count(&state, PLAYER_TWO, UnitKind::RangerTrooper) > 0);
}

#[test]
fn basic_ai_sends_attack_wave_after_training_units() {
    let mut state = crash_basin_state();
    let mut ai = BasicSkirmishAi::new(PLAYER_TWO);

    run_ai_for(&mut state, &mut ai, 170.0);

    let attacking_units = state
        .units
        .iter()
        .filter(|unit| {
            unit.owner == PLAYER_TWO && matches!(unit.command, UnitCommand::AttackBuilding(_))
        })
        .count();

    assert!(attacking_units >= ai.attack_wave_size);
}

#[test]
fn basic_ai_can_damage_enemy_base_if_ignored() {
    let mut state = crash_basin_state();
    let mut ai = BasicSkirmishAi::new(PLAYER_TWO);
    let heartwood_id = state
        .first_completed_building_for(PLAYER_ONE, BuildingKind::HeartwoodNexus)
        .unwrap();
    let starting_health = state
        .buildings
        .iter()
        .find(|building| building.id == heartwood_id)
        .unwrap()
        .health;

    run_ai_for(&mut state, &mut ai, 220.0);

    let ending_health = state
        .buildings
        .iter()
        .find(|building| building.id == heartwood_id)
        .map(|building| building.health)
        .unwrap_or(0.0);

    assert!(ending_health < starting_health || state.winner == Some(PLAYER_TWO));
}
