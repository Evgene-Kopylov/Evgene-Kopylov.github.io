use macroquad::prelude::*;
use crate::settings::*;

pub struct Projectile {
    pub texture: Texture2D,
    pub rotation: f32,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub speed: f32,
}


impl Projectile {
    pub fn new(
        texture: Texture2D,
        rotation: f32,
        position: (f32, f32),
        size: (f32, f32),
        speed: f32
    ) -> Projectile {
        Projectile {
            texture,
            rotation,
            position,
            size,
            speed,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.0 - self.size.0 * 0.5,
            self.position.1 - self.size.1 * 0.5,
            PROJECTILE_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size .0, self.size.1)),
                rotation: self.rotation,
                ..Default::default()
            }
        );
    }
}