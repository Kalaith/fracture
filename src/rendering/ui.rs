//! UI rendering - panels, HUD, menus

use crate::config::Config;
use crate::game::GameState;
use crate::types::FactionId;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub fn render_minimap(game: &GameState) {
    let minimap_size = 200.0;
    let minimap_x = screen_width() - minimap_size - 10.0;
    let minimap_y = 10.0;
    let scale_x = minimap_size / Config::WORLD_WIDTH;
    let scale_y = minimap_size / Config::WORLD_HEIGHT;

    // Background
    draw_rectangle(minimap_x, minimap_y, minimap_size, minimap_size, dark::PANEL);
    draw_rectangle_lines(minimap_x, minimap_y, minimap_size, minimap_size, 2.0, dark::TEXT);

    // Draw sectors
    for sector in &game.sectors {
        let x = minimap_x + sector.position.x * scale_x;
        let y = minimap_y + sector.position.y * scale_y;
        let radius = sector.radius * scale_x.min(scale_y);

        let color = match sector.control {
            Some(FactionId::Faction1) => Color::from_rgba(100, 150, 255, 150),
            Some(FactionId::Faction2) => Color::from_rgba(255, 100, 100, 150),
            Some(FactionId::Faction3) => Color::from_rgba(100, 255, 150, 150),
            None => Color::from_rgba(128, 128, 128, 100),
        };

        draw_circle(x, y, radius, color);
    }

    // Draw units
    for squad in &game.squads {
        let color = match squad.owner {
            FactionId::Faction1 => Color::from_rgba(100, 150, 255, 255),
            FactionId::Faction2 => Color::from_rgba(255, 100, 100, 255),
            FactionId::Faction3 => Color::from_rgba(100, 255, 150, 255),
        };

        for unit in &squad.units {
            let x = minimap_x + unit.position.x * scale_x;
            let y = minimap_y + unit.position.y * scale_y;
            draw_circle(x, y, 2.0, color);
        }
    }

    // Label
    draw_text(
        "MINIMAP",
        minimap_x + 5.0,
        minimap_y + minimap_size + 15.0,
        14.0,
        dark::TEXT_DIM,
    );
}

pub fn render_ui(game: &GameState) {
    let margin = Config::UI_MARGIN;
    let panel_width = Config::UI_PANEL_WIDTH;

    // Commander panel
    render_commander_panel(game, margin, margin, panel_width);

    // Build panel
    let commander_height = 290.0;
    render_build_panel(game, margin, margin + commander_height + 10.0, panel_width);

    // Minimap
    render_minimap(game);

    // Game status (pause/speed)
    render_game_status(game);

    // Victory progress panel
    if game.victory_timer > 0.0 {
        render_victory_panel(game, screen_width() / 2.0 - 150.0, margin);
    }

    // Game over screen
    if game.game_over {
        render_game_over(game);
    }
}

fn render_build_panel(game: &GameState, x: f32, y: f32, width: f32) {
    use crate::types::UnitType;

    let height = 450.0; // Increased height for more units
    macroquad_toolkit::ui::panel(x, y, width, height, Some("Build Units"));

    let mut y_offset = y + 35.0;
    let text_x = x + Config::UI_MARGIN;
    let can_spawn = game.can_spawn();

    // Unit types organized by category with hotkeys
    let units = [
        // Infantry
        (UnitType::InfantryLight, "1", "Light Infantry"),
        (UnitType::InfantryHeavy, "2", "Heavy Infantry"),
        // Armor
        (UnitType::ArmorLight, "3", "Light Tank"),
        (UnitType::ArmorHeavy, "4", "Heavy Tank"),
        // Artillery
        (UnitType::Artillery, "5", "Artillery"),
        // Drones
        (UnitType::DroneScout, "6", "Scout Drone"),
        (UnitType::DroneGunship, "7", "Gunship"),
        // Support
        (UnitType::Engineer, "8", "Engineer"),
        // Specialist
        (UnitType::AntiArmor, "9", "Anti-Armor"),
        (UnitType::AntiAir, "0", "Anti-Air"),
    ];

    for (unit_type, key, name) in units {
        let cost_per_unit = unit_type.supply_cost();
        let squad_size = 3;
        let total_cost = cost_per_unit * squad_size;
        let damage = unit_type.attack_damage();
        let health = unit_type.max_health();
        let armor = unit_type.armor();

        // Unit name and key
        let color = if can_spawn {
            dark::TEXT
        } else {
            dark::TEXT_DIM
        };

        draw_text(
            &format!("[{}] {} (x3)", key, name),
            text_x,
            y_offset,
            16.0,
            color,
        );
        y_offset += 18.0;

        // Stats in smaller text - show cost, HP, armor, damage
        let stats_text = if armor > 0.0 {
            format!(
                "  Cost:{} HP:{:.0} Armor:{:.0} DMG:{:.0}",
                total_cost, health, armor, damage
            )
        } else {
            format!(
                "  Cost:{} HP:{:.0} DMG:{:.0}",
                total_cost, health, damage
            )
        };
        draw_text(
            &stats_text,
            text_x,
            y_offset,
            12.0,
            dark::TEXT_DIM,
        );
        y_offset += 20.0;
    }

    // Instructions
    y_offset += 5.0;
    draw_text(
        "Press 1-0 to spawn units",
        text_x,
        y_offset,
        14.0,
        if can_spawn {
            dark::POSITIVE
        } else {
            dark::TEXT_DIM
        },
    );
}

fn render_commander_panel(game: &GameState, x: f32, y: f32, width: f32) {
    let commander = game.get_local_commander();

    macroquad_toolkit::ui::panel(x, y, width, 290.0, Some("Commander"));

    let mut y_offset = y + 40.0;
    let text_x = x + Config::UI_MARGIN;

    // Supply info
    draw_text(
        &format!(
            "Supply: {}/{}",
            commander.supply_used, commander.supply_max
        ),
        text_x,
        y_offset,
        Config::UI_FONT_SIZE,
        dark::TEXT,
    );
    y_offset += 25.0;

    // Supply bar
    macroquad_toolkit::ui::progress_bar(
        text_x,
        y_offset,
        width - Config::UI_MARGIN * 2.0,
        20.0,
        commander.supply_used as f32,
        commander.supply_max as f32,
        dark::ACCENT,
    );
    y_offset += 30.0;

    // Spawn cooldown
    let spawn_ready = game.can_spawn();
    let local_commander = &game.commanders[game.local_faction.index()];
    let cooldown_text = if spawn_ready {
        "Spawn Ready".to_string()
    } else {
        format!(
            "Spawn in {:.1}s",
            Config::SPAWN_COOLDOWN - local_commander.spawn_timer
        )
    };
    draw_text(
        &cooldown_text,
        text_x,
        y_offset,
        Config::UI_FONT_SIZE,
        if spawn_ready {
            dark::POSITIVE
        } else {
            dark::TEXT_DIM
        },
    );
    y_offset += 25.0;

    // Spawn cooldown bar
    macroquad_toolkit::ui::progress_bar(
        text_x,
        y_offset,
        width - Config::UI_MARGIN * 2.0,
        20.0,
        local_commander.spawn_timer,
        Config::SPAWN_COOLDOWN,
        if spawn_ready {
            dark::POSITIVE
        } else {
            dark::WARNING
        },
    );
    y_offset += 30.0;

    // Active doctrines
    draw_text(
        "Doctrines:",
        text_x,
        y_offset,
        Config::UI_FONT_SIZE,
        dark::TEXT,
    );
    y_offset += 25.0;

    for (i, doctrine) in commander.active_doctrines.iter().enumerate() {
        let text = match doctrine {
            Some(d) => format!("{}: {:?}", i + 1, d),
            None => format!("{}: None", i + 1),
        };
        draw_text(
            &text,
            text_x,
            y_offset,
            Config::UI_FONT_SIZE - 2.0,
            dark::TEXT_DIM,
        );
        y_offset += 20.0;
    }
}

fn render_victory_panel(game: &GameState, x: f32, y: f32) {
    macroquad_toolkit::ui::panel(x, y, 300.0, 100.0, Some("Victory Progress"));

    let text_x = x + Config::UI_MARGIN;
    let mut y_offset = y + 40.0;

    draw_text(
        &format!(
            "Holding sectors: {:.1}s / {:.0}s",
            game.victory_timer,
            Config::VICTORY_TIME_REQUIRED
        ),
        text_x,
        y_offset,
        Config::UI_FONT_SIZE,
        dark::TEXT,
    );
    y_offset += 25.0;

    macroquad_toolkit::ui::progress_bar(
        text_x,
        y_offset,
        280.0,
        20.0,
        game.victory_timer,
        Config::VICTORY_TIME_REQUIRED,
        dark::POSITIVE,
    );
}

fn render_game_over(game: &GameState) {
    use crate::types::FactionId;

    let panel_width = 400.0;
    let panel_height = 200.0;
    let x = screen_width() / 2.0 - panel_width / 2.0;
    let y = screen_height() / 2.0 - panel_height / 2.0;

    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 180),
    );

    macroquad_toolkit::ui::panel(x, y, panel_width, panel_height, Some("Game Over"));

    let text_x = x + panel_width / 2.0;
    let mut y_offset = y + 80.0;

    let winner_text = match game.winner {
        Some(faction_id) => {
            if faction_id == game.local_faction {
                "You Win!"
            } else {
                match faction_id {
                    FactionId::Faction1 => "Blue Faction Wins!",
                    FactionId::Faction2 => "Red Faction Wins!",
                    FactionId::Faction3 => "Green Faction Wins!",
                }
            }
        }
        None => "Draw",
    };

    let text_color = if game.winner == Some(game.local_faction) {
        dark::POSITIVE
    } else {
        dark::NEGATIVE
    };

    // Center the text
    let text_size = 40.0;
    let text_dims = measure_text(winner_text, None, text_size as u16, 1.0);
    draw_text(
        winner_text,
        text_x - text_dims.width / 2.0,
        y_offset,
        text_size,
        text_color,
    );
    y_offset += 60.0;

    let subtitle = "Press ESC to exit";
    let subtitle_dims = measure_text(subtitle, None, Config::UI_FONT_SIZE as u16, 1.0);
    draw_text(
        subtitle,
        text_x - subtitle_dims.width / 2.0,
        y_offset,
        Config::UI_FONT_SIZE,
        dark::TEXT_DIM,
    );
}

pub fn render_controls_help() {
    let x = screen_width() - 220.0;
    let y = Config::UI_MARGIN;

    macroquad_toolkit::ui::panel(x, y, 210.0, 240.0, Some("Controls"));

    let mut y_offset = y + 40.0;
    let text_x = x + Config::UI_MARGIN;
    let font_size = 16.0;

    let controls = [
        "WASD - Pan camera",
        "Mouse wheel - Zoom",
        "1-4 - Spawn squads",
        "Q/E - Change orders",
        "F1/F2 - Set doctrines",
        "SPACE - Pause/Resume",
        "- / = - Slow/Fast",
        "0 - Normal speed",
        "ESC - Exit",
    ];

    for control in &controls {
        draw_text(control, text_x, y_offset, font_size, dark::TEXT_DIM);
        y_offset += 22.0;
    }
}

pub fn render_game_status(game: &GameState) {
    let x = screen_width() / 2.0 - 60.0;
    let y = 10.0;

    // Pause indicator
    if game.paused {
        draw_text(
            "PAUSED",
            x,
            y,
            32.0,
            Color::from_rgba(255, 200, 100, 200),
        );
    }

    // Speed indicator
    if game.game_speed != 1.0 {
        let speed_text = format!("Speed: {:.1}x", game.game_speed);
        draw_text(
            &speed_text,
            x,
            y + 35.0,
            20.0,
            dark::TEXT,
        );
    }
}
