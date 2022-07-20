use macroquad::prelude::*;


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const UNIT_COLOR: Color = DARKGRAY;
const UNIT_SIZE: Vec2 = const_vec2!([40., 60.]);


struct Unit {
    rect: Rect
}

impl Unit {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5 - UNIT_SIZE.x * 0.5,
                screen_height() * 0.5 - UNIT_SIZE.y * 0.5,
                UNIT_SIZE.x,
                UNIT_SIZE.y,
            )
        }
    }
}


#[macroquad::main("breakout")]
async fn main() {
    let unit_rect = Unit::new();

    loop {
        clear_background(GROUND_COLOR);
        draw_rectangle(
            unit_rect.rect.x,
            unit_rect.rect.y,
            unit_rect.rect.w,
            unit_rect.rect.h,
            UNIT_COLOR);
        next_frame().await
    }
}
