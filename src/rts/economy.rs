use super::{
    BuildingKind, ResourceNodeId, ResourceNodeKind, RtsError, RtsGameState, UnitCommand,
    AETHERBORN_LEY_FLOW_PER_SHRINE, AETHERBORN_RITUAL_LINK_RANGE, AETHERBORN_SHRINE_RATE,
    TERRAN_BATTERY_ROUTE_RANGE, TERRAN_EXTRACTOR_RATE, WORKER_GATHER_RATE,
};
use std::collections::HashSet;

impl RtsGameState {
    pub(super) fn update_gathering(&mut self, dt: f32) {
        for unit_index in 0..self.units.len() {
            let UnitCommand::GatherMatter(node_id) = self.units[unit_index].command else {
                continue;
            };

            let Some(node_index) = self
                .resource_nodes
                .iter()
                .position(|node| node.id == node_id && node.kind == ResourceNodeKind::Matter)
            else {
                self.units[unit_index].command = UnitCommand::Idle;
                continue;
            };

            if self.resource_nodes[node_index].remaining == 0 {
                self.units[unit_index].command = UnitCommand::Idle;
                continue;
            }

            self.units[unit_index].gather_buffer += WORKER_GATHER_RATE * dt;
            let gathered = self.units[unit_index].gather_buffer.floor() as u32;
            if gathered == 0 {
                continue;
            }

            let owner = self.units[unit_index].owner;
            let gathered = gathered.min(self.resource_nodes[node_index].remaining);
            self.units[unit_index].gather_buffer -= gathered as f32;
            self.resource_nodes[node_index].remaining -= gathered;
            self.players[owner].resources.matter += gathered;
        }
    }

    pub(super) fn update_construction(&mut self, dt: f32) {
        let mut completed_buildings = Vec::new();

        for building in &mut self.buildings {
            if building.completed {
                continue;
            }

            let has_builder = self.units.iter().any(|unit| {
                unit.owner == building.owner && unit.command == UnitCommand::Build(building.id)
            });

            if !has_builder {
                continue;
            }

            building.build_time_remaining = (building.build_time_remaining - dt).max(0.0);
            if building.build_time_remaining == 0.0 {
                building.completed = true;
                completed_buildings.push((building.owner, building.kind, building.id));
            }
        }

        for (owner, kind, building_id) in completed_buildings {
            self.players[owner].supply_cap += kind.supply_provided();
            for unit in self.units.iter_mut().filter(|unit| {
                unit.owner == owner && unit.command == UnitCommand::Build(building_id)
            }) {
                unit.command = UnitCommand::Idle;
            }
        }
    }

    pub(super) fn update_aether(&mut self, dt: f32) {
        let mut active_aether_buildings = Vec::new();
        let mut flow_by_player = vec![0; self.players.len()];

        for building_index in 0..self.buildings.len() {
            let building = &self.buildings[building_index];
            if !building.completed || building.claimed_node.is_none() {
                continue;
            }

            match building.kind {
                BuildingKind::LeyShrine if self.aetherborn_shrine_connected(building_index) => {
                    active_aether_buildings.push((building_index, AETHERBORN_SHRINE_RATE));
                    flow_by_player[building.owner] += AETHERBORN_LEY_FLOW_PER_SHRINE;
                }
                BuildingKind::AetherExtractorRig
                    if self.terran_extractor_has_route(building_index) =>
                {
                    active_aether_buildings.push((building_index, TERRAN_EXTRACTOR_RATE));
                }
                _ => {}
            }
        }

        for (player_id, player) in self.players.iter_mut().enumerate() {
            player.ley_flow_capacity = flow_by_player[player_id];
        }

        for (building_index, rate) in active_aether_buildings {
            let building = &mut self.buildings[building_index];
            building.aether_buffer += rate * dt;
            let produced = building.aether_buffer.floor() as u32;
            if produced == 0 {
                continue;
            }

            building.aether_buffer -= produced as f32;
            self.players[building.owner].resources.aether += produced;
        }
    }

    pub(super) fn validate_ley_claim(
        &self,
        building_kind: BuildingKind,
        node_id: ResourceNodeId,
    ) -> Result<(), RtsError> {
        if !building_kind.can_claim_ley_node() {
            return Err(RtsError::InvalidAetherClaim);
        }

        let node = self
            .resource_node(node_id)
            .ok_or(RtsError::InvalidResourceNode)?;
        if node.kind != ResourceNodeKind::Ley {
            return Err(RtsError::InvalidResourceNode);
        }

        if self
            .buildings
            .iter()
            .any(|building| building.claimed_node == Some(node_id))
        {
            return Err(RtsError::ResourceNodeAlreadyClaimed);
        }

        Ok(())
    }

    fn aetherborn_shrine_connected(&self, shrine_index: usize) -> bool {
        let shrine = &self.buildings[shrine_index];
        let mut frontier: Vec<super::EntityId> = self
            .buildings
            .iter()
            .filter(|building| {
                building.owner == shrine.owner
                    && building.completed
                    && building.kind == BuildingKind::HeartwoodNexus
            })
            .map(|building| building.id)
            .collect();
        let mut visited = HashSet::new();

        while let Some(current_id) = frontier.pop() {
            if !visited.insert(current_id) {
                continue;
            }

            if current_id == shrine.id {
                return true;
            }

            let Some(current) = self.building(current_id) else {
                continue;
            };

            for neighbor in self.buildings.iter().filter(|building| {
                building.owner == shrine.owner
                    && building.completed
                    && building.kind.is_aetherborn_ritual_network_member()
                    && !visited.contains(&building.id)
                    && (building.position - current.position).length()
                        <= AETHERBORN_RITUAL_LINK_RANGE
            }) {
                frontier.push(neighbor.id);
            }
        }

        false
    }

    fn terran_extractor_has_route(&self, extractor_index: usize) -> bool {
        let extractor = &self.buildings[extractor_index];
        self.buildings.iter().any(|building| {
            building.owner == extractor.owner
                && building.completed
                && building.kind.is_terran_battery_destination()
                && (building.position - extractor.position).length() <= TERRAN_BATTERY_ROUTE_RANGE
        })
    }
}
