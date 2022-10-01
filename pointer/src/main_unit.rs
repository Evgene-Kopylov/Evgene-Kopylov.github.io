use macroquad::audio;
use macroquad::audio::{PlaySoundParams, Sound};
use macroquad::prelude::*;
use crate::projectile::*;
use crate::settings::*;


pub struct MainUnit {
    pub texture: Texture2D,
    pub shoot_sound: Sound,
    pub size: (f32, f32),
    pub scale: f32,
    pub radius: f32,
    pub rotation: f32,
    pub position: (f32, f32),
    pub speed: f32,
    pub projectile_texture: Texture2D,
    pub projectiles: Vec<Projectile>,
    shoot_timer: f32,
    shoot_delay: f32,
}


impl MainUnit {
    pub fn new(
        texture: Texture2D,
        projectile_texture: Texture2D,
        shoot_sound: Sound,
        position: (f32, f32)
    ) -> Self {
        Self {
            texture, projectile_texture, shoot_sound,
            size: (texture.width(), texture.height()),
            scale: 1.,
            radius: f32::max(texture.width(), texture.height()),
            rotation: 0.,
            position,
            speed: 300.,
            projectiles: Vec::new(),
            shoot_timer: 0.,
            shoot_delay: MAIN_UNIT_SHOOT_DELAY,
        }
    }

    pub fn update(&mut self, dt: f32, mouse_position: Vec2) {
        self.shoot_timer += dt;
        // print!("self.shoot_timer {}\ndt {}\n", self.shoot_timer, dt);
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

        if is_mouse_button_down(MouseButton::Left) && self.shoot_timer >= self.shoot_delay {
            let size = (
                self.projectile_texture.width(), self.projectile_texture.height());
            let speed = self.speed * 3.;
            let position = (  // точка появления выстрела
                self.position.0 + 65. * (self.rotation - f32::to_radians(90.)).cos(),
                self.position.1 + 65. * (self.rotation - f32::to_radians(90.)).sin()
            );

            let projectile = Projectile::new(
                self.projectile_texture,
                self.rotation,
                position,
                size,
                speed
            );
            self.projectiles.push(projectile);
            self.shoot_timer = 0.;
            let mut sound_params: PlaySoundParams = PlaySoundParams::default();
            sound_params.looped = false;
            sound_params.volume = 0.16;
            audio::play_sound(self.shoot_sound, sound_params);
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
            projectile.draw();
        }
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