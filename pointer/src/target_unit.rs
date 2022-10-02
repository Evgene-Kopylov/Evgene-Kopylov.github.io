use macroquad::color::{WHITE};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};


pub struct TargetUnit {
    pub texture: Texture2D,
    pub position: (f32, f32),
    pub shift: (f32, f32),
}

impl TargetUnit {
    pub fn new(texture: Texture2D, spawn_position: (f32, f32)) -> Self {
        Self {
            texture,
            position: spawn_position,
            shift: (0., 0.),
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.0 - self.texture.width() * 0.5 + self.shift.0,
            self.position.1 - self.texture.height() * 0.5 - self.shift.1,
            WHITE,
            DrawTextureParams {
                rotation: f32::to_radians(-5.),
                ..Default::default()
            }
        );
    }

}