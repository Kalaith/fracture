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

pub fn render_markers(markers: &[crate::types::StrategicMarker], sectors: &[crate::types::Sector]) {
    use crate::types::MarkerType;
    
    for marker in markers {
        // Find the sector this marker is on
        if let Some(sector) = sectors.iter().find(|s| s.id == marker.sector_id) {
            let pulse = ((macroquad::time::get_time() * 2.0).sin() * 0.2 + 0.8) as f32;
            
            match marker.marker_type {
                MarkerType::Attack => {
                    // Red crosshair/target for attack
                    let size = 25.0 * pulse;
                    let color = Color::from_rgba(255, 50, 50, 220);
                    
                    // Draw X
                    draw_line(
                        sector.position.x - size,
                        sector.position.y - size,
                        sector.position.x + size,
                        sector.position.y + size,
                        3.0,
                        color,
                    );
                    draw_line(
                        sector.position.x + size,
                        sector.position.y - size,
                        sector.position.x - size,
                        sector.position.y + size,
                        3.0,
                        color,
                    );
                    
                    // Outer circle
                    draw_circle_lines(
                        sector.position.x,
                        sector.position.y,
                        size * 1.3,
                        3.0,
                        color,
                    );
                }
                MarkerType::Defend => {
                    // Blue shield for defend
                    let size = 20.0 * pulse;
                    let color = Color::from_rgba(100, 150, 255, 220);
                    
                    // Draw shield shape (hexagon)
                    let points = [
                        vec2(0.0, -size),
                        vec2(size * 0.866, -size * 0.5),
                        vec2(size * 0.866, size * 0.5),
                        vec2(0.0, size),
                        vec2(-size * 0.866, size * 0.5),
                        vec2(-size * 0.866, -size * 0.5),
                    ];
                    
                    for i in 0..points.len() {
                        let next = (i + 1) % points.len();
                        draw_line(
                            sector.position.x + points[i].x,
                            sector.position.y + points[i].y,
                            sector.position.x + points[next].x,
                            sector.position.y + points[next].y,
                            3.0,
                            color,
                        );
                    }
                }
            }
        }
    }
}
