use macroquad::prelude::*;


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const UNIT_COLOR: Color = DARKGRAY;
const UNIT_SIZE: (f32, f32) = (50.0, 75.0);
const UNIT_SPEED: f32 = 300.0;

struct Unit {
    collision: Circle,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            collision: Circle::new(
                screen_width() * 0.5 - UNIT_SIZE.1 * 0.5,
                screen_height() * 0.5 - UNIT_SIZE.1 * 0.5,
                UNIT_SIZE.1 / 2.
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

        if self.collision.x < 1f32 {
            x_move = 1f32;
        }
        if self.collision.x > screen_width() - UNIT_SIZE.0 {
            x_move = -1f32;
        }

        if self.collision.y < 1f32 {
            y_move = 1f32;
        }
        if self.collision.y > screen_height() - UNIT_SIZE.1 {
            y_move = -1f32;
        }

        self.collision.x += x_move * dt * UNIT_SPEED;
        self.collision.y += y_move * dt * UNIT_SPEED;
    }

    pub fn draw_collision(&self) {
        draw_circle_lines(
            self.collision.x + UNIT_SIZE.0 * 0.5,
            self.collision.y + UNIT_SIZE.1 * 0.5,
            self.collision.r,
            1.,
            RED
        )
    }

    pub fn draw(&self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            self.collision.x,
            self.collision.y,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(UNIT_SIZE.0, UNIT_SIZE.1)),
                rotation: 0.,
                ..Default::default()
            }
        );
    }
}


#[macroquad::main("breakout")]
async fn main() {
    let texture: Texture2D = load_texture("materials/path726-5.png").await.unwrap();
    let mut unit = Unit::new();

    loop {
        unit.update(get_frame_time());
        clear_background(GROUND_COLOR);
        unit.draw_collision();
        unit.draw(texture);

        next_frame().await
    }
}
