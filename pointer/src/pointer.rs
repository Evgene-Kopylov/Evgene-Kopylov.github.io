use macroquad::prelude::*;
use crate::projectile::Projectile;
use crate::settings::*;


pub struct Unit {
    pub texture: Texture2D,
    pub size: (f32, f32),
    pub scale: f32,
    pub radius: f32,
    pub rotation: f32,
    pub position: (f32, f32),
    pub speed: f32,
    pub projectile_texture: Texture2D,
    pub projectiles: Vec<Projectile>
}


impl Unit {
    pub fn new(
        texture: Texture2D,
        projectile_texture: Texture2D,
        position: (f32, f32)
    ) -> Self {
        Self {
            texture, projectile_texture,
            size: (texture.width(), texture.height()),
            scale: 1.,
            radius: f32::max(texture.width(), texture.height()),
            rotation: 0.,
            position,
            speed: 300.,
            projectiles: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, mouse_position: Vec2) {
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D){
            x_move += 1f32;
        }

        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
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

        // поворот в сторону курсора
        self.rotation = self.rotation % f32::to_radians(360.);
        let mut dx = self.position.0 - mouse_position.x;
        if dx == 0f32 { dx += 1f32; };

        let mut dy = self.position.1 - mouse_position.y;
        if dy == 0f32 { dy += 1f32; };

        if dx >= 0f32 {
            self.rotation = (dy / dx).atan() - f32::to_radians(90.);
        } else {
            self.rotation = (dy / dx).atan() - f32::to_radians(270.);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let size = (
                self.projectile_texture.width(), self.projectile_texture.height());
            let speed = self.speed * 2.3;
            let position = (  // точка появления выстрела
                              self.position.0 + 65. * (self.rotation - f32::to_radians(90.)).cos(),
                              self.position.1 + 65. * (self.rotation - f32::to_radians(90.)).sin()
            );

            let mut projectile = Projectile::new(
                self.rotation, position, size, speed);
            self.projectiles.push(projectile);
        }

        for i in 0..self.projectiles.len() {
            self.projectiles[i].position.0 +=
                dt * self.projectiles[i].speed *
                    (self.projectiles[i].rotation - f32::to_radians(90.)).cos();
            self.projectiles[i].position.1 +=
                dt * self.projectiles[i].speed *
                    (self.projectiles[i].rotation - f32::to_radians(90.)).sin();
        }


    }

    pub fn draw(&self) {
        // Выстрелы
        for projectile in &self.projectiles {
            draw_texture_ex(
                self.projectile_texture,
                projectile.position.0 - projectile.size.0 * 0.5,
                projectile.position.1 - projectile.size.1 * 0.5,
                PROJECTILE_COLOR,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(projectile.size .0, projectile.size.1)),
                    rotation: projectile.rotation,
                    ..Default::default()
                }
            );
        }

        // Юнит
        // тень
        draw_texture_ex(
            self.texture,
            self.position.0 - self.size.0 * 0.5 + 3.,
            self.position.1 - self.size.1 * 0.5 + 4.,
            DARKGRAY,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size .0, self.size.1)),
                rotation: self.rotation,
                ..Default::default()
            }
        );
        // сам юнит
        draw_texture_ex(
            self.texture,
            self.position.0 - self.size.0 * 0.5,
            self.position.1 - self.size.1 * 0.5,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size .0, self.size.1)),
                rotation: self.rotation,
                ..Default::default()
            }
        );

    }
}
