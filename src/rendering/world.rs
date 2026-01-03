//! World rendering - units, sectors, battlefield

use crate::types::*;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub fn render_sector(sector: &Sector) {
    use crate::types::FactionId;

    // Base color based on ownership
    let base_color = match sector.control {
        Some(FactionId::Faction1) => Color::from_rgba(100, 150, 255, 100),  // Blue
        Some(FactionId::Faction2) => Color::from_rgba(255, 100, 100, 100),  // Red
        Some(FactionId::Faction3) => Color::from_rgba(100, 255, 150, 100),  // Green
        None => Color::from_rgba(128, 128, 128, 50),
    };

    draw_circle(sector.position.x, sector.position.y, sector.radius, base_color);

    // Draw capture progress overlay (gradual color transition)
    let progress_alpha = (sector.control_progress.abs() * 150.0) as u8;
    let progress_color = if sector.control_progress < -0.3 {
        Color::from_rgba(100, 150, 255, progress_alpha) // Blue (Faction1)
    } else if sector.control_progress > 0.3 {
        Color::from_rgba(100, 255, 150, progress_alpha) // Green (Faction3)
    } else {
        Color::from_rgba(255, 100, 100, progress_alpha) // Red (Faction2)
    };

    // Fill shows capture progress
    draw_circle(
        sector.position.x,
        sector.position.y,
        sector.radius * sector.control_progress.abs().max(0.2),
        progress_color,
    );

    // Outer ring shows ownership
    let ring_color = match sector.control {
        Some(FactionId::Faction1) => Color::from_rgba(100, 150, 255, 255),
        Some(FactionId::Faction2) => Color::from_rgba(255, 100, 100, 255),
        Some(FactionId::Faction3) => Color::from_rgba(100, 255, 150, 255),
        None => dark::TEXT_DIM,
    };

    draw_circle_lines(
        sector.position.x,
        sector.position.y,
        sector.radius,
        3.0,
        ring_color,
    );

    // Draw contested indicator (pulsing ring if control_progress is changing)
    if sector.control_progress.abs() > 0.05 && sector.control_progress.abs() < 0.95 {
        let pulse = ((macroquad::time::get_time() * 3.0).sin() * 0.3 + 0.7) as f32;
        draw_circle_lines(
            sector.position.x,
            sector.position.y,
            sector.radius * 1.1,
            2.0,
            Color::from_rgba(255, 255, 100, (pulse * 150.0) as u8),
        );
    }
}

pub fn render_squad(squad: &Squad) {
    use crate::types::FactionId;

    let squad_color = match squad.owner {
        FactionId::Faction1 => Color::from_rgba(100, 150, 255, 255),  // Blue
        FactionId::Faction2 => Color::from_rgba(255, 100, 100, 255),  // Red
        FactionId::Faction3 => Color::from_rgba(100, 255, 150, 255),  // Green
    };

    for unit in &squad.units {
        render_unit(unit, squad_color);
    }

    // Draw rally point
    draw_circle(
        squad.rally_point.x,
        squad.rally_point.y,
        5.0,
        Color::from_rgba(
            squad_color.r as u8,
            squad_color.g as u8,
            squad_color.b as u8,
            100,
        ),
    );
}

pub fn render_unit(unit: &Unit, color: Color) {
    let size = match unit.unit_type {
        UnitType::Infantry => 8.0,
        UnitType::Heavy => 12.0,
        UnitType::Artillery => 10.0,
        UnitType::Drone => 6.0,
    };

    // Draw unit
    draw_circle(unit.position.x, unit.position.y, size, color);

    // Draw health bar
    let health_percent = unit.health_percent();
    let bar_width = size * 2.0;
    let bar_height = 3.0;
    let bar_y = unit.position.y - size - 5.0;

    draw_rectangle(
        unit.position.x - bar_width / 2.0,
        bar_y,
        bar_width,
        bar_height,
        dark::PANEL,
    );

    let health_color = if health_percent > 0.5 {
        dark::POSITIVE
    } else if health_percent > 0.25 {
        dark::WARNING
    } else {
        dark::NEGATIVE
    };

    draw_rectangle(
        unit.position.x - bar_width / 2.0,
        bar_y,
        bar_width * health_percent,
        bar_height,
        health_color,
    );

    // Draw target line (movement)
    if let Some(target_pos) = unit.target_position {
        draw_line(
            unit.position.x,
            unit.position.y,
            target_pos.x,
            target_pos.y,
            1.0,
            Color::from_rgba(color.r as u8, color.g as u8, color.b as u8, 100),
        );
    }

    // Draw attack line (combat target)
    if unit.target_enemy.is_some() {
        draw_circle_lines(
            unit.position.x,
            unit.position.y,
            size * 1.5,
            2.0,
            Color::from_rgba(255, 100, 100, 200), // Red ring when attacking
        );
    }
}

pub fn render_combat_effects(effects: &[crate::types::CombatEffect]) {
    use crate::types::EffectType;

    for effect in effects {
        let alpha = (effect.time_remaining * 255.0) as u8;

        match effect.effect_type {
            EffectType::Hit => {
                // Draw damage number
                let damage_text = format!("{:.0}", effect.damage);
                let font_size = 16.0;
                let text_size = measure_text(&damage_text, None, font_size as u16, 1.0);
                let y_offset = (1.0 - effect.time_remaining) * 20.0;

                draw_text(
                    &damage_text,
                    effect.position.x - text_size.width / 2.0,
                    effect.position.y - y_offset,
                    font_size,
                    Color::from_rgba(255, 100, 100, alpha),
                );

                // Draw hit flash
                draw_circle(
                    effect.position.x,
                    effect.position.y,
                    8.0 * effect.time_remaining,
                    Color::from_rgba(255, 200, 100, (alpha as f32 * 0.5) as u8),
                );
            }
            EffectType::Kill => {
                // Draw explosion effect
                let size = (1.0 - effect.time_remaining) * 30.0;
                draw_circle(
                    effect.position.x,
                    effect.position.y,
                    size,
                    Color::from_rgba(255, 100, 50, (alpha as f32 * 0.3) as u8),
                );
                draw_circle_lines(
                    effect.position.x,
                    effect.position.y,
                    size,
                    2.0,
                    Color::from_rgba(255, 150, 50, alpha),
                );
            }
        }
    }
}
