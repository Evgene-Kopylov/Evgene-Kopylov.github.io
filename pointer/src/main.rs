use macroquad::prelude::*;


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);

struct Unit {
    texture: Texture2D,
    size: (f32, f32),
    scale: f32,
    radius: f32,
    rotation: f32,
    position: (f32, f32),
    speed: f32,
}

impl Unit {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            size: (texture.width(), texture.height()),
            scale: 1.,
            radius: f32::max(texture.width(), texture.height()),
            rotation: 0.,
            position: (screen_width() * 0.5, screen_height() * 0.5),
            speed: 300.,
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

        if self.position.0 < 1f32 {
            x_move = 1f32;
        }
        if self.position.0 > screen_width() {
            x_move = -1f32;
        }

        if self.position.1 < 1f32 {
            y_move = 1f32;
        }
        if self.position.1 > screen_height() {
            y_move = -1f32;
        }

        self.position.0 += x_move * dt * self.speed;
        self.position.1 += y_move * dt * self.speed;

    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.0 - self.size.0 * 0.5,
            self.position.1 - self.size.1 * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size .0, self.size.1)),
                rotation: self.rotation,
                ..Default::default()
            }
        );
    }
}


#[macroquad::main("breakout")]
async fn main() {
    let texture: Texture2D = load_texture("../materials/pointer_0.png").await.unwrap();
    let mut unit = Unit::new(texture);

    loop {
        unit.update(get_frame_time());
        clear_background(GROUND_COLOR);
        unit.draw();
        next_frame().await
    }
}
