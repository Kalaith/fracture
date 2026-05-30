use super::{ProductionJob, ResearchJob, RtsError, RtsGameState, TechKind, UnitKind, UnitStats};
use macroquad::prelude::vec2;

impl RtsGameState {
    pub fn can_train_unit(&self, player_id: usize, unit_kind: UnitKind) -> Result<(), RtsError> {
        self.ensure_player(player_id)?;

        let player = &self.players[player_id];
        if player.race != unit_kind.race() {
            return Err(RtsError::WrongRace);
        }

        if player.resources.matter < unit_kind.matter_cost() {
            return Err(RtsError::InsufficientMatter);
        }

        if self.available_supply_after_queued(player_id) < unit_kind.supply_cost() {
            return Err(RtsError::SupplyBlocked);
        }

        if unit_kind
            .required_tech()
            .is_some_and(|tech_kind| !player.has_tech(tech_kind))
        {
            return Err(RtsError::MissingTech);
        }

        Ok(())
    }

    pub fn train_unit(
        &mut self,
        producer_id: super::EntityId,
        unit_kind: UnitKind,
    ) -> Result<(), RtsError> {
        let (owner, building_kind) = {
            let building = self.building(producer_id).ok_or(RtsError::InvalidEntity)?;
            if !building.completed {
                return Err(RtsError::BuildingIncomplete);
            }
            (building.owner, building.kind)
        };

        if !building_kind.can_produce(unit_kind) {
            return Err(RtsError::UnsupportedProduction);
        }

        self.can_train_unit(owner, unit_kind)?;
        self.players[owner].resources.matter -= unit_kind.matter_cost();

        let building = self
            .building_mut(producer_id)
            .ok_or(RtsError::InvalidEntity)?;
        building.production_queue.push_back(ProductionJob {
            unit_kind,
            time_remaining: unit_kind.production_time(),
        });

        Ok(())
    }

    pub fn research_tech(
        &mut self,
        researcher_id: super::EntityId,
        tech_kind: TechKind,
    ) -> Result<(), RtsError> {
        let (owner, building_kind) = {
            let building = self
                .building(researcher_id)
                .ok_or(RtsError::InvalidEntity)?;
            if !building.completed {
                return Err(RtsError::BuildingIncomplete);
            }
            (building.owner, building.kind)
        };

        if self.players[owner].race != tech_kind.race() {
            return Err(RtsError::WrongRace);
        }

        if !building_kind.can_research(tech_kind) {
            return Err(RtsError::UnsupportedResearch);
        }

        if self.players[owner].has_tech(tech_kind) || self.is_tech_queued_for(owner, tech_kind) {
            return Err(RtsError::TechAlreadyResearched);
        }

        let cost = tech_kind.matter_cost();
        if self.players[owner].resources.matter < cost {
            return Err(RtsError::InsufficientMatter);
        }

        self.players[owner].resources.matter -= cost;

        let building = self
            .building_mut(researcher_id)
            .ok_or(RtsError::InvalidEntity)?;
        building.research_queue.push_back(ResearchJob {
            tech_kind,
            time_remaining: tech_kind.research_time(),
        });

        Ok(())
    }

    pub fn unit_stats_for_player(&self, player_id: usize, unit_kind: UnitKind) -> UnitStats {
        let mut stats = unit_kind.base_stats();

        if let Some(player) = self.players.get(player_id) {
            for tech_kind in &player.researched_techs {
                for bonus in tech_kind.stat_bonuses() {
                    if bonus.unit_kind != unit_kind {
                        continue;
                    }

                    stats.max_health += bonus.max_health;
                    stats.attack_damage += bonus.attack_damage;
                    stats.armor += bonus.armor;
                }
            }
        }

        stats
    }

    pub(super) fn update_production(&mut self, dt: f32) {
        let mut completed_units = Vec::new();

        for building in &mut self.buildings {
            if !building.completed {
                continue;
            }

            let Some(job) = building.production_queue.front_mut() else {
                continue;
            };

            job.time_remaining = (job.time_remaining - dt).max(0.0);
            if job.time_remaining == 0.0 {
                let job = building.production_queue.pop_front().unwrap();
                completed_units.push((building.owner, job.unit_kind, building.position));
            }
        }

        for (owner, unit_kind, position) in completed_units {
            self.add_unit(owner, unit_kind, position + vec2(32.0, 0.0));
        }
    }

    pub(super) fn update_research(&mut self, dt: f32) {
        let mut completed_research = Vec::new();

        for building in &mut self.buildings {
            if !building.completed {
                continue;
            }

            let Some(job) = building.research_queue.front_mut() else {
                continue;
            };

            job.time_remaining = (job.time_remaining - dt).max(0.0);
            if job.time_remaining == 0.0 {
                let job = building.research_queue.pop_front().unwrap();
                completed_research.push((building.owner, job.tech_kind));
            }
        }

        for (owner, tech_kind) in completed_research {
            if !self.players[owner].has_tech(tech_kind) {
                self.players[owner].researched_techs.push(tech_kind);
            }
        }
    }

    fn available_supply_after_queued(&self, player_id: usize) -> u32 {
        let queued_supply: u32 = self
            .buildings
            .iter()
            .filter(|building| building.owner == player_id)
            .flat_map(|building| &building.production_queue)
            .map(|job| job.unit_kind.supply_cost())
            .sum();

        self.players[player_id]
            .supply_cap
            .saturating_sub(self.players[player_id].supply_used + queued_supply)
    }

    fn is_tech_queued_for(&self, player_id: usize, tech_kind: TechKind) -> bool {
        self.buildings
            .iter()
            .filter(|building| building.owner == player_id)
            .flat_map(|building| &building.research_queue)
            .any(|job| job.tech_kind == tech_kind)
    }
}
