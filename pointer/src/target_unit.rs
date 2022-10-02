use macroquad::color::{DARKGRAY, WHITE};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D, Vec2};


pub struct TargetUnit {
    texture: Texture2D,
    spawn_position: (f32, f32),
    pub position: (f32, f32),
}

impl TargetUnit {
    pub fn new(texture: Texture2D, spawn_position: (f32, f32)) -> Self {
        Self {
            texture,
            spawn_position,
            position: spawn_position,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.0 - self.texture.width() * 0.5,
            self.position.1 - self.texture.height() * 0.5,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            }
        );
    }

}