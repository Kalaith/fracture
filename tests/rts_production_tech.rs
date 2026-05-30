use fracture_command::rts::{
    BuildingKind, RtsError, RtsGameState, TechKind, UnitKind, PLAYER_ONE, PLAYER_TWO,
};
use macroquad::prelude::vec2;

const TEST_DT: f32 = 0.25;

fn run_for(state: &mut RtsGameState, seconds: f32) {
    let steps = (seconds / TEST_DT).ceil() as usize;

    for _ in 0..steps {
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

#[test]
fn fabricator_trains_ranger_after_cost_paid() {
    let mut state = RtsGameState::new_two_player_test_match();
    let fabricator_id =
        state.add_completed_building(PLAYER_TWO, BuildingKind::FabricatorBay, vec2(960.0, 340.0));
    state.players[PLAYER_TWO].resources.matter = 500;

    let starting_matter = state.players[PLAYER_TWO].resources.matter;
    let starting_supply = state.players[PLAYER_TWO].supply_used;
    let starting_rangers = unit_count(&state, PLAYER_TWO, UnitKind::RangerTrooper);

    state
        .train_unit(fabricator_id, UnitKind::RangerTrooper)
        .unwrap();

    assert_eq!(
        starting_matter - UnitKind::RangerTrooper.matter_cost(),
        state.players[PLAYER_TWO].resources.matter
    );

    run_for(
        &mut state,
        UnitKind::RangerTrooper.production_time() + TEST_DT,
    );

    assert_eq!(
        starting_rangers + 1,
        unit_count(&state, PLAYER_TWO, UnitKind::RangerTrooper)
    );
    assert_eq!(
        starting_supply + UnitKind::RangerTrooper.supply_cost(),
        state.players[PLAYER_TWO].supply_used
    );
}

#[test]
fn grove_circle_trains_warden_after_cost_paid() {
    let mut state = RtsGameState::new_two_player_test_match();
    let grove_circle_id =
        state.add_completed_building(PLAYER_ONE, BuildingKind::GroveCircle, vec2(240.0, 340.0));
    state.players[PLAYER_ONE].resources.matter = 500;

    let starting_matter = state.players[PLAYER_ONE].resources.matter;
    let starting_supply = state.players[PLAYER_ONE].supply_used;
    let starting_wardens = unit_count(&state, PLAYER_ONE, UnitKind::ElvenWarden);

    state
        .train_unit(grove_circle_id, UnitKind::ElvenWarden)
        .unwrap();

    assert_eq!(
        starting_matter - UnitKind::ElvenWarden.matter_cost(),
        state.players[PLAYER_ONE].resources.matter
    );

    run_for(
        &mut state,
        UnitKind::ElvenWarden.production_time() + TEST_DT,
    );

    assert_eq!(
        starting_wardens + 1,
        unit_count(&state, PLAYER_ONE, UnitKind::ElvenWarden)
    );
    assert_eq!(
        starting_supply + UnitKind::ElvenWarden.supply_cost(),
        state.players[PLAYER_ONE].supply_used
    );
}

#[test]
fn tech_requires_correct_building() {
    let mut state = RtsGameState::new_two_player_test_match();
    let command_ark_id = state
        .first_completed_building_for(PLAYER_TWO, BuildingKind::CommandArk)
        .unwrap();
    state.players[PLAYER_TWO].resources.matter = 500;

    let result = state.research_tech(command_ark_id, TechKind::StabilizedBarrels);

    assert_eq!(Err(RtsError::UnsupportedResearch), result);
}

#[test]
fn researched_upgrade_modifies_unit_stats() {
    let mut state = RtsGameState::new_two_player_test_match();
    let fabricator_id =
        state.add_completed_building(PLAYER_TWO, BuildingKind::FabricatorBay, vec2(960.0, 340.0));
    state.players[PLAYER_TWO].resources.matter = 500;

    let starting_stats = state.unit_stats_for_player(PLAYER_TWO, UnitKind::RangerTrooper);

    state
        .research_tech(fabricator_id, TechKind::StabilizedBarrels)
        .unwrap();
    run_for(
        &mut state,
        TechKind::StabilizedBarrels.research_time() + TEST_DT,
    );

    assert!(state.players[PLAYER_TWO].has_tech(TechKind::StabilizedBarrels));

    let upgraded_stats = state.unit_stats_for_player(PLAYER_TWO, UnitKind::RangerTrooper);
    assert!(upgraded_stats.attack_damage > starting_stats.attack_damage);
}

#[test]
fn advanced_units_require_researched_tech() {
    let mut state = RtsGameState::new_two_player_test_match();
    let foundry_id =
        state.add_completed_building(PLAYER_TWO, BuildingKind::MechFoundry, vec2(920.0, 300.0));
    state.players[PLAYER_TWO].resources.matter = 1_000;

    let blocked = state.train_unit(foundry_id, UnitKind::AegisWalker);
    assert_eq!(Err(RtsError::MissingTech), blocked);

    let lab_id =
        state.add_completed_building(PLAYER_TWO, BuildingKind::TacticalLab, vec2(900.0, 360.0));
    state.research_tech(lab_id, TechKind::AegisFrame).unwrap();
    run_for(&mut state, TechKind::AegisFrame.research_time() + TEST_DT);

    state.train_unit(foundry_id, UnitKind::AegisWalker).unwrap();
}

#[test]
fn aetherborn_advanced_buildings_unlock_new_units_and_tech() {
    let mut state = RtsGameState::new_two_player_test_match();
    let sanctum_id = state.add_completed_building(
        PLAYER_ONE,
        BuildingKind::StarfallSanctum,
        vec2(280.0, 300.0),
    );
    let elder_id =
        state.add_completed_building(PLAYER_ONE, BuildingKind::ElderSanctum, vec2(300.0, 360.0));
    state.players[PLAYER_ONE].resources.matter = 2_000;

    state
        .research_tech(elder_id, TechKind::RootguardPact)
        .unwrap();
    run_for(
        &mut state,
        TechKind::RootguardPact.research_time() + TEST_DT,
    );
    state
        .research_tech(elder_id, TechKind::AstralChanneling)
        .unwrap();
    run_for(
        &mut state,
        TechKind::AstralChanneling.research_time() + TEST_DT,
    );

    state
        .train_unit(sanctum_id, UnitKind::GroveSentinel)
        .unwrap();
    state.train_unit(sanctum_id, UnitKind::WizardAdept).unwrap();
    run_for(
        &mut state,
        UnitKind::GroveSentinel.production_time() + UnitKind::WizardAdept.production_time(),
    );

    assert_eq!(1, unit_count(&state, PLAYER_ONE, UnitKind::GroveSentinel));
    assert_eq!(1, unit_count(&state, PLAYER_ONE, UnitKind::WizardAdept));
}
