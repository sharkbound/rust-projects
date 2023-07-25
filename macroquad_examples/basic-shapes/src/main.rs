use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut pos = Vec2::ZERO;
    loop {
        clear_background(BLACK);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        let speed = 100.0;
        if is_key_down(KeyCode::Down) {
            pos.y += speed * get_frame_time()
        }
        if is_key_down(KeyCode::Up) {
            pos.y -= speed * get_frame_time()
        }
        if is_key_down(KeyCode::Right) {
            pos.x += speed * get_frame_time()
        }
        if is_key_down(KeyCode::Left) {
            pos.x -= speed * get_frame_time()
        }


        draw_circle(pos.x, pos.y, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await
    }
}