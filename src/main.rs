use macroquad::prelude::*;

#[macroquad::main("rect_game")]
async fn main() {
    loop {
        clear_background(DARKGREEN);
        draw_circle(
            screen_width() / 2_f32,
            screen_height() / 2_f32,
            50_f32,
            GOLD,
        );
        next_frame().await
    }
}
