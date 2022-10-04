use macroquad::color::{WHITE};
use macroquad::prelude::{DARKGRAY, draw_texture_ex, DrawTextureParams, Texture2D};


pub struct TargetUnit {
    pub texture: Texture2D,
    pub shadow_texture: Texture2D,
    pub position: (f32, f32),
    pub shift: (f32, f32),
}

impl TargetUnit {
    pub fn new(texture: Texture2D, shadow_texture: Texture2D, spawn_position: (f32, f32)) -> Self {
        Self {
            texture,
            shadow_texture,
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
                ..Default::default()
            }
        );
    }

    pub fn draw_shadow(&self) {
        // тень
        let height = 3.;
        draw_texture_ex(
            self.shadow_texture,
            self.position.0 - self.texture.width() * 0.5 + 3. * height,
            self.position.1 - self.texture.height() * 0.5 + 4. * height,
            DARKGRAY,
            DrawTextureParams {
                ..Default::default()
            }
        );
    }

}