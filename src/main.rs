use macroquad::prelude::*;

#[macroquad::main("rect_game")]
async fn main() {
    let ferris = load_texture("./lag.png").await.unwrap();
    let mut cam = Camera3D {
        position: vec3(screen_width() / 2., screen_height() / 2., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    loop {
        clear_background(GRAY);
        cam = process_input(cam);
        set_camera(&cam);
        draw_sphere(
            vec3(screen_width() / 2., screen_height() / 2., 0.),
            10.,
            ferris,
            GOLD,
        );
        next_frame().await
    }
}

fn process_input(mut cam: Camera3D) -> Camera3D {
    if is_key_down(KeyCode::Left) {
        cam.position.x -= 0.1;
    } else if is_key_down(KeyCode::Right) {
        cam.position.x += 0.1;
    } else if is_key_down(KeyCode::Up) {
        cam.position.y -= 0.1;
    } else if is_key_down(KeyCode::Down) {
        cam.position.y += 0.1;
    } else if is_key_down(KeyCode::PageUp) {
        cam.position.z -= 0.1;
    } else if is_key_down(KeyCode::PageDown) {
        cam.position.z += 0.1;
    }
    cam
}
