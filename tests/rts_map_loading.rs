use fracture_command::rts::{
    BuildingKind, RaceId, ResourceNodeKind, RtsGameState, RtsMapDefinition, UnitKind, PLAYER_ONE,
    PLAYER_TWO,
};

const CRASH_BASIN_JSON: &str = include_str!("../assets/data/rts_maps/crash_basin_skirmish.json");

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
    assert_eq!(state.map_size.x, 1280.0);
    assert_eq!(state.map_size.y, 720.0);
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

    assert_eq!(matter_nodes, 3);
    assert_eq!(ley_nodes, 3);
    assert_eq!(map.ley_segments.len(), 2);
    assert_eq!(map.buildable_areas.len(), 3);
    assert_eq!(map.path_blockers.len(), 2);
    assert_eq!(map.expansion_markers.len(), 3);
}

#[test]
fn rts_map_loader_rejects_wrong_race_starting_unit() {
    let mut map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON).unwrap();
    map.players[PLAYER_ONE].starting_units[0].kind = UnitKind::FieldEngineer;

    let result = RtsGameState::from_map_definition(&map);

    assert!(result.is_err());
}
