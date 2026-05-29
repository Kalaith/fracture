use fracture_command::rts::RtsApp;
use macroquad::prelude::*;
use macroquad_toolkit::camera::Camera2D;

fn window_conf() -> Conf {
    Conf {
        window_title: "Fracture Command - RTS Prototype".to_owned(),
        window_width: 1600,
        window_height: 900,
        window_resizable: true,
        sample_count: 0,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = match RtsApp::new_crash_basin() {
        Ok(app) => app,
        Err(err) => {
            eprintln!("Failed to start RTS prototype: {}", err);
            return;
        }
    };
    let mut camera = Camera2D::new(app.camera_center(), 1.0);

    loop {
        let dt = get_frame_time();

        app.update(&mut camera, dt);
        app.render(&camera);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
