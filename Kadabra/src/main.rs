use macroquad::prelude::*;


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);

const UNIT_SIZE: Vec2 = const_vec2!([40., 60.]);


#[macroquad::main("breakout")]
async fn main() {
    let unit_rect = Rect::new(
        screen_width() * 0.5 - UNIT_SIZE.x * 0.5,
        screen_height() * 0.5 - UNIT_SIZE.y * 0.5,
        UNIT_SIZE.x,
        UNIT_SIZE.y,
    );

    loop {
        clear_background(GROUND_COLOR);
        draw_rectangle(unit_rect.x, unit_rect.y, unit_rect.w, unit_rect.h, DARKGRAY);
        next_frame().await
    }
}
