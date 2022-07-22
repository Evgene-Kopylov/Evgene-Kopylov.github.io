use macroquad::prelude::*;


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const UNIT_COLOR: Color = DARKGRAY;
const UNIT_SIZE: Vec2 = const_vec2!([40., 40.]);
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
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) {
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1f32;
        }

        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) {
            y_move += 1f32;
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
    let texture: Texture2D = load_texture("materials/path726-5.png").await.unwrap();

    loop {
        unit.update(get_frame_time());
        clear_background(GROUND_COLOR);
        unit.draw();

        draw_texture_ex(
            texture,
            // screen_width() / 2. - texture.width() / 2.,
            // screen_height() / 2. - texture.height() / 2.,
            0.,
            1.,
            GOLD,
            DrawTextureParams {
                dest_size: Some(Vec2::new(100.0, 100.0)),
                rotation: -50.,
                ..Default::default()
            }
        );

        next_frame().await
    }
}
