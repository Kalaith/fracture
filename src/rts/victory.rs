use super::{BuildingKind, RtsGameState, UnitKind};

impl RtsGameState {
    pub(super) fn cleanup_destroyed_entities(&mut self) {
        let destroyed_units: Vec<(usize, UnitKind)> = self
            .units
            .iter()
            .filter(|unit| unit.health <= 0.0)
            .map(|unit| (unit.owner, unit.kind))
            .collect();
        self.units.retain(|unit| unit.health > 0.0);

        for (owner, kind) in destroyed_units {
            self.players[owner].supply_used = self.players[owner]
                .supply_used
                .saturating_sub(kind.supply_cost());
        }

        let destroyed_buildings: Vec<(usize, BuildingKind)> = self
            .buildings
            .iter()
            .filter(|building| building.health <= 0.0)
            .map(|building| (building.owner, building.kind))
            .collect();
        self.buildings.retain(|building| building.health > 0.0);

        for (owner, kind) in destroyed_buildings {
            self.players[owner].supply_cap = self.players[owner]
                .supply_cap
                .saturating_sub(kind.supply_provided());
        }

        self.update_victory_state();
    }

    fn update_victory_state(&mut self) {
        if self.winner.is_some() {
            return;
        }

        let defeated_player = self.players.iter().enumerate().find_map(|(player_id, _)| {
            let has_main_base = self.buildings.iter().any(|building| {
                building.owner == player_id
                    && building.completed
                    && building.kind.is_main_base()
                    && building.health > 0.0
            });

            if has_main_base {
                None
            } else {
                Some(player_id)
            }
        });

        if let Some(defeated_player) = defeated_player {
            self.winner = self
                .players
                .iter()
                .enumerate()
                .find_map(|(player_id, _)| (player_id != defeated_player).then_some(player_id));
        }
    }
}
