use fracture_command::rts::{
    BuildingKind, RaceId, ResourceNodeKind, RtsError, RtsGameState, RtsMapDefinition, UnitKind,
    PLAYER_ONE, PLAYER_TWO,
};
use macroquad::prelude::vec2;

const CRASH_BASIN_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/rts_maps/crash_basin_skirmish.json");

fn unit_count(state: &RtsGameState, player_id: usize, unit_kind: UnitKind) -> usize {
    state
        .units
        .iter()
        .filter(|unit| unit.owner == player_id && unit.kind == unit_kind)
        .count()
}

#[test]
fn crash_basin_map_loads_into_rts_state() {
    let map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON).unwrap();
    let state = RtsGameState::from_map_definition(&map).unwrap();

    assert_eq!(state.map_id.as_deref(), Some("crash_basin_skirmish"));
    assert_eq!(state.map_size.x, 2400.0);
    assert_eq!(state.map_size.y, 1600.0);
    assert_eq!(state.players[PLAYER_ONE].race, RaceId::Aetherborn);
    assert_eq!(state.players[PLAYER_TWO].race, RaceId::Terran);
    assert_eq!(unit_count(&state, PLAYER_ONE, UnitKind::SpriteGatherer), 4);
    assert_eq!(unit_count(&state, PLAYER_TWO, UnitKind::FieldEngineer), 4);
    assert!(state
        .completed_buildings_for(PLAYER_ONE, BuildingKind::HeartwoodNexus)
        .next()
        .is_some());
    assert!(state
        .completed_buildings_for(PLAYER_TWO, BuildingKind::CommandArk)
        .next()
        .is_some());
}

#[test]
fn crash_basin_contains_required_resource_layout() {
    let map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON).unwrap();
    let state = RtsGameState::from_map_definition(&map).unwrap();

    let matter_nodes = state
        .resource_nodes
        .iter()
        .filter(|node| node.kind == ResourceNodeKind::Matter)
        .count();
    let ley_nodes = state
        .resource_nodes
        .iter()
        .filter(|node| node.kind == ResourceNodeKind::Ley)
        .count();

    assert_eq!(matter_nodes, 7);
    assert_eq!(ley_nodes, 7);
    assert_eq!(map.ley_segments.len(), 6);
    assert_eq!(map.buildable_areas.len(), 5);
    assert_eq!(map.path_blockers.len(), 5);
    assert_eq!(map.expansion_markers.len(), 5);
    assert!(state.camera_bounds.is_some());
    assert_eq!(state.buildable_areas.len(), map.buildable_areas.len());
    assert_eq!(state.path_blockers.len(), map.path_blockers.len());
    assert_eq!(state.expansion_markers.len(), map.expansion_markers.len());
}

#[test]
fn rts_map_loader_rejects_wrong_race_starting_unit() {
    let mut map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON).unwrap();
    map.players[PLAYER_ONE].starting_units[0].kind = UnitKind::FieldEngineer;

    let result = RtsGameState::from_map_definition(&map);

    assert!(result.is_err());
}

#[test]
fn map_metadata_controls_building_and_move_targets() {
    let map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON).unwrap();
    let state = RtsGameState::from_map_definition(&map).unwrap();

    assert!(state.can_place_standard_building(vec2(720.0, 690.0)));
    assert!(!state.can_place_standard_building(vec2(1200.0, 800.0)));
    assert!(!state.can_place_standard_building(vec2(-25.0, 800.0)));

    let central_blocker = map
        .path_blockers
        .iter()
        .find(|blocker| blocker.id == "central_crater")
        .unwrap();
    let blocked = vec2(central_blocker.position.x, central_blocker.position.y);
    let passable = state.nearest_passable_position(blocked);

    assert!(passable.distance(blocked) >= central_blocker.radius);
    assert_eq!(
        vec2(0.0, 0.0),
        state.nearest_passable_position(vec2(-40.0, -50.0))
    );
}

#[test]
fn construction_rejects_non_buildable_map_locations() {
    let map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON).unwrap();
    let mut state = RtsGameState::from_map_definition(&map).unwrap();
    let worker_id = state.first_worker_for(PLAYER_ONE).unwrap();

    let result = state.start_construction(worker_id, BuildingKind::Moonwell, vec2(1200.0, 800.0));

    assert_eq!(Err(RtsError::InvalidBuildLocation), result);
}
