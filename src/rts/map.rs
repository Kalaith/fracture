use super::{BuildingKind, PlayerState, RaceId, ResourceNodeKind, RtsGameState, UnitKind};
use macroquad::prelude::{vec2, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtsMapDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub dimensions: RtsMapDimensions,
    pub camera_bounds: RtsMapArea,
    pub players: Vec<RtsMapPlayerStart>,
    #[serde(default)]
    pub matter_nodes: Vec<RtsMapMatterNode>,
    #[serde(default)]
    pub ley_nodes: Vec<RtsMapLeyNode>,
    #[serde(default)]
    pub ley_segments: Vec<RtsMapLeySegment>,
    #[serde(default)]
    pub buildable_areas: Vec<RtsMapArea>,
    #[serde(default)]
    pub path_blockers: Vec<RtsMapBlocker>,
    #[serde(default)]
    pub expansion_markers: Vec<RtsMapExpansionMarker>,
}

impl RtsMapDefinition {
    pub fn from_json_str(content: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(content)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RtsMapDimensions {
    pub width: f32,
    pub height: f32,
}

impl RtsMapDimensions {
    fn as_vec2(self) -> Vec2 {
        vec2(self.width, self.height)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RtsMapPosition {
    pub x: f32,
    pub y: f32,
}

impl RtsMapPosition {
    fn as_vec2(self) -> Vec2 {
        vec2(self.x, self.y)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtsMapPlayerStart {
    pub player_id: usize,
    pub race: RaceId,
    pub main_base: RtsMapBuildingPlacement,
    #[serde(default)]
    pub starting_buildings: Vec<RtsMapBuildingPlacement>,
    #[serde(default)]
    pub starting_units: Vec<RtsMapUnitPlacement>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RtsMapBuildingPlacement {
    pub kind: BuildingKind,
    pub position: RtsMapPosition,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RtsMapUnitPlacement {
    pub kind: UnitKind,
    pub position: RtsMapPosition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtsMapMatterNode {
    pub id: String,
    pub position: RtsMapPosition,
    pub amount: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtsMapLeyNode {
    pub id: String,
    pub position: RtsMapPosition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtsMapLeySegment {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RtsMapArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtsMapBlocker {
    pub id: String,
    pub position: RtsMapPosition,
    pub radius: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtsMapExpansionMarker {
    pub id: String,
    pub position: RtsMapPosition,
    pub recommended_for: Option<RaceId>,
}

impl RtsGameState {
    pub fn from_map_definition(map: &RtsMapDefinition) -> Result<Self, String> {
        let mut player_starts = map.players.clone();
        player_starts.sort_by_key(|player| player.player_id);

        if player_starts.is_empty() {
            return Err("RTS map must define at least one player start".to_string());
        }

        for (expected_id, player) in player_starts.iter().enumerate() {
            if player.player_id != expected_id {
                return Err(format!(
                    "RTS map player ids must be sequential from 0; expected {}, found {}",
                    expected_id, player.player_id
                ));
            }
        }

        let mut state = Self {
            map_id: Some(map.id.clone()),
            map_size: map.dimensions.as_vec2(),
            players: player_starts
                .iter()
                .map(|player| PlayerState::new(player.race))
                .collect(),
            units: Vec::new(),
            buildings: Vec::new(),
            resource_nodes: Vec::new(),
            winner: None,
            next_entity_id: 0,
            next_resource_node_id: 0,
        };

        for node in &map.matter_nodes {
            state.add_matter_node(node.position.as_vec2(), node.amount);
        }

        for node in &map.ley_nodes {
            state.add_ley_node(node.position.as_vec2());
        }

        for player in &player_starts {
            validate_starting_building(player.player_id, player.race, player.main_base)?;
            if !player.main_base.kind.is_main_base() {
                return Err(format!(
                    "Player {} main base must use a main-base building kind",
                    player.player_id
                ));
            }

            state.add_completed_building(
                player.player_id,
                player.main_base.kind,
                player.main_base.position.as_vec2(),
            );

            for building in &player.starting_buildings {
                validate_starting_building(player.player_id, player.race, *building)?;
                state.add_completed_building(
                    player.player_id,
                    building.kind,
                    building.position.as_vec2(),
                );
            }

            for unit in &player.starting_units {
                if unit.kind.race() != player.race {
                    return Err(format!(
                        "Player {} has {:?} starting unit for wrong race",
                        player.player_id, unit.kind
                    ));
                }

                state.add_unit(player.player_id, unit.kind, unit.position.as_vec2());
            }
        }

        let has_matter = state
            .resource_nodes
            .iter()
            .any(|node| node.kind == ResourceNodeKind::Matter);
        let has_ley = state
            .resource_nodes
            .iter()
            .any(|node| node.kind == ResourceNodeKind::Ley);

        if !has_matter || !has_ley {
            return Err(
                "RTS map must contain at least one matter node and one ley node".to_string(),
            );
        }

        Ok(state)
    }
}

fn validate_starting_building(
    player_id: usize,
    race: RaceId,
    building: RtsMapBuildingPlacement,
) -> Result<(), String> {
    if building.kind.race() != race {
        return Err(format!(
            "Player {} has {:?} starting building for wrong race",
            player_id, building.kind
        ));
    }

    Ok(())
}
