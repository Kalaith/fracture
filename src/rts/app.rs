mod render;

use super::{
    command_catalog_for, BasicSkirmishAi, BuildingKind, EntityId, RaceId, ResourceNodeId,
    ResourceNodeKind, RtsGameState, RtsMapDefinition, TechKind, UnitCommand, UnitKind, PLAYER_ONE,
    PLAYER_TWO,
};
use macroquad::prelude::*;
use macroquad_toolkit::camera::Camera2D as ToolkitCamera;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

const CRASH_BASIN_JSON: &str = macroquad_toolkit::include_json_str!("../../assets/data/rts_maps/crash_basin_skirmish.json");
const CAMERA_PAN_SPEED: f32 = 520.0;
const CAMERA_ZOOM_SPEED: f32 = 0.12;
const CAMERA_MIN_ZOOM: f32 = 0.55;
const CAMERA_MAX_ZOOM: f32 = 2.2;
const SELECT_UNIT_RADIUS: f32 = 18.0;
const SELECT_BUILDING_RADIUS: f32 = 42.0;
const SELECT_RESOURCE_RADIUS: f32 = 34.0;
const AETHER_NODE_BUILD_RADIUS: f32 = 70.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BuildIntent {
    Standard(BuildingKind),
    LeyClaim,
}

pub struct RtsApp {
    pub state: RtsGameState,
    pub map: RtsMapDefinition,
    pub local_player: usize,
    ai: BasicSkirmishAi,
    selected_units: Vec<EntityId>,
    selected_building: Option<EntityId>,
    pending_build: Option<BuildIntent>,
    message: String,
    message_timer: f32,
}

impl RtsApp {
    pub fn new_crash_basin() -> Result<Self, String> {
        Self::new_crash_basin_for(PLAYER_ONE)
    }

    pub fn new_crash_basin_for(local_player: usize) -> Result<Self, String> {
        let map = RtsMapDefinition::from_json_str(CRASH_BASIN_JSON)
            .map_err(|err| format!("Failed to parse Crash Basin RTS map: {}", err))?;
        let state = RtsGameState::from_map_definition(&map)?;
        let ai_player = opposing_player(local_player);

        Ok(Self {
            state,
            map,
            local_player,
            ai: BasicSkirmishAi::new(ai_player),
            selected_units: Vec::new(),
            selected_building: None,
            pending_build: None,
            message: "Crash Basin loaded".to_string(),
            message_timer: 3.0,
        })
    }

    pub fn camera_center(&self) -> Vec2 {
        self.state
            .buildings
            .iter()
            .find(|building| building.owner == self.local_player && building.kind.is_main_base())
            .map(|building| building.position)
            .unwrap_or(self.state.map_size * 0.5)
    }

    pub fn update(&mut self, camera: &mut ToolkitCamera, dt: f32) {
        self.message_timer = (self.message_timer - dt).max(0.0);
        self.refresh_selection();
        self.handle_camera_input(camera, dt);
        self.handle_mouse_input(camera);
        self.handle_hotkeys(camera);

        self.ai.update(&mut self.state, dt);
        self.state.update(dt);
    }

    pub fn render(&self, camera: &ToolkitCamera) {
        clear_background(Color::from_rgba(17, 20, 23, 255));
        self.render_world(camera);
        self.render_ui();
    }

    fn refresh_selection(&mut self) {
        self.selected_units
            .retain(|unit_id| self.state.units.iter().any(|unit| unit.id == *unit_id));

        if let Some(building_id) = self.selected_building {
            if self
                .state
                .buildings
                .iter()
                .all(|building| building.id != building_id)
            {
                self.selected_building = None;
            }
        }
    }

    fn handle_camera_input(&self, camera: &mut ToolkitCamera, dt: f32) {
        let mut pan_direction = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            pan_direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            pan_direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            pan_direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            pan_direction.x += 1.0;
        }

        if pan_direction != Vec2::ZERO {
            camera.target += pan_direction.normalize() * CAMERA_PAN_SPEED * dt / camera.zoom;
        }

        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            let zoom_factor = 1.0 + wheel_y.signum() * CAMERA_ZOOM_SPEED;
            camera.zoom = (camera.zoom * zoom_factor).clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);
        }

        if let Some(bounds) = self.state.camera_bounds {
            camera.target = bounds.clamp(camera.target);
        }

        camera.update(dt, false);
    }

    fn handle_mouse_input(&mut self, camera: &ToolkitCamera) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let world_pos = mouse_world(camera);
            if self.place_pending_build(world_pos) {
                return;
            }

            self.select_at(world_pos);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            if self.pending_build.take().is_some() {
                self.set_message("Build placement canceled");
                return;
            }

            let world_pos = mouse_world(camera);
            self.issue_right_click_order(world_pos);
        }
    }

    fn select_at(&mut self, world_pos: Vec2) {
        if let Some(unit_id) = self.find_local_unit_at(world_pos) {
            self.selected_units.clear();
            self.selected_units.push(unit_id);
            self.selected_building = None;
            self.set_message("Unit selected");
            return;
        }

        if let Some(building_id) = self.find_local_building_at(world_pos) {
            self.selected_units.clear();
            self.selected_building = Some(building_id);
            self.set_message("Building selected");
            return;
        }

        self.selected_units.clear();
        self.selected_building = None;
    }

    fn issue_right_click_order(&mut self, world_pos: Vec2) {
        if self.selected_units.is_empty() {
            return;
        }

        let selected_units = self.selected_units.clone();
        if let Some(target_id) = self.find_enemy_unit_at(world_pos) {
            for unit_id in selected_units {
                let _ = self.state.command_attack_unit(unit_id, target_id);
            }
            self.set_message("Attack order issued");
            return;
        }

        if let Some(target_id) = self.find_enemy_building_at(world_pos) {
            for unit_id in selected_units {
                let _ = self.state.command_attack_building(unit_id, target_id);
            }
            self.set_message("Attack building order issued");
            return;
        }

        if let Some(node_id) = self.find_matter_node_at(world_pos) {
            let workers: Vec<EntityId> = self
                .selected_units
                .iter()
                .copied()
                .filter(|unit_id| {
                    self.state
                        .units
                        .iter()
                        .find(|unit| unit.id == *unit_id)
                        .is_some_and(|unit| unit.kind.is_worker())
                })
                .collect();

            if !workers.is_empty() {
                for unit_id in workers {
                    let _ = self.state.command_gather_matter(unit_id, node_id);
                }
                self.set_message("Gather order issued");
                return;
            }
        }

        for unit_id in selected_units {
            let _ = self.state.command_attack_move_unit(unit_id, world_pos);
        }
        self.set_message("Move order issued");
    }

    fn handle_hotkeys(&mut self, camera: &ToolkitCamera) {
        let _ = camera;
        let catalog = command_catalog_for(self.local_race());

        if is_key_pressed(KeyCode::Z) {
            self.start_build_placement(BuildIntent::Standard(catalog.supply_building));
        }
        if is_key_pressed(KeyCode::X) {
            self.start_build_placement(BuildIntent::Standard(catalog.production_building));
        }
        if is_key_pressed(KeyCode::C) {
            self.start_build_placement(BuildIntent::LeyClaim);
        }
        if is_key_pressed(KeyCode::V) {
            self.start_build_placement(BuildIntent::Standard(catalog.aether_link_building));
        }
        if is_key_pressed(KeyCode::B) {
            self.start_build_placement(BuildIntent::Standard(catalog.advanced_production_building));
        }
        if is_key_pressed(KeyCode::N) {
            self.start_build_placement(BuildIntent::Standard(catalog.research_building));
        }

        if is_key_pressed(KeyCode::Q) {
            self.try_train(catalog.worker_unit);
        }
        if is_key_pressed(KeyCode::T) {
            self.try_train(catalog.combat_unit);
        }
        if is_key_pressed(KeyCode::Y) {
            self.try_train(catalog.heavy_unit);
        }
        if is_key_pressed(KeyCode::U) {
            if let Some(unit_kind) = catalog.specialist_unit {
                self.try_train(unit_kind);
            } else {
                self.set_message("This race has no specialist unit yet");
            }
        }
        if is_key_pressed(KeyCode::R) {
            self.try_research(catalog.basic_tech);
        }
        if is_key_pressed(KeyCode::F) {
            self.try_research(catalog.heavy_tech);
        }
        if is_key_pressed(KeyCode::G) {
            if let Some(tech_kind) = catalog.specialist_tech {
                self.try_research(tech_kind);
            } else {
                self.set_message("This race has no specialist tech yet");
            }
        }
        if is_key_pressed(KeyCode::F5) {
            self.restart_as(opposing_player(self.local_player));
        }
    }

    fn start_build_placement(&mut self, intent: BuildIntent) {
        if self.selected_worker().is_none() {
            self.set_message("Select a worker, then choose a build command");
            return;
        }

        self.pending_build = Some(intent);
        match intent {
            BuildIntent::Standard(kind) => self.set_message(&format!(
                "Placing {:?}: left-click map to build, right-click to cancel",
                kind
            )),
            BuildIntent::LeyClaim => {
                self.set_message("Placing ley claim: left-click a ley node, right-click to cancel")
            }
        }
    }

    fn place_pending_build(&mut self, position: Vec2) -> bool {
        let Some(intent) = self.pending_build else {
            return false;
        };

        match intent {
            BuildIntent::Standard(kind) => {
                if self.try_build(kind, position) {
                    self.pending_build = None;
                }
            }
            BuildIntent::LeyClaim => {
                if self.try_build_ley_claim(position) {
                    self.pending_build = None;
                }
            }
        }

        true
    }

    fn restart_as(&mut self, local_player: usize) {
        match Self::new_crash_basin_for(local_player) {
            Ok(app) => *self = app,
            Err(err) => self.set_message(&format!("Restart failed: {}", err)),
        }
    }

    fn try_build(&mut self, building_kind: BuildingKind, position: Vec2) -> bool {
        let Some(worker_id) = self.selected_worker() else {
            self.set_message("Select a worker first");
            return false;
        };

        match self
            .state
            .start_construction(worker_id, building_kind, position)
        {
            Ok(_) => {
                self.set_message("Construction started");
                true
            }
            Err(err) => {
                self.set_message(&format!("Build failed: {:?}", err));
                false
            }
        }
    }

    fn try_build_ley_claim(&mut self, position: Vec2) -> bool {
        let Some(worker_id) = self.selected_worker() else {
            self.set_message("Select a worker first");
            return false;
        };
        let Some(node_id) = self.find_ley_node_near(position) else {
            self.set_message("Point at a ley node first");
            return false;
        };

        let building_kind = command_catalog_for(self.local_race()).ley_claim_building;
        match self
            .state
            .start_construction_on_resource_node(worker_id, building_kind, node_id)
        {
            Ok(_) => {
                self.set_message("Ley claim started");
                true
            }
            Err(err) => {
                self.set_message(&format!("Ley claim failed: {:?}", err));
                false
            }
        }
    }

    fn try_train(&mut self, unit_kind: UnitKind) {
        let Some(building_id) = self.selected_building else {
            self.set_message("Select a production building first");
            return;
        };

        match self.state.train_unit(building_id, unit_kind) {
            Ok(_) => self.set_message("Training queued"),
            Err(err) => self.set_message(&format!("Training failed: {:?}", err)),
        }
    }

    fn try_research(&mut self, tech_kind: TechKind) {
        let Some(building_id) = self.selected_building else {
            self.set_message("Select a tech building first");
            return;
        };

        match self.state.research_tech(building_id, tech_kind) {
            Ok(_) => self.set_message("Research queued"),
            Err(err) => self.set_message(&format!("Research failed: {:?}", err)),
        }
    }

    fn selected_worker(&self) -> Option<EntityId> {
        self.selected_units.iter().copied().find(|unit_id| {
            self.state
                .units
                .iter()
                .find(|unit| unit.id == *unit_id)
                .is_some_and(|unit| unit.owner == self.local_player && unit.kind.is_worker())
        })
    }

    fn local_race(&self) -> RaceId {
        self.state.players[self.local_player].race
    }

    fn set_message(&mut self, message: &str) {
        self.message.clear();
        self.message.push_str(message);
        self.message_timer = 3.0;
    }
    fn find_local_unit_at(&self, position: Vec2) -> Option<EntityId> {
        self.state
            .units
            .iter()
            .find(|unit| {
                unit.owner == self.local_player
                    && unit.position.distance(position) <= SELECT_UNIT_RADIUS
            })
            .map(|unit| unit.id)
    }

    fn find_enemy_unit_at(&self, position: Vec2) -> Option<EntityId> {
        self.state
            .units
            .iter()
            .find(|unit| {
                unit.owner != self.local_player
                    && unit.position.distance(position) <= SELECT_UNIT_RADIUS
            })
            .map(|unit| unit.id)
    }

    fn find_local_building_at(&self, position: Vec2) -> Option<EntityId> {
        self.state
            .buildings
            .iter()
            .find(|building| {
                building.owner == self.local_player
                    && building.position.distance(position) <= SELECT_BUILDING_RADIUS
            })
            .map(|building| building.id)
    }

    fn find_enemy_building_at(&self, position: Vec2) -> Option<EntityId> {
        self.state
            .buildings
            .iter()
            .find(|building| {
                building.owner != self.local_player
                    && building.position.distance(position) <= SELECT_BUILDING_RADIUS
            })
            .map(|building| building.id)
    }

    fn find_matter_node_at(&self, position: Vec2) -> Option<ResourceNodeId> {
        self.state
            .resource_nodes
            .iter()
            .find(|node| {
                node.kind == ResourceNodeKind::Matter
                    && node.position.distance(position) <= SELECT_RESOURCE_RADIUS
            })
            .map(|node| node.id)
    }

    fn find_ley_node_near(&self, position: Vec2) -> Option<ResourceNodeId> {
        self.state
            .resource_nodes
            .iter()
            .find(|node| {
                node.kind == ResourceNodeKind::Ley
                    && node.position.distance(position) <= AETHER_NODE_BUILD_RADIUS
            })
            .map(|node| node.id)
    }
}

fn mouse_world(camera: &ToolkitCamera) -> Vec2 {
    let mouse = mouse_position();
    camera.screen_to_world(vec2(mouse.0, mouse.1))
}

fn owner_color(owner: usize) -> Color {
    match owner {
        PLAYER_ONE => Color::from_rgba(92, 178, 125, 255),
        PLAYER_TWO => Color::from_rgba(90, 150, 220, 255),
        _ => GRAY,
    }
}

fn race_label(race: RaceId) -> &'static str {
    match race {
        RaceId::Aetherborn => "Aetherborn Concord",
        RaceId::Terran => "Terran Directorate",
    }
}

fn building_label(kind: BuildingKind) -> &'static str {
    match kind {
        BuildingKind::HeartwoodNexus => "Nexus",
        BuildingKind::CommandArk => "Ark",
        BuildingKind::Moonwell => "Moonwell",
        BuildingKind::SupplyPylon => "Pylon",
        BuildingKind::GroveCircle => "Grove",
        BuildingKind::FabricatorBay => "Fab",
        BuildingKind::StarfallSanctum => "Stars",
        BuildingKind::MechFoundry => "Foundry",
        BuildingKind::ElderSanctum => "Elder",
        BuildingKind::TacticalLab => "Lab",
        BuildingKind::LeyShrine => "Shrine",
        BuildingKind::RitualNode => "Node",
        BuildingKind::AetherExtractorRig => "Rig",
        BuildingKind::BatteryDepot => "Depot",
    }
}

fn building_size(kind: BuildingKind) -> Vec2 {
    match kind {
        BuildingKind::HeartwoodNexus | BuildingKind::CommandArk => vec2(70.0, 58.0),
        BuildingKind::GroveCircle | BuildingKind::FabricatorBay => vec2(58.0, 46.0),
        BuildingKind::StarfallSanctum | BuildingKind::MechFoundry => vec2(64.0, 52.0),
        BuildingKind::ElderSanctum | BuildingKind::TacticalLab => vec2(52.0, 44.0),
        BuildingKind::Moonwell | BuildingKind::SupplyPylon => vec2(44.0, 38.0),
        BuildingKind::LeyShrine | BuildingKind::AetherExtractorRig => vec2(42.0, 42.0),
        BuildingKind::RitualNode | BuildingKind::BatteryDepot => vec2(38.0, 34.0),
    }
}

fn unit_radius(kind: UnitKind) -> f32 {
    match kind {
        UnitKind::SpriteGatherer | UnitKind::FieldEngineer => 9.0,
        UnitKind::ElvenWarden | UnitKind::RangerTrooper => 12.0,
        UnitKind::GroveSentinel | UnitKind::WizardAdept => 14.0,
        UnitKind::AegisWalker => 17.0,
    }
}

fn draw_health_bar(top_left: Vec2, width: f32, health: f32, max_health: f32) {
    let pct = if max_health <= 0.0 {
        0.0
    } else {
        (health / max_health).clamp(0.0, 1.0)
    };
    draw_rectangle(
        top_left.x,
        top_left.y,
        width,
        5.0,
        Color::from_rgba(35, 37, 39, 255),
    );
    draw_rectangle(
        top_left.x,
        top_left.y,
        width * pct,
        5.0,
        Color::from_rgba(75, 220, 105, 255),
    );
}

fn opposing_player(player_id: usize) -> usize {
    if player_id == PLAYER_ONE {
        PLAYER_TWO
    } else {
        PLAYER_ONE
    }
}
