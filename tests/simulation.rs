use fracture_command::config::Config;
use fracture_command::game::GameState;
use fracture_command::types::{FactionId, Sector, Squad, SquadOrder, Unit, UnitType};
use macroquad::prelude::{vec2, Vec2};

const TEST_DT: f32 = 0.2;

fn run_for(game: &mut GameState, seconds: f32) {
    let steps = (seconds / TEST_DT).ceil() as usize;

    for _ in 0..steps {
        game.update(TEST_DT);
    }
}

fn sector_progress_for(faction: FactionId) -> f32 {
    match faction {
        FactionId::Faction1 => -1.0,
        FactionId::Faction2 => 1.0,
        FactionId::Faction3 => 0.0,
    }
}

fn clear_sector_control(sectors: &mut [Sector]) {
    for sector in sectors {
        sector.control = None;
        sector.control_progress = 0.0;
    }
}

fn control_sectors(game: &mut GameState, faction: FactionId, sector_ids: &[usize]) {
    clear_sector_control(&mut game.sectors);

    for &sector_id in sector_ids {
        let sector = &mut game.sectors[sector_id];
        sector.control = Some(faction);
        sector.control_progress = sector_progress_for(faction);
    }
}

fn spawn_test_squad(game: &mut GameState, owner: FactionId, position: Vec2, count: u32) -> u32 {
    let squad_id = game.simulation.next_squad_id();
    let mut squad = Squad::new(squad_id, owner, SquadOrder::Hold, position);

    for index in 0..count {
        let unit_id = game.simulation.next_unit_id();
        let offset = vec2(index as f32 * 4.0, 0.0);
        squad.add_unit(Unit::new(
            unit_id,
            UnitType::InfantryLight,
            position + offset,
            owner,
        ));
    }

    game.squads.push(squad);
    squad_id
}

#[test]
fn sector_flips_when_player_has_clear_majority() {
    let mut game = GameState::new(FactionId::Faction1);
    assert_eq!(Config::NUM_SECTORS as usize, game.sectors.len());

    let sector_id = 0;
    let sector_position = game.sectors[sector_id].position;
    spawn_test_squad(&mut game, FactionId::Faction1, sector_position, 3);

    run_for(&mut game, 7.0);

    assert_eq!(Some(FactionId::Faction1), game.sectors[sector_id].control);
}

#[test]
fn player_wins_after_holding_sector_majority() {
    let mut game = GameState::new(FactionId::Faction1);
    control_sectors(&mut game, FactionId::Faction1, &[0, 1, 2]);

    run_for(&mut game, Config::VICTORY_TIME_REQUIRED + TEST_DT);

    assert!(game.game_over);
    assert_eq!(Some(FactionId::Faction1), game.winner);
}

#[test]
fn ai_wins_after_holding_sector_majority() {
    let mut game = GameState::new(FactionId::Faction1);
    control_sectors(&mut game, FactionId::Faction2, &[0, 3, 4]);

    run_for(&mut game, Config::VICTORY_TIME_REQUIRED + TEST_DT);

    assert!(game.game_over);
    assert_eq!(Some(FactionId::Faction2), game.winner);
}
