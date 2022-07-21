use macroquad::prelude::*;


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const UNIT_COLOR: Color = DARKGRAY;
const UNIT_SIZE: Vec2 = const_vec2!([40., 60.]);
const UNIT_SPEED: f32 = 300.0;

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

    pub fn update(&mut self, dt: f32) {
        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) {
            y_move += 1f32;
        }
        self.rect.y += y_move * dt * UNIT_SPEED;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            UNIT_COLOR
        );
    }
}


#[macroquad::main("breakout")]
async fn main() {
    let mut unit = Unit::new();

    loop {
        unit.update(get_frame_time());
        clear_background(GROUND_COLOR);
        unit.draw();
        next_frame().await
    }
}
