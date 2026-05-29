use fracture_command::rts::{RaceId, RtsApp, PLAYER_ONE, PLAYER_TWO};

#[test]
fn rts_app_can_start_with_either_playable_race() {
    let aetherborn_app = RtsApp::new_crash_basin_for(PLAYER_ONE).unwrap();
    let terran_app = RtsApp::new_crash_basin_for(PLAYER_TWO).unwrap();

    assert_eq!(aetherborn_app.local_player, PLAYER_ONE);
    assert_eq!(
        aetherborn_app.state.players[PLAYER_ONE].race,
        RaceId::Aetherborn
    );
    assert_eq!(terran_app.local_player, PLAYER_TWO);
    assert_eq!(terran_app.state.players[PLAYER_TWO].race, RaceId::Terran);
}
