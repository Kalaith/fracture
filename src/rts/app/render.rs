use super::*;

impl RtsApp {
    pub(super) fn render_world(&self, camera: &ToolkitCamera) {
        camera.begin();

        draw_rectangle_lines(
            0.0,
            0.0,
            self.state.map_size.x,
            self.state.map_size.y,
            3.0,
            Color::from_rgba(77, 88, 96, 255),
        );

        self.render_map_features();
        self.render_buildings();
        self.render_units();
        self.render_build_preview(camera);

        set_default_camera();
    }

    pub(super) fn render_map_features(&self) {
        for segment in &self.map.ley_segments {
            let Some(from) = self
                .map
                .ley_nodes
                .iter()
                .find(|node| node.id == segment.from)
                .map(|node| node.position)
            else {
                continue;
            };
            let Some(to) = self
                .map
                .ley_nodes
                .iter()
                .find(|node| node.id == segment.to)
                .map(|node| node.position)
            else {
                continue;
            };

            draw_line(
                from.x,
                from.y,
                to.x,
                to.y,
                3.0,
                Color::from_rgba(78, 178, 210, 150),
            );
        }

        for node in &self.state.resource_nodes {
            match node.kind {
                ResourceNodeKind::Matter => {
                    draw_circle(
                        node.position.x,
                        node.position.y,
                        24.0,
                        Color::from_rgba(130, 148, 120, 255),
                    );
                    draw_circle_lines(node.position.x, node.position.y, 26.0, 2.0, DARKGREEN);
                }
                ResourceNodeKind::Ley => {
                    draw_circle(
                        node.position.x,
                        node.position.y,
                        20.0,
                        Color::from_rgba(54, 165, 217, 210),
                    );
                    draw_circle_lines(node.position.x, node.position.y, 28.0, 2.0, SKYBLUE);
                }
            }
        }
    }

    pub(super) fn render_buildings(&self) {
        for building in &self.state.buildings {
            let color = owner_color(building.owner);
            let size = building_size(building.kind);
            let top_left = building.position - size * 0.5;

            draw_rectangle(top_left.x, top_left.y, size.x, size.y, color);
            draw_rectangle_lines(top_left.x, top_left.y, size.x, size.y, 2.0, BLACK);

            if self.selected_building == Some(building.id) {
                draw_rectangle_lines(
                    top_left.x - 4.0,
                    top_left.y - 4.0,
                    size.x + 8.0,
                    size.y + 8.0,
                    3.0,
                    YELLOW,
                );
            }

            let label = building_label(building.kind);
            draw_ui_text(label, top_left.x, top_left.y - 8.0, 18.0, WHITE);
            draw_health_bar(
                top_left + vec2(0.0, size.y + 5.0),
                size.x,
                building.health,
                building.kind.max_health(),
            );
        }
    }

    pub(super) fn render_units(&self) {
        for unit in &self.state.units {
            let color = owner_color(unit.owner);
            let radius = unit_radius(unit.kind);
            draw_circle(unit.position.x, unit.position.y, radius, color);
            draw_circle_lines(unit.position.x, unit.position.y, radius, 2.0, BLACK);

            if self.selected_units.contains(&unit.id) {
                draw_circle_lines(unit.position.x, unit.position.y, radius + 5.0, 3.0, YELLOW);
            }

            let stats = self.state.unit_stats_for_player(unit.owner, unit.kind);
            draw_health_bar(
                unit.position + vec2(-18.0, radius + 6.0),
                36.0,
                unit.health,
                stats.max_health as f32,
            );

            if matches!(
                unit.command,
                UnitCommand::AttackBuilding(_) | UnitCommand::AttackUnit(_)
            ) {
                draw_circle_lines(unit.position.x, unit.position.y, radius + 9.0, 1.5, ORANGE);
            }
        }
    }

    pub(super) fn render_build_preview(&self, camera: &ToolkitCamera) {
        let Some(intent) = self.pending_build else {
            return;
        };

        let world_pos = mouse_world(camera);
        match intent {
            BuildIntent::Standard(kind) => {
                let size = building_size(kind);
                let top_left = world_pos - size * 0.5;
                draw_rectangle(
                    top_left.x,
                    top_left.y,
                    size.x,
                    size.y,
                    Color::from_rgba(240, 220, 90, 80),
                );
                draw_rectangle_lines(top_left.x, top_left.y, size.x, size.y, 2.0, YELLOW);
            }
            BuildIntent::LeyClaim => {
                draw_circle_lines(
                    world_pos.x,
                    world_pos.y,
                    AETHER_NODE_BUILD_RADIUS,
                    2.0,
                    YELLOW,
                );
            }
        }
    }

    pub(super) fn render_ui(&self) {
        let panel_height = 116.0;
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            panel_height,
            Color::from_rgba(20, 24, 28, 235),
        );

        let player = &self.state.players[self.local_player];
        let title = format!(
            "Fracture Command RTS - {} | Matter {}  Aether {}  Supply {}/{}  Flow {}",
            race_label(player.race),
            player.resources.matter,
            player.resources.aether,
            player.supply_used,
            player.supply_cap,
            player.ley_flow_capacity
        );
        draw_ui_text(&title, 18.0, 30.0, 24.0, WHITE);

        let selection = self.selection_label();
        draw_ui_text(
            &selection,
            18.0,
            58.0,
            20.0,
            Color::from_rgba(220, 226, 230, 255),
        );

        let controls = self.controls_text();
        draw_ui_text(
            &controls,
            18.0,
            88.0,
            18.0,
            Color::from_rgba(185, 194, 201, 255),
        );

        if self.message_timer > 0.0 {
            draw_ui_text(
                &self.message,
                18.0,
                panel_height + 28.0,
                22.0,
                Color::from_rgba(250, 230, 130, 255),
            );
        }

        if let Some(winner) = self.state.winner {
            let text = if winner == self.local_player {
                "Victory"
            } else {
                "Defeat"
            };
            let dims = measure_ui_text(text, None, 64, 1.0);
            draw_rectangle(
                screen_width() * 0.5 - 210.0,
                screen_height() * 0.5 - 70.0,
                420.0,
                130.0,
                Color::from_rgba(10, 12, 14, 230),
            );
            draw_ui_text(
                text,
                screen_width() * 0.5 - dims.width * 0.5,
                screen_height() * 0.5 + 18.0,
                64.0,
                WHITE,
            );
        }
    }

    pub(super) fn selection_label(&self) -> String {
        if let Some(intent) = self.pending_build {
            return match intent {
                BuildIntent::Standard(kind) => {
                    format!("Placing {:?}: left-click map to build", kind)
                }
                BuildIntent::LeyClaim => "Placing ley claim: left-click a ley node".to_string(),
            };
        }

        if !self.selected_units.is_empty() {
            return format!("Selected units: {}", self.selected_units.len());
        }

        if let Some(building_id) = self.selected_building {
            if let Some(building) = self
                .state
                .buildings
                .iter()
                .find(|building| building.id == building_id)
            {
                return format!("Selected building: {:?}", building.kind);
            }
        }

        "No selection".to_string()
    }

    pub(super) fn controls_text(&self) -> String {
        if self.pending_build.is_some() {
            return "Build placement: left-click to place | right-click cancel | WASD pan"
                .to_string();
        }

        if self.selected_worker().is_some() {
            let catalog = command_catalog_for(self.local_race());
            return format!(
                "Worker build: Z {:?}, X {:?}, C {:?}, V {:?}, B {:?}, N {:?} | then left-click placement",
                catalog.supply_building,
                catalog.production_building,
                catalog.ley_claim_building,
                catalog.aether_link_building,
                catalog.advanced_production_building,
                catalog.research_building
            );
        }

        if self.selected_building.is_some() {
            return "Building commands: Q worker | T basic | Y heavy | U specialist | R/F/G tech | F5 switch race"
                .to_string();
        }

        "Left select worker/building | right-click move/attack/gather | F5 switch race | WASD pan"
            .to_string()
    }
}
