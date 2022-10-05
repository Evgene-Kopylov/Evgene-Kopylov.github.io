use macroquad::prelude::*;


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const UNIT_COLOR: Color = DARKGRAY;
const UNIT_SIZE: Vec2 = Vec2::new(40., 40.);
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

    pub fn update(&mut self, dt: f32) -> &str {
        let mut keycode: &str = "";
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) {
            x_move -= 1f32;
            keycode = "Left"
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1f32;
            keycode = "Right"
        }

        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) {
            y_move -= 1f32;
            keycode = "Up"
        }
        if is_key_down(KeyCode::Down) {
            y_move += 1f32;
            keycode = "Down"
        }

        if self.rect.x < 1f32 {
            x_move = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            x_move = -1f32;
        }

        if self.rect.y < 1f32 {
            y_move = 1f32;
        }
        if self.rect.y > screen_height() - self.rect.h {
            y_move = -1f32;
        }

        self.rect.x += x_move * dt * UNIT_SPEED;
        self.rect.y += y_move * dt * UNIT_SPEED;
        keycode
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
        let keycode = unit.update(get_frame_time());
        if keycode != "" { info!("{}", keycode); }
        clear_background(GROUND_COLOR);
        unit.draw();
        next_frame().await
    }
}
